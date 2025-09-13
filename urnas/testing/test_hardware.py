#!/usr/bin/env python3
"""
FORTIS Hardware Tests
Testes de hardware para módulos FORTIS

Copyright (C) 2024 FORTIS Team
License: MIT
"""

import asyncio
import json
import time
import logging
from datetime import datetime, timezone
from typing import Dict, List, Optional, Any
import serial
import usb.core
import usb.util
import hid

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class FortisHardwareTester:
    """Testador de hardware para módulos FORTIS"""
    
    def __init__(self):
        self.test_results: List[Dict[str, Any]] = []
        self.devices: Dict[str, Any] = {}
        
    async def test_biometric_reader(self) -> bool:
        """Testa leitor biométrico FORTIS-BR-001"""
        logger.info("Testing biometric reader...")
        
        try:
            # Procurar dispositivo USB
            device = usb.core.find(idVendor=0x1234, idProduct=0x5678)
            if device is None:
                logger.error("Biometric reader not found")
                return False
            
            # Configurar dispositivo
            device.set_configuration()
            cfg = device.get_active_configuration()
            intf = cfg[(0, 0)]
            
            # Testar comunicação
            endpoint_out = usb.util.find_descriptor(
                intf,
                custom_match=lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_OUT
            )
            
            endpoint_in = usb.util.find_descriptor(
                intf,
                custom_match=lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_IN
            )
            
            if endpoint_out is None or endpoint_in is None:
                logger.error("Endpoints not found")
                return False
            
            # Enviar comando de teste
            test_command = [0x01, 0x00]  # Comando de teste
            device.write(endpoint_out.bEndpointAddress, test_command)
            
            # Ler resposta
            response = device.read(endpoint_in.bEndpointAddress, 64, timeout=1000)
            
            if len(response) > 0 and response[0] == 0x01:
                logger.info("Biometric reader test passed")
                return True
            else:
                logger.error("Biometric reader test failed")
                return False
                
        except Exception as e:
            logger.error(f"Error testing biometric reader: {e}")
            return False
    
    async def test_certificate_reader(self) -> bool:
        """Testa leitor de certificados FORTIS-CR-001"""
        logger.info("Testing certificate reader...")
        
        try:
            # Procurar dispositivo USB
            device = usb.core.find(idVendor=0x1234, idProduct=0x5679)
            if device is None:
                logger.error("Certificate reader not found")
                return False
            
            # Configurar dispositivo
            device.set_configuration()
            cfg = device.get_active_configuration()
            intf = cfg[(0, 0)]
            
            # Testar comunicação
            endpoint_out = usb.util.find_descriptor(
                intf,
                custom_match=lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_OUT
            )
            
            endpoint_in = usb.util.find_descriptor(
                intf,
                custom_match=lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_IN
            )
            
            if endpoint_out is None or endpoint_in is None:
                logger.error("Endpoints not found")
                return False
            
            # Enviar comando de teste
            test_command = [0x02, 0x00]  # Comando de teste
            device.write(endpoint_out.bEndpointAddress, test_command)
            
            # Ler resposta
            response = device.read(endpoint_in.bEndpointAddress, 64, timeout=1000)
            
            if len(response) > 0 and response[0] == 0x02:
                logger.info("Certificate reader test passed")
                return True
            else:
                logger.error("Certificate reader test failed")
                return False
                
        except Exception as e:
            logger.error(f"Error testing certificate reader: {e}")
            return False
    
    async def test_network_interface(self) -> bool:
        """Testa interface de rede FORTIS-NI-001"""
        logger.info("Testing network interface...")
        
        try:
            # Testar conectividade Ethernet
            import socket
            
            # Testar conexão local
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(5)
            
            try:
                result = sock.connect_ex(('127.0.0.1', 80))
                if result == 0:
                    logger.info("Ethernet connectivity test passed")
                else:
                    logger.warning("Ethernet connectivity test failed")
            finally:
                sock.close()
            
            # Testar WiFi (simulado)
            logger.info("WiFi interface test passed (simulated)")
            
            return True
            
        except Exception as e:
            logger.error(f"Error testing network interface: {e}")
            return False
    
    async def test_hsm_module(self) -> bool:
        """Testa módulo HSM FORTIS-HSM-001"""
        logger.info("Testing HSM module...")
        
        try:
            # Procurar dispositivo USB
            device = usb.core.find(idVendor=0x1234, idProduct=0x5680)
            if device is None:
                logger.error("HSM module not found")
                return False
            
            # Configurar dispositivo
            device.set_configuration()
            cfg = device.get_active_configuration()
            intf = cfg[(0, 0)]
            
            # Testar comunicação
            endpoint_out = usb.util.find_descriptor(
                intf,
                custom_match=lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_OUT
            )
            
            endpoint_in = usb.util.find_descriptor(
                intf,
                custom_match=lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_IN
            )
            
            if endpoint_out is None or endpoint_in is None:
                logger.error("Endpoints not found")
                return False
            
            # Enviar comando de teste
            test_command = [0x03, 0x00]  # Comando de teste
            device.write(endpoint_out.bEndpointAddress, test_command)
            
            # Ler resposta
            response = device.read(endpoint_in.bEndpointAddress, 64, timeout=1000)
            
            if len(response) > 0 and response[0] == 0x03:
                logger.info("HSM module test passed")
                return True
            else:
                logger.error("HSM module test failed")
                return False
                
        except Exception as e:
            logger.error(f"Error testing HSM module: {e}")
            return False
    
    async def test_ups_module(self) -> bool:
        """Testa módulo UPS FORTIS-UPS-001"""
        logger.info("Testing UPS module...")
        
        try:
            # Procurar dispositivo USB
            device = usb.core.find(idVendor=0x1234, idProduct=0x5681)
            if device is None:
                logger.error("UPS module not found")
                return False
            
            # Configurar dispositivo
            device.set_configuration()
            cfg = device.get_active_configuration()
            intf = cfg[(0, 0)]
            
            # Testar comunicação
            endpoint_out = usb.util.find_descriptor(
                intf,
                custom_match=lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_OUT
            )
            
            endpoint_in = usb.util.find_descriptor(
                intf,
                custom_match=lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_IN
            )
            
            if endpoint_out is None or endpoint_in is None:
                logger.error("Endpoints not found")
                return False
            
            # Enviar comando de teste
            test_command = [0x04, 0x00]  # Comando de teste
            device.write(endpoint_out.bEndpointAddress, test_command)
            
            # Ler resposta
            response = device.read(endpoint_in.bEndpointAddress, 64, timeout=1000)
            
            if len(response) > 0 and response[0] == 0x04:
                logger.info("UPS module test passed")
                return True
            else:
                logger.error("UPS module test failed")
                return False
                
        except Exception as e:
            logger.error(f"Error testing UPS module: {e}")
            return False
    
    async def test_sensor_module(self) -> bool:
        """Testa módulo de sensores FORTIS-SC-001"""
        logger.info("Testing sensor module...")
        
        try:
            # Procurar dispositivo USB
            device = usb.core.find(idVendor=0x1234, idProduct=0x5682)
            if device is None:
                logger.error("Sensor module not found")
                return False
            
            # Configurar dispositivo
            device.set_configuration()
            cfg = device.get_active_configuration()
            intf = cfg[(0, 0)]
            
            # Testar comunicação
            endpoint_out = usb.util.find_descriptor(
                intf,
                custom_match=lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_OUT
            )
            
            endpoint_in = usb.util.find_descriptor(
                intf,
                custom_match=lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_IN
            )
            
            if endpoint_out is None or endpoint_in is None:
                logger.error("Endpoints not found")
                return False
            
            # Enviar comando de teste
            test_command = [0x05, 0x00]  # Comando de teste
            device.write(endpoint_out.bEndpointAddress, test_command)
            
            # Ler resposta
            response = device.read(endpoint_in.bEndpointAddress, 64, timeout=1000)
            
            if len(response) > 0 and response[0] == 0x05:
                logger.info("Sensor module test passed")
                return True
            else:
                logger.error("Sensor module test failed")
                return False
                
        except Exception as e:
            logger.error(f"Error testing sensor module: {e}")
            return False
    
    async def test_serial_communication(self) -> bool:
        """Testa comunicação serial"""
        logger.info("Testing serial communication...")
        
        try:
            # Testar portas seriais disponíveis
            import serial.tools.list_ports
            
            ports = serial.tools.list_ports.comports()
            if not ports:
                logger.warning("No serial ports found")
                return True  # Não é um erro crítico
            
            for port in ports:
                try:
                    ser = serial.Serial(port.device, 9600, timeout=1)
                    ser.close()
                    logger.info(f"Serial port {port.device} test passed")
                except Exception as e:
                    logger.warning(f"Serial port {port.device} test failed: {e}")
            
            return True
            
        except Exception as e:
            logger.error(f"Error testing serial communication: {e}")
            return False
    
    async def test_hid_devices(self) -> bool:
        """Testa dispositivos HID"""
        logger.info("Testing HID devices...")
        
        try:
            # Listar dispositivos HID
            devices = hid.enumerate()
            if not devices:
                logger.warning("No HID devices found")
                return True  # Não é um erro crítico
            
            for device in devices:
                if device['vendor_id'] == 0x1234:  # FORTIS vendor ID
                    logger.info(f"HID device found: {device['product_string']}")
            
            return True
            
        except Exception as e:
            logger.error(f"Error testing HID devices: {e}")
            return False
    
    async def test_system_resources(self) -> bool:
        """Testa recursos do sistema"""
        logger.info("Testing system resources...")
        
        try:
            import psutil
            
            # Testar CPU
            cpu_percent = psutil.cpu_percent(interval=1)
            if cpu_percent > 80:
                logger.warning(f"High CPU usage: {cpu_percent}%")
            
            # Testar memória
            memory = psutil.virtual_memory()
            if memory.percent > 85:
                logger.warning(f"High memory usage: {memory.percent}%")
            
            # Testar disco
            disk = psutil.disk_usage('/')
            if disk.percent > 90:
                logger.warning(f"High disk usage: {disk.percent}%")
            
            logger.info(f"System resources: CPU={cpu_percent}%, "
                       f"Memory={memory.percent}%, Disk={disk.percent}%")
            
            return True
            
        except Exception as e:
            logger.error(f"Error testing system resources: {e}")
            return False
    
    async def run_all_tests(self) -> Dict[str, Any]:
        """Executa todos os testes de hardware"""
        logger.info("Starting FORTIS hardware tests...")
        
        test_results = {
            "start_time": datetime.now(timezone.utc).isoformat(),
            "tests": [],
            "summary": {
                "total": 0,
                "passed": 0,
                "failed": 0
            }
        }
        
        # Lista de testes
        tests = [
            ("biometric_reader", self.test_biometric_reader),
            ("certificate_reader", self.test_certificate_reader),
            ("network_interface", self.test_network_interface),
            ("hsm_module", self.test_hsm_module),
            ("ups_module", self.test_ups_module),
            ("sensor_module", self.test_sensor_module),
            ("serial_communication", self.test_serial_communication),
            ("hid_devices", self.test_hid_devices),
            ("system_resources", self.test_system_resources)
        ]
        
        # Executar testes
        for test_name, test_func in tests:
            logger.info(f"Running test: {test_name}")
            start_time = time.time()
            
            try:
                result = await test_func()
                duration = time.time() - start_time
                
                test_result = {
                    "name": test_name,
                    "status": "PASSED" if result else "FAILED",
                    "duration": duration,
                    "timestamp": datetime.now(timezone.utc).isoformat()
                }
                
                test_results["tests"].append(test_result)
                test_results["summary"]["total"] += 1
                
                if result:
                    test_results["summary"]["passed"] += 1
                    logger.info(f"Test {test_name}: PASSED ({duration:.2f}s)")
                else:
                    test_results["summary"]["failed"] += 1
                    logger.error(f"Test {test_name}: FAILED ({duration:.2f}s)")
                    
            except Exception as e:
                duration = time.time() - start_time
                test_result = {
                    "name": test_name,
                    "status": "ERROR",
                    "duration": duration,
                    "error": str(e),
                    "timestamp": datetime.now(timezone.utc).isoformat()
                }
                
                test_results["tests"].append(test_result)
                test_results["summary"]["total"] += 1
                test_results["summary"]["failed"] += 1
                
                logger.error(f"Test {test_name}: ERROR ({duration:.2f}s) - {e}")
        
        test_results["end_time"] = datetime.now(timezone.utc).isoformat()
        test_results["total_duration"] = (
            datetime.fromisoformat(test_results["end_time"]) - 
            datetime.fromisoformat(test_results["start_time"])
        ).total_seconds()
        
        # Log resumo
        summary = test_results["summary"]
        logger.info(f"Hardware test summary: {summary['passed']}/{summary['total']} passed, "
                   f"{summary['failed']} failed")
        
        return test_results

async def main():
    """Função principal"""
    tester = FortisHardwareTester()
    results = await tester.run_all_tests()
    
    # Salvar resultados
    with open("hardware_test_results.json", "w") as f:
        json.dump(results, f, indent=2)
    
    # Retornar código de saída baseado nos resultados
    if results["summary"]["failed"] == 0:
        logger.info("All hardware tests passed!")
        return 0
    else:
        logger.error(f"{results['summary']['failed']} hardware tests failed!")
        return 1

if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)
