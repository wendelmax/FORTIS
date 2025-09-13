import React, {useState} from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  TouchableOpacity,
} from 'react-native';
import {
  Card,
  Title,
  Paragraph,
  List,
  Accordion,
  Button,
} from 'react-native-paper';
import Icon from 'react-native-vector-icons/MaterialIcons';
import {colors, spacing, shadows} from '../styles/theme';

const HelpScreen: React.FC = () => {
  const [expandedItems, setExpandedItems] = useState<number[]>([]);

  const handleAccordionPress = (itemIndex: number) => {
    setExpandedItems(prev => 
      prev.includes(itemIndex) 
        ? prev.filter(index => index !== itemIndex)
        : [...prev, itemIndex]
    );
  };

  const faqData = [
    {
      title: 'Como votar no sistema FORTIS?',
      content: '1. Faça login com seu CPF e senha\n2. Autentique-se com biometria\n3. Selecione uma eleição ativa\n4. Escolha seu candidato\n5. Confirme seu voto\n6. Receba o comprovante',
    },
    {
      title: 'É seguro votar pelo celular?',
      content: 'Sim! O FORTIS utiliza:\n• Criptografia de ponta a ponta\n• Blockchain para auditoria\n• Zero-Knowledge Proofs para privacidade\n• Autenticação biométrica\n• Verificação de integridade',
    },
    {
      title: 'Posso votar mais de uma vez?',
      content: 'Não. O sistema impede votos duplicados através de:\n• Verificação de identidade única\n• Controle de tempo de eleição\n• Validação biométrica\n• Sistema de nullifiers',
    },
    {
      title: 'Como verificar se meu voto foi registrado?',
      content: 'Você receberá um comprovante com:\n• ID único do voto\n• Hash de verificação\n• Timestamp da votação\n• Informações do candidato\n• Link para verificação pública',
    },
    {
      title: 'O que fazer se o app travar?',
      content: '1. Feche e reabra o aplicativo\n2. Verifique sua conexão com a internet\n3. Reinicie o dispositivo se necessário\n4. Entre em contato com o suporte se o problema persistir',
    },
    {
      title: 'Posso votar offline?',
      content: 'Não. O FORTIS requer conexão com a internet para:\n• Verificar sua identidade\n• Sincronizar com a blockchain\n• Garantir a integridade do voto\n• Registrar o voto no sistema',
    },
  ];

  const contactInfo = [
    {
      icon: 'phone',
      title: 'Telefone',
      value: '0800 123 4567',
      description: 'Atendimento 24h',
    },
    {
      icon: 'email',
      title: 'Email',
      value: 'suporte@fortis.gov.br',
      description: 'Resposta em até 2h',
    },
    {
      icon: 'web',
      title: 'Site',
      value: 'www.fortis.gov.br',
      description: 'Central de ajuda online',
    },
    {
      icon: 'chat',
      title: 'Chat',
      value: 'Disponível no app',
      description: 'Suporte em tempo real',
    },
  ];

  const renderFAQItem = (item: any, index: number) => (
    <Accordion
      key={index}
      title={item.title}
      left={() => <Icon name="help" size={24} color={colors.primary} />}
      expanded={expandedItems.includes(index)}
      onPress={() => handleAccordionPress(index)}>
      <View style={styles.accordionContent}>
        <Text style={styles.accordionText}>{item.content}</Text>
      </View>
    </Accordion>
  );

  return (
    <ScrollView style={styles.container} showsVerticalScrollIndicator={false}>
      {/* Header */}
      <View style={styles.header}>
        <Title style={styles.headerTitle}>Central de Ajuda</Title>
        <Paragraph style={styles.headerSubtitle}>
          Encontre respostas para suas dúvidas
        </Paragraph>
      </View>

      {/* FAQ */}
      <Card style={[styles.card, shadows.medium]}>
        <Card.Content style={styles.cardContent}>
          <Title style={styles.cardTitle}>Perguntas Frequentes</Title>
          <Paragraph style={styles.cardSubtitle}>
            Clique nas perguntas para ver as respostas
          </Paragraph>
        </Card.Content>
      </Card>

      <View style={styles.accordionContainer}>
        {faqData.map((item, index) => renderFAQItem(item, index))}
      </View>

      {/* Contato */}
      <Card style={[styles.card, shadows.medium]}>
        <Card.Content style={styles.cardContent}>
          <Title style={styles.cardTitle}>Contato e Suporte</Title>
          <Paragraph style={styles.cardSubtitle}>
            Entre em contato conosco para obter ajuda
          </Paragraph>
        </Card.Content>
      </Card>

      <View style={styles.contactContainer}>
        {contactInfo.map((contact, index) => (
          <Card key={index} style={[styles.contactCard, shadows.small]}>
            <Card.Content style={styles.contactContent}>
              <View style={styles.contactItem}>
                <Icon name={contact.icon} size={24} color={colors.primary} />
                <View style={styles.contactDetails}>
                  <Text style={styles.contactTitle}>{contact.title}</Text>
                  <Text style={styles.contactValue}>{contact.value}</Text>
                  <Text style={styles.contactDescription}>{contact.description}</Text>
                </View>
              </View>
            </Card.Content>
          </Card>
        ))}
      </View>

      {/* Informações Técnicas */}
      <Card style={[styles.card, shadows.medium]}>
        <Card.Content style={styles.cardContent}>
          <Title style={styles.cardTitle}>Informações Técnicas</Title>
          
          <View style={styles.techInfo}>
            <View style={styles.techItem}>
              <Icon name="security" size={20} color={colors.success} />
              <Text style={styles.techText}>Criptografia AES-256</Text>
            </View>
            
            <View style={styles.techItem}>
              <Icon name="blockchain" size={20} color={colors.success} />
              <Text style={styles.techText}>Blockchain Ethereum</Text>
            </View>
            
            <View style={styles.techItem}>
              <Icon name="fingerprint" size={20} color={colors.success} />
              <Text style={styles.techText}>Autenticação Biométrica</Text>
            </View>
            
            <View style={styles.techItem}>
              <Icon name="verified" size={20} color={colors.success} />
              <Text style={styles.techText}>Zero-Knowledge Proofs</Text>
            </View>
          </View>
        </Card.Content>
      </Card>

      {/* Botões de Ação */}
      <View style={styles.actionContainer}>
        <Button
          mode="contained"
          style={styles.actionButton}
          contentStyle={styles.buttonContent}
          icon="chat"
          onPress={() => {/* Implementar chat */}}>
          Iniciar Chat
        </Button>
        
        <Button
          mode="outlined"
          style={styles.actionButton}
          contentStyle={styles.buttonContent}
          icon="email"
          onPress={() => {/* Implementar email */}}>
          Enviar Email
        </Button>
      </View>

      {/* Rodapé */}
      <View style={styles.footer}>
        <Text style={styles.footerText}>
          FORTIS - Sistema de Votação Eletrônica Seguro
        </Text>
        <Text style={styles.footerSubtext}>
          Versão 1.0.0 - Desenvolvido com tecnologia de ponta
        </Text>
      </View>
    </ScrollView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  header: {
    padding: spacing.lg,
    paddingBottom: spacing.md,
  },
  headerTitle: {
    fontSize: 24,
    fontWeight: 'bold',
    color: colors.onSurface,
    marginBottom: spacing.sm,
  },
  headerSubtitle: {
    fontSize: 16,
    color: colors.onSurface,
    opacity: 0.7,
  },
  card: {
    marginHorizontal: spacing.lg,
    marginBottom: spacing.md,
    backgroundColor: colors.surface,
  },
  cardContent: {
    padding: spacing.lg,
  },
  cardTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: colors.onSurface,
    marginBottom: spacing.sm,
  },
  cardSubtitle: {
    fontSize: 14,
    color: colors.onSurface,
    opacity: 0.7,
  },
  accordionContainer: {
    marginHorizontal: spacing.lg,
    marginBottom: spacing.md,
  },
  accordionContent: {
    padding: spacing.md,
    backgroundColor: colors.background,
  },
  accordionText: {
    fontSize: 14,
    color: colors.onSurface,
    lineHeight: 20,
  },
  contactContainer: {
    marginHorizontal: spacing.lg,
    marginBottom: spacing.md,
  },
  contactCard: {
    marginBottom: spacing.sm,
    backgroundColor: colors.surface,
  },
  contactContent: {
    padding: spacing.md,
  },
  contactItem: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  contactDetails: {
    marginLeft: spacing.md,
    flex: 1,
  },
  contactTitle: {
    fontSize: 16,
    fontWeight: 'bold',
    color: colors.onSurface,
  },
  contactValue: {
    fontSize: 14,
    color: colors.primary,
    marginVertical: spacing.xs,
  },
  contactDescription: {
    fontSize: 12,
    color: colors.onSurface,
    opacity: 0.7,
  },
  techInfo: {
    marginTop: spacing.md,
  },
  techItem: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: spacing.sm,
  },
  techText: {
    marginLeft: spacing.sm,
    fontSize: 14,
    color: colors.onSurface,
  },
  actionContainer: {
    padding: spacing.lg,
  },
  actionButton: {
    marginBottom: spacing.md,
  },
  buttonContent: {
    paddingVertical: spacing.sm,
  },
  footer: {
    alignItems: 'center',
    padding: spacing.lg,
    paddingTop: spacing.md,
  },
  footerText: {
    fontSize: 14,
    color: colors.onSurface,
    textAlign: 'center',
    opacity: 0.7,
  },
  footerSubtext: {
    fontSize: 12,
    color: colors.onSurface,
    textAlign: 'center',
    marginTop: spacing.sm,
    opacity: 0.5,
  },
});

export default HelpScreen;
