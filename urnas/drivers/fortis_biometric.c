/*
 * FORTIS Biometric Reader Driver
 * Driver para leitor biométrico FORTIS-BR-001
 * 
 * Copyright (C) 2024 FORTIS Team
 * License: MIT
 */

#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/usb.h>
#include <linux/hid.h>
#include <linux/input.h>
#include <linux/cdev.h>
#include <linux/device.h>
#include <linux/mutex.h>
#include <linux/completion.h>
#include <linux/workqueue.h>
#include <linux/timer.h>
#include <linux/crypto.h>
#include <linux/scatterlist.h>
#include <linux/random.h>

#define FORTIS_BIOMETRIC_VENDOR_ID    0x1234
#define FORTIS_BIOMETRIC_PRODUCT_ID   0x5678
#define FORTIS_BIOMETRIC_MAX_DEVICES  16
#define FORTIS_BIOMETRIC_BUFFER_SIZE  4096
#define FORTIS_BIOMETRIC_TIMEOUT      5000  /* 5 seconds */

/* Device structure */
struct fortis_biometric_device {
    struct usb_device *udev;
    struct usb_interface *interface;
    struct cdev cdev;
    struct device *device;
    struct class *class;
    dev_t devt;
    
    /* Data buffers */
    unsigned char *bulk_in_buffer;
    size_t bulk_in_size;
    unsigned char *bulk_out_buffer;
    size_t bulk_out_size;
    
    /* Synchronization */
    struct mutex mutex;
    struct completion data_available;
    struct workqueue_struct *workqueue;
    struct work_struct work;
    
    /* Device state */
    int open_count;
    bool device_present;
    bool data_ready;
    
    /* Biometric data */
    unsigned char fingerprint_data[FORTIS_BIOMETRIC_BUFFER_SIZE];
    size_t fingerprint_size;
    unsigned char facial_data[FORTIS_BIOMETRIC_BUFFER_SIZE];
    size_t facial_size;
    
    /* Security */
    struct crypto_cipher *cipher;
    unsigned char encryption_key[32];
    bool encryption_enabled;
    
    /* Statistics */
    unsigned long capture_count;
    unsigned long error_count;
    unsigned long last_capture_time;
};

/* Global variables */
static struct class *fortis_biometric_class;
static struct usb_driver fortis_biometric_driver;
static DEFINE_MUTEX(fortis_biometric_mutex);
static int fortis_biometric_major;
static struct fortis_biometric_device *fortis_biometric_devices[FORTIS_BIOMETRIC_MAX_DEVICES];

/* Function prototypes */
static int fortis_biometric_probe(struct usb_interface *interface,
                                 const struct usb_device_id *id);
static void fortis_biometric_disconnect(struct usb_interface *interface);
static int fortis_biometric_open(struct inode *inode, struct file *file);
static int fortis_biometric_release(struct inode *inode, struct file *file);
static ssize_t fortis_biometric_read(struct file *file, char __user *buffer,
                                    size_t count, loff_t *ppos);
static ssize_t fortis_biometric_write(struct file *file, const char __user *buffer,
                                     size_t count, loff_t *ppos);
static long fortis_biometric_ioctl(struct file *file, unsigned int cmd,
                                  unsigned long arg);
static int fortis_biometric_capture_fingerprint(struct fortis_biometric_device *dev);
static int fortis_biometric_capture_facial(struct fortis_biometric_device *dev);
static int fortis_biometric_encrypt_data(struct fortis_biometric_device *dev,
                                        const unsigned char *data, size_t size,
                                        unsigned char *encrypted, size_t *encrypted_size);
static int fortis_biometric_decrypt_data(struct fortis_biometric_device *dev,
                                        const unsigned char *encrypted, size_t size,
                                        unsigned char *data, size_t *data_size);
static void fortis_biometric_work_handler(struct work_struct *work);
static void fortis_biometric_bulk_callback(struct urb *urb);

/* File operations */
static const struct file_operations fortis_biometric_fops = {
    .owner = THIS_MODULE,
    .open = fortis_biometric_open,
    .release = fortis_biometric_release,
    .read = fortis_biometric_read,
    .write = fortis_biometric_write,
    .unlocked_ioctl = fortis_biometric_ioctl,
    .llseek = no_llseek,
};

/* USB device table */
static const struct usb_device_id fortis_biometric_table[] = {
    { USB_DEVICE(FORTIS_BIOMETRIC_VENDOR_ID, FORTIS_BIOMETRIC_PRODUCT_ID) },
    { }
};
MODULE_DEVICE_TABLE(usb, fortis_biometric_table);

/* IOCTL commands */
#define FORTIS_BIOMETRIC_IOCTL_CAPTURE_FINGERPRINT    _IOW('F', 1, int)
#define FORTIS_BIOMETRIC_IOCTL_CAPTURE_FACIAL         _IOW('F', 2, int)
#define FORTIS_BIOMETRIC_IOCTL_GET_STATUS             _IOR('F', 3, int)
#define FORTIS_BIOMETRIC_IOCTL_SET_ENCRYPTION         _IOW('F', 4, int)
#define FORTIS_BIOMETRIC_IOCTL_GET_STATISTICS         _IOR('F', 5, int)

/* Module initialization */
static int __init fortis_biometric_init(void)
{
    int result;
    
    printk(KERN_INFO "FORTIS Biometric Driver: Initializing\n");
    
    /* Register USB driver */
    result = usb_register(&fortis_biometric_driver);
    if (result) {
        printk(KERN_ERR "FORTIS Biometric Driver: USB registration failed\n");
        return result;
    }
    
    /* Allocate major number */
    result = alloc_chrdev_region(&fortis_biometric_major, 0, FORTIS_BIOMETRIC_MAX_DEVICES,
                                "fortis_biometric");
    if (result < 0) {
        printk(KERN_ERR "FORTIS Biometric Driver: Major number allocation failed\n");
        usb_deregister(&fortis_biometric_driver);
        return result;
    }
    
    /* Create device class */
    fortis_biometric_class = class_create(THIS_MODULE, "fortis_biometric");
    if (IS_ERR(fortis_biometric_class)) {
        printk(KERN_ERR "FORTIS Biometric Driver: Class creation failed\n");
        unregister_chrdev_region(fortis_biometric_major, FORTIS_BIOMETRIC_MAX_DEVICES);
        usb_deregister(&fortis_biometric_driver);
        return PTR_ERR(fortis_biometric_class);
    }
    
    printk(KERN_INFO "FORTIS Biometric Driver: Initialized successfully\n");
    return 0;
}

/* Module cleanup */
static void __exit fortis_biometric_exit(void)
{
    printk(KERN_INFO "FORTIS Biometric Driver: Cleaning up\n");
    
    /* Destroy device class */
    class_destroy(fortis_biometric_class);
    
    /* Unregister character device */
    unregister_chrdev_region(fortis_biometric_major, FORTIS_BIOMETRIC_MAX_DEVICES);
    
    /* Deregister USB driver */
    usb_deregister(&fortis_biometric_driver);
    
    printk(KERN_INFO "FORTIS Biometric Driver: Cleanup complete\n");
}

/* USB driver structure */
static struct usb_driver fortis_biometric_driver = {
    .name = "fortis_biometric",
    .probe = fortis_biometric_probe,
    .disconnect = fortis_biometric_disconnect,
    .id_table = fortis_biometric_table,
};

/* Probe function */
static int fortis_biometric_probe(struct usb_interface *interface,
                                 const struct usb_device_id *id)
{
    struct usb_device *udev = interface_to_usbdev(interface);
    struct fortis_biometric_device *dev;
    struct usb_endpoint_descriptor *bulk_in, *bulk_out;
    int result;
    int i;
    
    printk(KERN_INFO "FORTIS Biometric Driver: Device connected\n");
    
    /* Find available device slot */
    for (i = 0; i < FORTIS_BIOMETRIC_MAX_DEVICES; i++) {
        if (fortis_biometric_devices[i] == NULL) {
            break;
        }
    }
    
    if (i >= FORTIS_BIOMETRIC_MAX_DEVICES) {
        printk(KERN_ERR "FORTIS Biometric Driver: No available device slots\n");
        return -ENOMEM;
    }
    
    /* Allocate device structure */
    dev = kzalloc(sizeof(struct fortis_biometric_device), GFP_KERNEL);
    if (!dev) {
        printk(KERN_ERR "FORTIS Biometric Driver: Memory allocation failed\n");
        return -ENOMEM;
    }
    
    /* Initialize device */
    dev->udev = usb_get_dev(udev);
    dev->interface = interface;
    dev->devt = MKDEV(fortis_biometric_major, i);
    dev->device_present = true;
    
    /* Initialize synchronization */
    mutex_init(&dev->mutex);
    init_completion(&dev->data_available);
    
    /* Create workqueue */
    dev->workqueue = create_singlethread_workqueue("fortis_biometric");
    if (!dev->workqueue) {
        printk(KERN_ERR "FORTIS Biometric Driver: Workqueue creation failed\n");
        kfree(dev);
        return -ENOMEM;
    }
    
    INIT_WORK(&dev->work, fortis_biometric_work_handler);
    
    /* Find bulk endpoints */
    result = usb_find_common_endpoints(interface->cur_altsetting,
                                      &bulk_in, &bulk_out, NULL, NULL);
    if (result) {
        printk(KERN_ERR "FORTIS Biometric Driver: Endpoint discovery failed\n");
        destroy_workqueue(dev->workqueue);
        kfree(dev);
        return result;
    }
    
    /* Allocate buffers */
    dev->bulk_in_size = usb_endpoint_maxp(bulk_in);
    dev->bulk_in_buffer = kmalloc(dev->bulk_in_size, GFP_KERNEL);
    if (!dev->bulk_in_buffer) {
        printk(KERN_ERR "FORTIS Biometric Driver: Bulk in buffer allocation failed\n");
        destroy_workqueue(dev->workqueue);
        kfree(dev);
        return -ENOMEM;
    }
    
    dev->bulk_out_size = usb_endpoint_maxp(bulk_out);
    dev->bulk_out_buffer = kmalloc(dev->bulk_out_size, GFP_KERNEL);
    if (!dev->bulk_out_buffer) {
        printk(KERN_ERR "FORTIS Biometric Driver: Bulk out buffer allocation failed\n");
        kfree(dev->bulk_in_buffer);
        destroy_workqueue(dev->workqueue);
        kfree(dev);
        return -ENOMEM;
    }
    
    /* Initialize encryption */
    dev->cipher = crypto_alloc_cipher("aes", 0, 0);
    if (IS_ERR(dev->cipher)) {
        printk(KERN_ERR "FORTIS Biometric Driver: Cipher allocation failed\n");
        kfree(dev->bulk_in_buffer);
        kfree(dev->bulk_out_buffer);
        destroy_workqueue(dev->workqueue);
        kfree(dev);
        return PTR_ERR(dev->cipher);
    }
    
    /* Generate encryption key */
    get_random_bytes(dev->encryption_key, sizeof(dev->encryption_key));
    crypto_cipher_setkey(dev->cipher, dev->encryption_key, 32);
    dev->encryption_enabled = true;
    
    /* Register character device */
    cdev_init(&dev->cdev, &fortis_biometric_fops);
    dev->cdev.owner = THIS_MODULE;
    result = cdev_add(&dev->cdev, dev->devt, 1);
    if (result) {
        printk(KERN_ERR "FORTIS Biometric Driver: Character device registration failed\n");
        crypto_free_cipher(dev->cipher);
        kfree(dev->bulk_in_buffer);
        kfree(dev->bulk_out_buffer);
        destroy_workqueue(dev->workqueue);
        kfree(dev);
        return result;
    }
    
    /* Create device file */
    dev->device = device_create(fortis_biometric_class, NULL, dev->devt, dev,
                               "fortis_biometric%d", i);
    if (IS_ERR(dev->device)) {
        printk(KERN_ERR "FORTIS Biometric Driver: Device file creation failed\n");
        cdev_del(&dev->cdev);
        crypto_free_cipher(dev->cipher);
        kfree(dev->bulk_in_buffer);
        kfree(dev->bulk_out_buffer);
        destroy_workqueue(dev->workqueue);
        kfree(dev);
        return PTR_ERR(dev->device);
    }
    
    /* Store device */
    fortis_biometric_devices[i] = dev;
    usb_set_intfdata(interface, dev);
    
    printk(KERN_INFO "FORTIS Biometric Driver: Device registered as /dev/fortis_biometric%d\n", i);
    return 0;
}

/* Disconnect function */
static void fortis_biometric_disconnect(struct usb_interface *interface)
{
    struct fortis_biometric_device *dev = usb_get_intfdata(interface);
    int i;
    
    printk(KERN_INFO "FORTIS Biometric Driver: Device disconnected\n");
    
    if (!dev) {
        return;
    }
    
    /* Find device index */
    for (i = 0; i < FORTIS_BIOMETRIC_MAX_DEVICES; i++) {
        if (fortis_biometric_devices[i] == dev) {
            break;
        }
    }
    
    if (i < FORTIS_BIOMETRIC_MAX_DEVICES) {
        fortis_biometric_devices[i] = NULL;
    }
    
    /* Mark device as not present */
    mutex_lock(&dev->mutex);
    dev->device_present = false;
    mutex_unlock(&dev->mutex);
    
    /* Wake up any waiting processes */
    complete_all(&dev->data_available);
    
    /* Clean up */
    device_destroy(fortis_biometric_class, dev->devt);
    cdev_del(&dev->cdev);
    crypto_free_cipher(dev->cipher);
    kfree(dev->bulk_in_buffer);
    kfree(dev->bulk_out_buffer);
    destroy_workqueue(dev->workqueue);
    usb_put_dev(dev->udev);
    kfree(dev);
}

/* Open function */
static int fortis_biometric_open(struct inode *inode, struct file *file)
{
    struct fortis_biometric_device *dev;
    int minor = iminor(inode);
    
    if (minor >= FORTIS_BIOMETRIC_MAX_DEVICES) {
        return -ENODEV;
    }
    
    dev = fortis_biometric_devices[minor];
    if (!dev) {
        return -ENODEV;
    }
    
    mutex_lock(&dev->mutex);
    if (dev->open_count) {
        mutex_unlock(&dev->mutex);
        return -EBUSY;
    }
    
    dev->open_count++;
    file->private_data = dev;
    mutex_unlock(&dev->mutex);
    
    printk(KERN_INFO "FORTIS Biometric Driver: Device opened\n");
    return 0;
}

/* Release function */
static int fortis_biometric_release(struct inode *inode, struct file *file)
{
    struct fortis_biometric_device *dev = file->private_data;
    
    mutex_lock(&dev->mutex);
    dev->open_count--;
    mutex_unlock(&dev->mutex);
    
    printk(KERN_INFO "FORTIS Biometric Driver: Device closed\n");
    return 0;
}

/* Read function */
static ssize_t fortis_biometric_read(struct file *file, char __user *buffer,
                                    size_t count, loff_t *ppos)
{
    struct fortis_biometric_device *dev = file->private_data;
    ssize_t result;
    
    if (!dev->device_present) {
        return -ENODEV;
    }
    
    mutex_lock(&dev->mutex);
    
    if (!dev->data_ready) {
        mutex_unlock(&dev->mutex);
        if (file->f_flags & O_NONBLOCK) {
            return -EAGAIN;
        }
        
        if (wait_for_completion_interruptible(&dev->data_available)) {
            return -ERESTARTSYS;
        }
        
        mutex_lock(&dev->mutex);
    }
    
    if (count > dev->fingerprint_size) {
        count = dev->fingerprint_size;
    }
    
    if (copy_to_user(buffer, dev->fingerprint_data, count)) {
        result = -EFAULT;
    } else {
        result = count;
        dev->data_ready = false;
    }
    
    mutex_unlock(&dev->mutex);
    return result;
}

/* Write function */
static ssize_t fortis_biometric_write(struct file *file, const char __user *buffer,
                                     size_t count, loff_t *ppos)
{
    struct fortis_biometric_device *dev = file->private_data;
    
    if (!dev->device_present) {
        return -ENODEV;
    }
    
    /* Write is not supported for biometric device */
    return -EOPNOTSUPP;
}

/* IOCTL function */
static long fortis_biometric_ioctl(struct file *file, unsigned int cmd,
                                  unsigned long arg)
{
    struct fortis_biometric_device *dev = file->private_data;
    int result = 0;
    
    if (!dev->device_present) {
        return -ENODEV;
    }
    
    switch (cmd) {
    case FORTIS_BIOMETRIC_IOCTL_CAPTURE_FINGERPRINT:
        result = fortis_biometric_capture_fingerprint(dev);
        break;
        
    case FORTIS_BIOMETRIC_IOCTL_CAPTURE_FACIAL:
        result = fortis_biometric_capture_facial(dev);
        break;
        
    case FORTIS_BIOMETRIC_IOCTL_GET_STATUS:
        result = put_user(dev->device_present ? 1 : 0, (int __user *)arg);
        break;
        
    case FORTIS_BIOMETRIC_IOCTL_SET_ENCRYPTION:
        mutex_lock(&dev->mutex);
        dev->encryption_enabled = (arg != 0);
        mutex_unlock(&dev->mutex);
        break;
        
    case FORTIS_BIOMETRIC_IOCTL_GET_STATISTICS:
        {
            struct {
                unsigned long capture_count;
                unsigned long error_count;
                unsigned long last_capture_time;
            } stats;
            
            mutex_lock(&dev->mutex);
            stats.capture_count = dev->capture_count;
            stats.error_count = dev->error_count;
            stats.last_capture_time = dev->last_capture_time;
            mutex_unlock(&dev->mutex);
            
            result = copy_to_user((void __user *)arg, &stats, sizeof(stats));
            if (result) {
                result = -EFAULT;
            } else {
                result = 0;
            }
        }
        break;
        
    default:
        result = -ENOTTY;
        break;
    }
    
    return result;
}

/* Capture fingerprint function */
static int fortis_biometric_capture_fingerprint(struct fortis_biometric_device *dev)
{
    int result;
    
    printk(KERN_INFO "FORTIS Biometric Driver: Capturing fingerprint\n");
    
    mutex_lock(&dev->mutex);
    
    /* Send capture command to device */
    dev->bulk_out_buffer[0] = 0x01; /* Command: Capture fingerprint */
    dev->bulk_out_buffer[1] = 0x00; /* Parameters */
    
    result = usb_bulk_msg(dev->udev, usb_sndbulkpipe(dev->udev, 0x01),
                         dev->bulk_out_buffer, 2, NULL, FORTIS_BIOMETRIC_TIMEOUT);
    if (result < 0) {
        printk(KERN_ERR "FORTIS Biometric Driver: Bulk out failed: %d\n", result);
        dev->error_count++;
        mutex_unlock(&dev->mutex);
        return result;
    }
    
    /* Read fingerprint data */
    result = usb_bulk_msg(dev->udev, usb_rcvbulkpipe(dev->udev, 0x81),
                         dev->bulk_in_buffer, dev->bulk_in_size, NULL, FORTIS_BIOMETRIC_TIMEOUT);
    if (result < 0) {
        printk(KERN_ERR "FORTIS Biometric Driver: Bulk in failed: %d\n", result);
        dev->error_count++;
        mutex_unlock(&dev->mutex);
        return result;
    }
    
    /* Store fingerprint data */
    dev->fingerprint_size = result;
    memcpy(dev->fingerprint_data, dev->bulk_in_buffer, result);
    
    /* Encrypt data if enabled */
    if (dev->encryption_enabled) {
        unsigned char encrypted[FORTIS_BIOMETRIC_BUFFER_SIZE];
        size_t encrypted_size;
        
        result = fortis_biometric_encrypt_data(dev, dev->fingerprint_data, dev->fingerprint_size,
                                              encrypted, &encrypted_size);
        if (result == 0) {
            memcpy(dev->fingerprint_data, encrypted, encrypted_size);
            dev->fingerprint_size = encrypted_size;
        }
    }
    
    dev->data_ready = true;
    dev->capture_count++;
    dev->last_capture_time = jiffies;
    
    mutex_unlock(&dev->mutex);
    
    /* Wake up waiting processes */
    complete(&dev->data_available);
    
    printk(KERN_INFO "FORTIS Biometric Driver: Fingerprint captured successfully\n");
    return 0;
}

/* Capture facial function */
static int fortis_biometric_capture_facial(struct fortis_biometric_device *dev)
{
    int result;
    
    printk(KERN_INFO "FORTIS Biometric Driver: Capturing facial data\n");
    
    mutex_lock(&dev->mutex);
    
    /* Send capture command to device */
    dev->bulk_out_buffer[0] = 0x02; /* Command: Capture facial */
    dev->bulk_out_buffer[1] = 0x00; /* Parameters */
    
    result = usb_bulk_msg(dev->udev, usb_sndbulkpipe(dev->udev, 0x01),
                         dev->bulk_out_buffer, 2, NULL, FORTIS_BIOMETRIC_TIMEOUT);
    if (result < 0) {
        printk(KERN_ERR "FORTIS Biometric Driver: Bulk out failed: %d\n", result);
        dev->error_count++;
        mutex_unlock(&dev->mutex);
        return result;
    }
    
    /* Read facial data */
    result = usb_bulk_msg(dev->udev, usb_rcvbulkpipe(dev->udev, 0x81),
                         dev->bulk_in_buffer, dev->bulk_in_size, NULL, FORTIS_BIOMETRIC_TIMEOUT);
    if (result < 0) {
        printk(KERN_ERR "FORTIS Biometric Driver: Bulk in failed: %d\n", result);
        dev->error_count++;
        mutex_unlock(&dev->mutex);
        return result;
    }
    
    /* Store facial data */
    dev->facial_size = result;
    memcpy(dev->facial_data, dev->bulk_in_buffer, result);
    
    /* Encrypt data if enabled */
    if (dev->encryption_enabled) {
        unsigned char encrypted[FORTIS_BIOMETRIC_BUFFER_SIZE];
        size_t encrypted_size;
        
        result = fortis_biometric_encrypt_data(dev, dev->facial_data, dev->facial_size,
                                              encrypted, &encrypted_size);
        if (result == 0) {
            memcpy(dev->facial_data, encrypted, encrypted_size);
            dev->facial_size = encrypted_size;
        }
    }
    
    dev->data_ready = true;
    dev->capture_count++;
    dev->last_capture_time = jiffies;
    
    mutex_unlock(&dev->mutex);
    
    /* Wake up waiting processes */
    complete(&dev->data_available);
    
    printk(KERN_INFO "FORTIS Biometric Driver: Facial data captured successfully\n");
    return 0;
}

/* Encrypt data function */
static int fortis_biometric_encrypt_data(struct fortis_biometric_device *dev,
                                        const unsigned char *data, size_t size,
                                        unsigned char *encrypted, size_t *encrypted_size)
{
    struct scatterlist sg;
    struct crypto_cipher *cipher = dev->cipher;
    unsigned char iv[16];
    int result;
    
    if (size > FORTIS_BIOMETRIC_BUFFER_SIZE - 16) {
        return -EINVAL;
    }
    
    /* Generate random IV */
    get_random_bytes(iv, sizeof(iv));
    
    /* Copy IV to encrypted data */
    memcpy(encrypted, iv, sizeof(iv));
    
    /* Prepare scatterlist */
    sg_init_one(&sg, encrypted + sizeof(iv), size);
    
    /* Encrypt data */
    crypto_cipher_set_iv(cipher, iv, sizeof(iv));
    crypto_cipher_encrypt_one(cipher, sg.sg_data, data);
    
    *encrypted_size = size + sizeof(iv);
    return 0;
}

/* Decrypt data function */
static int fortis_biometric_decrypt_data(struct fortis_biometric_device *dev,
                                        const unsigned char *encrypted, size_t size,
                                        unsigned char *data, size_t *data_size)
{
    struct scatterlist sg;
    struct crypto_cipher *cipher = dev->cipher;
    unsigned char iv[16];
    int result;
    
    if (size < 16) {
        return -EINVAL;
    }
    
    /* Extract IV */
    memcpy(iv, encrypted, sizeof(iv));
    
    /* Prepare scatterlist */
    sg_init_one(&sg, data, size - sizeof(iv));
    
    /* Decrypt data */
    crypto_cipher_set_iv(cipher, iv, sizeof(iv));
    crypto_cipher_decrypt_one(cipher, data, encrypted + sizeof(iv));
    
    *data_size = size - sizeof(iv);
    return 0;
}

/* Work handler function */
static void fortis_biometric_work_handler(struct work_struct *work)
{
    struct fortis_biometric_device *dev = container_of(work, struct fortis_biometric_device, work);
    
    /* Process any pending work */
    printk(KERN_DEBUG "FORTIS Biometric Driver: Work handler executed\n");
}

/* Bulk callback function */
static void fortis_biometric_bulk_callback(struct urb *urb)
{
    struct fortis_biometric_device *dev = urb->context;
    
    if (urb->status) {
        printk(KERN_ERR "FORTIS Biometric Driver: Bulk transfer failed: %d\n", urb->status);
        dev->error_count++;
    } else {
        printk(KERN_DEBUG "FORTIS Biometric Driver: Bulk transfer completed\n");
    }
}

/* Module information */
MODULE_LICENSE("MIT");
MODULE_AUTHOR("FORTIS Team");
MODULE_DESCRIPTION("FORTIS Biometric Reader Driver");
MODULE_VERSION("1.0");

module_init(fortis_biometric_init);
module_exit(fortis_biometric_exit);
