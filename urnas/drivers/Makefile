# FORTIS Drivers Makefile
# Makefile para compilação dos drivers FORTIS

# Kernel version
KERNEL_VERSION := $(shell uname -r)

# Kernel source directory
KERNEL_SRC := /lib/modules/$(KERNEL_VERSION)/build

# Module directory
MODULE_DIR := /lib/modules/$(KERNEL_VERSION)/extra

# Compiler
CC := gcc

# Compiler flags
CFLAGS := -Wall -Wextra -O2 -std=gnu99

# Module flags
MODULE_CFLAGS := -DMODULE -D__KERNEL__ -DLINUX

# Include directories
INCLUDES := -I$(KERNEL_SRC)/include

# Source files
BIOMETRIC_SRC := fortis_biometric.c
CERTIFICATE_SRC := fortis_certificate.c
NETWORK_SRC := fortis_network.c
HSM_SRC := fortis_hsm.c
UPS_SRC := fortis_ups.c
SENSOR_SRC := fortis_sensor.c

# Object files
BIOMETRIC_OBJ := $(BIOMETRIC_SRC:.c=.o)
CERTIFICATE_OBJ := $(CERTIFICATE_SRC:.c=.o)
NETWORK_OBJ := $(NETWORK_SRC:.c=.o)
HSM_OBJ := $(HSM_SRC:.c=.o)
UPS_OBJ := $(UPS_SRC:.c=.o)
SENSOR_OBJ := $(SENSOR_SRC:.c=.o)

# Module files
BIOMETRIC_KO := fortis_biometric.ko
CERTIFICATE_KO := fortis_certificate.ko
NETWORK_KO := fortis_network.ko
HSM_KO := fortis_hsm.ko
UPS_KO := fortis_ups.ko
SENSOR_KO := fortis_sensor.ko

# All modules
MODULES := $(BIOMETRIC_KO) $(CERTIFICATE_KO) $(NETWORK_KO) $(HSM_KO) $(UPS_KO) $(SENSOR_KO)

# Default target
all: $(MODULES)

# Biometric module
$(BIOMETRIC_KO): $(BIOMETRIC_OBJ)
	$(MAKE) -C $(KERNEL_SRC) M=$(PWD) modules
	@echo "Built $(BIOMETRIC_KO)"

# Certificate module
$(CERTIFICATE_KO): $(CERTIFICATE_OBJ)
	$(MAKE) -C $(KERNEL_SRC) M=$(PWD) modules
	@echo "Built $(CERTIFICATE_KO)"

# Network module
$(NETWORK_KO): $(NETWORK_OBJ)
	$(MAKE) -C $(KERNEL_SRC) M=$(PWD) modules
	@echo "Built $(NETWORK_KO)"

# HSM module
$(HSM_KO): $(HSM_OBJ)
	$(MAKE) -C $(KERNEL_SRC) M=$(PWD) modules
	@echo "Built $(HSM_KO)"

# UPS module
$(UPS_KO): $(UPS_OBJ)
	$(MAKE) -C $(KERNEL_SRC) M=$(PWD) modules
	@echo "Built $(UPS_KO)"

# Sensor module
$(SENSOR_KO): $(SENSOR_OBJ)
	$(MAKE) -C $(KERNEL_SRC) M=$(PWD) modules
	@echo "Built $(SENSOR_KO)"

# Compile C files
%.o: %.c
	$(CC) $(CFLAGS) $(MODULE_CFLAGS) $(INCLUDES) -c $< -o $@

# Install modules
install: $(MODULES)
	@echo "Installing FORTIS drivers..."
	sudo mkdir -p $(MODULE_DIR)
	sudo cp $(MODULES) $(MODULE_DIR)/
	sudo depmod -a
	@echo "FORTIS drivers installed successfully"

# Load modules
load: $(MODULES)
	@echo "Loading FORTIS drivers..."
	sudo modprobe fortis_biometric
	sudo modprobe fortis_certificate
	sudo modprobe fortis_network
	sudo modprobe fortis_hsm
	sudo modprobe fortis_ups
	sudo modprobe fortis_sensor
	@echo "FORTIS drivers loaded successfully"

# Unload modules
unload:
	@echo "Unloading FORTIS drivers..."
	sudo modprobe -r fortis_sensor
	sudo modprobe -r fortis_ups
	sudo modprobe -r fortis_hsm
	sudo modprobe -r fortis_network
	sudo modprobe -r fortis_certificate
	sudo modprobe -r fortis_biometric
	@echo "FORTIS drivers unloaded successfully"

# Remove modules
remove: unload
	@echo "Removing FORTIS drivers..."
	sudo rm -f $(MODULE_DIR)/fortis_*.ko
	sudo depmod -a
	@echo "FORTIS drivers removed successfully"

# Clean build files
clean:
	@echo "Cleaning build files..."
	rm -f *.o *.ko *.mod.c *.mod.o *.symvers *.order
	rm -rf .tmp_versions
	@echo "Clean complete"

# Test modules
test: $(MODULES)
	@echo "Testing FORTIS drivers..."
	@echo "Checking module dependencies..."
	modinfo fortis_biometric.ko
	modinfo fortis_certificate.ko
	modinfo fortis_network.ko
	modinfo fortis_hsm.ko
	modinfo fortis_ups.ko
	modinfo fortis_sensor.ko
	@echo "Module test complete"

# Check module signatures
check: $(MODULES)
	@echo "Checking module signatures..."
	@for module in $(MODULES); do \
		echo "Checking $$module:"; \
		modinfo $$module | grep -E "(signature|vermagic)"; \
	done

# Generate module documentation
docs:
	@echo "Generating module documentation..."
	@for module in $(MODULES); do \
		echo "Documentation for $$module:"; \
		modinfo $$module; \
		echo "---"; \
	done > fortis_drivers_docs.txt
	@echo "Documentation generated: fortis_drivers_docs.txt"

# Help
help:
	@echo "FORTIS Drivers Makefile"
	@echo "======================="
	@echo "Available targets:"
	@echo "  all      - Build all modules"
	@echo "  install  - Install modules to system"
	@echo "  load     - Load modules into kernel"
	@echo "  unload   - Unload modules from kernel"
	@echo "  remove   - Remove modules from system"
	@echo "  clean    - Clean build files"
	@echo "  test     - Test module information"
	@echo "  check    - Check module signatures"
	@echo "  docs     - Generate documentation"
	@echo "  help     - Show this help"

# Phony targets
.PHONY: all install load unload remove clean test check docs help

# Module objects
obj-m := fortis_biometric.o fortis_certificate.o fortis_network.o fortis_hsm.o fortis_ups.o fortis_sensor.o
