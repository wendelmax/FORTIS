# FORTIS Hardware Specifications
## Especificações de Hardware para Urnas Eletrônicas

### 🎯 **Visão Geral**

O hardware FORTIS foi projetado para integrar-se perfeitamente com as urnas eletrônicas brasileiras existentes, adicionando módulos de segurança, autenticação biométrica e conectividade sem alterar a interface familiar para eleitores e mesários.

### 🏗️ **Arquitetura do Hardware**

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                            URNA ELETRÔNICA FORTIS                              │
├─────────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────────────────┐    │
│  │                    HARDWARE EXISTENTE                                 │    │
│  │  • CPU: ARM Cortex-A78 / x86-64                                       │    │
│  │  • RAM: 4GB DDR4                                                      │    │
│  │  • Storage: 32GB eMMC/UFS                                             │    │
│  │  • Display: 10.1" LCD 1920x1200                                       │    │
│  │  • Touch: Capacitivo multi-touch                                      │    │
│  │  • Teclado: Numérico 12 teclas                                        │    │
│  │  • Impressora: Térmica 80mm                                           │    │
│  └─────────────────────────────────────────────────────────────────────────┘    │
│  ┌─────────────────────────────────────────────────────────────────────────┐    │
│  │                    MÓDULOS FORTIS                                     │    │
│  │  • FORTIS-BR-001: Leitor Biométrico                                   │    │
│  │  • FORTIS-CR-001: Leitor de Certificados                              │    │
│  │  • FORTIS-NI-001: Interface de Rede                                   │    │
│  │  • FORTIS-HSM-001: Hardware Security Module                           │    │
│  │  • FORTIS-UPS-001: Bateria de Backup                                  │    │
│  │  • FORTIS-SC-001: Sensor de Violação                                  │    │
│  └─────────────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 🔧 **Especificações Detalhadas**

#### **1. Leitor Biométrico FORTIS-BR-001**

**Características:**
- **Tipo**: Óptico + Capacitivo
- **Resolução**: 500 DPI
- **Área de Captura**: 15.6mm x 15.6mm
- **Tempo de Captura**: < 1 segundo
- **Precisão**: 99.9%
- **Falsos Positivos**: < 0.01%
- **Falsos Negativos**: < 0.1%

**Especificações Técnicas:**
- **Interface**: USB 3.0
- **Alimentação**: 5V DC, 100mA
- **Temperatura**: -10°C a +60°C
- **Umidade**: 10% a 90% RH
- **Dimensões**: 60mm x 40mm x 15mm
- **Peso**: 50g

**Recursos de Segurança:**
- Detecção de dedos falsos
- Criptografia de dados biométricos
- Armazenamento seguro de templates
- Auto-limpeza após cada uso

#### **2. Leitor de Certificados FORTIS-CR-001**

**Características:**
- **Tipos Suportados**: USB Token, Smart Card, NFC
- **Padrões**: ICP-Brasil, X.509 v3
- **Algoritmos**: RSA 2048/4096, ECDSA P-256/P-384
- **Tempo de Leitura**: < 2 segundos
- **Compatibilidade**: 100% ICP-Brasil

**Especificações Técnicas:**
- **Interface**: USB 2.0 + NFC
- **Alimentação**: 5V DC, 200mA
- **Temperatura**: -10°C a +60°C
- **Dimensões**: 80mm x 50mm x 20mm
- **Peso**: 80g

**Recursos de Segurança:**
- Verificação de assinatura digital
- Validação de cadeia de certificados
- Verificação de revogação (CRL/OCSP)
- Armazenamento seguro de chaves

#### **3. Interface de Rede FORTIS-NI-001**

**Características:**
- **Ethernet**: Gigabit (1000BASE-T)
- **WiFi**: 802.11ax (WiFi 6)
- **Cellular**: 4G LTE / 5G
- **Bluetooth**: 5.2 (para configuração)
- **VPN**: IPSec, OpenVPN
- **Firewall**: Hardware integrado

**Especificações Técnicas:**
- **Interface**: PCIe x1
- **Alimentação**: 12V DC, 500mA
- **Temperatura**: -20°C a +70°C
- **Dimensões**: 100mm x 60mm x 15mm
- **Peso**: 100g

**Recursos de Segurança:**
- Criptografia de ponta a ponta
- Certificado pinning
- Detecção de intrusão
- Isolamento de rede

#### **4. Hardware Security Module FORTIS-HSM-001**

**Características:**
- **Certificação**: FIPS 140-2 Level 3
- **Algoritmos**: AES-256, RSA-4096, ECDSA P-384
- **Chaves**: Até 1000 chaves simultâneas
- **Performance**: 1000 operações/segundo
- **Temperatura**: -40°C a +85°C

**Especificações Técnicas:**
- **Interface**: USB 3.0 + SPI
- **Alimentação**: 5V DC, 300mA
- **Dimensões**: 50mm x 30mm x 10mm
- **Peso**: 30g

**Recursos de Segurança:**
- Proteção contra ataques físicos
- Zeroização automática em violação
- Geração de números aleatórios
- Assinatura digital segura

#### **5. Bateria de Backup FORTIS-UPS-001**

**Características:**
- **Tipo**: LiFePO4 (Lítio Ferro Fosfato)
- **Capacidade**: 50Wh
- **Autonomia**: 4 horas (urna ativa)
- **Tempo de Carregamento**: 2 horas
- **Ciclos**: 2000+ ciclos
- **Temperatura**: -20°C a +60°C

**Especificações Técnicas:**
- **Voltagem**: 12V DC
- **Corrente**: 4.2A
- **Dimensões**: 120mm x 80mm x 30mm
- **Peso**: 500g

**Recursos de Segurança:**
- Proteção contra sobrecarga
- Proteção contra descarga profunda
- Monitoramento de temperatura
- Indicador de status

#### **6. Sensor de Violação FORTIS-SC-001**

**Características:**
- **Sensores**: 8 sensores de abertura
- **Detecção**: Magnética + Capacitiva
- **Tempo de Resposta**: < 100ms
- **Alcance**: 5mm
- **Temperatura**: -40°C a +85°C

**Especificações Técnicas:**
- **Interface**: I2C
- **Alimentação**: 3.3V DC, 50mA
- **Dimensões**: 200mm x 150mm x 5mm
- **Peso**: 100g

**Recursos de Segurança:**
- Detecção de violação em tempo real
- Log de eventos de segurança
- Alarme sonoro e visual
- Desligamento automático

### 🔌 **Interfaces e Conectores**

#### **Conectores Principais**
- **USB 3.0**: 2 portas (biométrico + certificado)
- **Ethernet**: RJ45 Gigabit
- **Display**: LVDS 24-pin
- **Touch**: I2C 4-pin
- **Teclado**: GPIO 12-pin
- **Impressora**: Serial RS-232
- **Alimentação**: DC 12V 5A

#### **Conectores FORTIS**
- **FORTIS-BUS**: 40-pin (módulos FORTIS)
- **HSM-Interface**: SPI 6-pin
- **Sensor-BUS**: I2C 4-pin
- **UPS-Interface**: I2C 4-pin

### 🛡️ **Segurança Física**

#### **Proteção contra Violação**
- **Carcaça**: Alumínio anodizado 2mm
- **Parafusos**: Torx de segurança
- **Selos**: Destrutivos numerados
- **Sensores**: 8 pontos de detecção
- **Tinta**: Reativa a solventes

#### **Proteção Ambiental**
- **IP Rating**: IP65 (prova d'água)
- **Temperatura**: -20°C a +60°C
- **Umidade**: 10% a 90% RH
- **Vibração**: 5G (10-2000Hz)
- **Choque**: 30G (11ms)

#### **Proteção Eletromagnética**
- **EMC**: EN 55022 Class A
- **Immunity**: EN 55024
- **ESD**: IEC 61000-4-2 (8kV)
- **Surge**: IEC 61000-4-5 (2kV)

### 📊 **Especificações de Performance**

#### **Processamento**
- **CPU**: ARM Cortex-A78 2.0GHz / Intel Core i5
- **RAM**: 4GB DDR4-3200
- **Storage**: 32GB eMMC 5.1
- **GPU**: Mali-G78 / Intel UHD Graphics

#### **Rede**
- **Ethernet**: 1 Gbps
- **WiFi**: 802.11ax (1.2 Gbps)
- **Cellular**: 4G LTE (100 Mbps) / 5G (1 Gbps)
- **Latência**: < 50ms (local), < 200ms (remoto)

#### **Energia**
- **Consumo**: 15W (idle), 25W (ativa)
- **UPS**: 4 horas de autonomia
- **Eficiência**: 90% (80 Plus Gold)

### 🔧 **Instalação e Manutenção**

#### **Instalação**
1. **Desligar urna existente**
2. **Remover painel traseiro**
3. **Instalar módulos FORTIS**
4. **Conectar cabos de interface**
5. **Instalar software FORTIS**
6. **Testar funcionalidades**
7. **Selar e numerar**

#### **Manutenção**
- **Preventiva**: Mensal
- **Corretiva**: Imediata
- **Atualizações**: Automáticas
- **Calibração**: Trimestral
- **Substituição**: 5 anos

#### **Ferramentas Necessárias**
- Chave Torx T10, T15, T20
- Multímetro digital
- Osciloscópio (opcional)
- Software de diagnóstico FORTIS
- Certificados de calibração

### 📋 **Certificações e Compliance**

#### **Certificações de Segurança**
- **FIPS 140-2 Level 3** (HSM)
- **Common Criteria EAL4+**
- **ISO 27001**
- **NIST SP 800-53**

#### **Certificações de Hardware**
- **CE Marking** (Europa)
- **FCC Class A** (Estados Unidos)
- **ANATEL** (Brasil)
- **IC** (Canadá)

#### **Padrões de Qualidade**
- **ISO 9001:2015**
- **ISO 14001:2015**
- **IEC 62443** (Cibersegurança)
- **IEC 61508** (Segurança Funcional)

### 💰 **Custos e Disponibilidade**

#### **Custo por Módulo**
- **FORTIS-BR-001**: R$ 800,00
- **FORTIS-CR-001**: R$ 600,00
- **FORTIS-NI-001**: R$ 1.200,00
- **FORTIS-HSM-001**: R$ 2.000,00
- **FORTIS-UPS-001**: R$ 1.500,00
- **FORTIS-SC-001**: R$ 400,00

#### **Custo Total por Urna**
- **Módulos FORTIS**: R$ 6.500,00
- **Instalação**: R$ 500,00
- **Software**: R$ 1.000,00
- **Certificação**: R$ 300,00
- **Total**: R$ 8.300,00

#### **Disponibilidade**
- **Lead Time**: 4-6 semanas
- **Garantia**: 3 anos
- **Suporte**: 24/7
- **Reposição**: 48 horas

### 🎯 **Benefícios do Hardware FORTIS**

#### **Segurança**
- **Proteção física** contra violação
- **Criptografia** de nível militar
- **Autenticação** biométrica obrigatória
- **Auditoria** completa de eventos

#### **Confiabilidade**
- **Uptime** de 99.99%
- **MTBF** de 50.000 horas
- **Recuperação** automática de falhas
- **Backup** de energia contínuo

#### **Integração**
- **Compatibilidade** total com hardware existente
- **Interface** familiar preservada
- **Instalação** não-invasiva
- **Manutenção** simplificada

#### **Escalabilidade**
- **Modular** e expansível
- **Atualizações** via software
- **Configuração** remota
- **Monitoramento** centralizado

---

*FORTIS Hardware v1.0 - Especificações Técnicas para Democracia Digital*
