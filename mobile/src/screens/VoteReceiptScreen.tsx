import React, {useState, useEffect} from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  Alert,
  Animated,
  Share,
} from 'react-native';
import {
  Card,
  Title,
  Paragraph,
  Button,
  Chip,
  Divider,
} from 'react-native-paper';
import Icon from 'react-native-vector-icons/MaterialIcons';
import LinearGradient from 'react-native-linear-gradient';
import {colors, spacing, shadows} from '../styles/theme';

interface VoteReceiptScreenProps {
  route: {
    params: {
      voteId: string;
      electionId: string;
      candidateName: string;
      candidateNumber: string;
      timestamp: string;
      receiptHash: string;
    };
  };
  navigation: any;
}

const VoteReceiptScreen: React.FC<VoteReceiptScreenProps> = ({
  route,
  navigation,
}) => {
  const [fadeAnim] = useState(new Animated.Value(0));
  const [scaleAnim] = useState(new Animated.Value(0.8));

  useEffect(() => {
    // Animação de entrada
    Animated.parallel([
      Animated.timing(fadeAnim, {
        toValue: 1,
        duration: 800,
        useNativeDriver: true,
      }),
      Animated.spring(scaleAnim, {
        toValue: 1,
        tension: 50,
        friction: 7,
        useNativeDriver: true,
      }),
    ]).start();
  }, []);

  const handleShare = async () => {
    try {
      const message = `Comprovante de Votação FORTIS\n\n` +
        `Candidato: ${route.params.candidateName}\n` +
        `Número: ${route.params.candidateNumber}\n` +
        `ID do Voto: ${route.params.voteId}\n` +
        `Hash: ${route.params.receiptHash}\n` +
        `Data/Hora: ${route.params.timestamp}\n\n` +
        `Este é um comprovante oficial de votação eletrônica.`;

      await Share.share({
        message,
        title: 'Comprovante de Votação FORTIS',
      });
    } catch (error) {
      console.error('Erro ao compartilhar:', error);
      Alert.alert('Erro', 'Não foi possível compartilhar o comprovante');
    }
  };

  const handleFinish = () => {
    Alert.alert(
      'Votação Concluída',
      'Sua votação foi registrada com sucesso. Obrigado por participar!',
      [
        {
          text: 'OK',
          onPress: () => navigation.navigate('ElectionList'),
        },
      ]
    );
  };

  const formatTimestamp = (timestamp: string) => {
    const date = new Date(timestamp);
    return date.toLocaleString('pt-BR', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  };

  return (
    <LinearGradient
      colors={[colors.primary, colors.primaryVariant]}
      style={styles.container}>
      <ScrollView
        contentContainerStyle={styles.scrollContent}
        showsVerticalScrollIndicator={false}>
        
        {/* Header */}
        <Animated.View
          style={[
            styles.header,
            {
              opacity: fadeAnim,
              transform: [{scale: scaleAnim}],
            },
          ]}>
          <Icon name="receipt" size={80} color={colors.onPrimary} />
          <Title style={styles.title}>Comprovante de Votação</Title>
          <Paragraph style={styles.subtitle}>
            Seu voto foi registrado com sucesso
          </Paragraph>
        </Animated.View>

        {/* Card Principal */}
        <Animated.View
          style={[
            styles.cardContainer,
            {
              opacity: fadeAnim,
            },
          ]}>
          <Card style={[styles.receiptCard, shadows.large]}>
            <Card.Content style={styles.cardContent}>
              <Title style={styles.cardTitle}>FORTIS - Sistema de Votação Eletrônica</Title>
              <Paragraph style={styles.cardSubtitle}>
                Comprovante Oficial de Votação
              </Paragraph>
              
              <Divider style={styles.divider} />
              
              {/* Informações do Voto */}
              <View style={styles.section}>
                <Text style={styles.sectionTitle}>Informações do Voto</Text>
                
                <View style={styles.detailRow}>
                  <Icon name="person" size={20} color={colors.primary} />
                  <Text style={styles.detailLabel}>Candidato:</Text>
                  <Text style={styles.detailValue}>{route.params.candidateName}</Text>
                </View>
                
                <View style={styles.detailRow}>
                  <Icon name="tag" size={20} color={colors.primary} />
                  <Text style={styles.detailLabel}>Número:</Text>
                  <Text style={styles.detailValue}>{route.params.candidateNumber}</Text>
                </View>
                
                <View style={styles.detailRow}>
                  <Icon name="schedule" size={20} color={colors.primary} />
                  <Text style={styles.detailLabel}>Data/Hora:</Text>
                  <Text style={styles.detailValue}>
                    {formatTimestamp(route.params.timestamp)}
                  </Text>
                </View>
              </View>
              
              <Divider style={styles.divider} />
              
              {/* Informações Técnicas */}
              <View style={styles.section}>
                <Text style={styles.sectionTitle}>Informações Técnicas</Text>
                
                <View style={styles.detailRow}>
                  <Icon name="fingerprint" size={20} color={colors.primary} />
                  <Text style={styles.detailLabel}>ID do Voto:</Text>
                  <Text style={styles.detailValue}>{route.params.voteId}</Text>
                </View>
                
                <View style={styles.detailRow}>
                  <Icon name="security" size={20} color={colors.primary} />
                  <Text style={styles.detailLabel}>Hash:</Text>
                  <Text style={styles.detailValue}>{route.params.receiptHash}</Text>
                </View>
                
                <View style={styles.detailRow}>
                  <Icon name="ballot" size={20} color={colors.primary} />
                  <Text style={styles.detailLabel}>Eleição:</Text>
                  <Text style={styles.detailValue}>{route.params.electionId}</Text>
                </View>
              </View>
              
              <Divider style={styles.divider} />
              
              {/* Status */}
              <View style={styles.statusContainer}>
                <Chip
                  mode="outlined"
                  textStyle={styles.statusText}
                  style={[styles.statusChip, {borderColor: colors.success}]}>
                  ✓ Voto Registrado e Verificado
                </Chip>
              </View>
            </Card.Content>
          </Card>
        </Animated.View>

        {/* Card de Segurança */}
        <Animated.View
          style={[
            styles.securityCardContainer,
            {
              opacity: fadeAnim,
            },
          ]}>
          <Card style={[styles.securityCard, shadows.medium]}>
            <Card.Content style={styles.securityContent}>
              <Title style={styles.securityTitle}>Garantias de Segurança</Title>
              
              <View style={styles.securityItem}>
                <Icon name="shield" size={20} color={colors.success} />
                <Text style={styles.securityText}>
                  Voto criptografado com tecnologia de ponta
                </Text>
              </View>
              
              <View style={styles.securityItem}>
                <Icon name="verified" size={20} color={colors.success} />
                <Text style={styles.securityText}>
                  Integridade verificada por blockchain
                </Text>
              </View>
              
              <View style={styles.securityItem}>
                <Icon name="audit" size={20} color={colors.success} />
                <Text style={styles.securityText}>
                  Processo totalmente auditável
                </Text>
              </View>
              
              <View style={styles.securityItem}>
                <Icon name="privacy" size={20} color={colors.success} />
                <Text style={styles.securityText}>
                  Privacidade garantida por Zero-Knowledge Proofs
                </Text>
              </View>
            </Card.Content>
          </Card>
        </Animated.View>

        {/* Botões */}
        <Animated.View
          style={[
            styles.buttonContainer,
            {
              opacity: fadeAnim,
            },
          ]}>
          <Button
            mode="contained"
            onPress={handleShare}
            style={styles.shareButton}
            contentStyle={styles.buttonContent}
            icon="share">
            Compartilhar Comprovante
          </Button>
          
          <Button
            mode="outlined"
            onPress={handleFinish}
            style={styles.finishButton}
            contentStyle={styles.buttonContent}
            icon="check">
            Concluir
          </Button>
        </Animated.View>

        {/* Rodapé */}
        <Animated.View
          style={[
            styles.footer,
            {
              opacity: fadeAnim,
            },
          ]}>
          <Text style={styles.footerText}>
            Este comprovante é um documento oficial do sistema FORTIS
          </Text>
          <Text style={styles.footerSubtext}>
            Guarde este comprovante para sua referência
          </Text>
        </Animated.View>
      </ScrollView>
    </LinearGradient>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollContent: {
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.xl,
  },
  header: {
    alignItems: 'center',
    marginBottom: spacing.xl,
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: colors.onPrimary,
    marginTop: spacing.md,
    textAlign: 'center',
  },
  subtitle: {
    fontSize: 16,
    color: colors.onPrimary,
    textAlign: 'center',
    marginTop: spacing.sm,
    opacity: 0.9,
  },
  cardContainer: {
    marginBottom: spacing.lg,
  },
  receiptCard: {
    backgroundColor: colors.surface,
  },
  cardContent: {
    padding: spacing.lg,
  },
  cardTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: colors.onSurface,
    textAlign: 'center',
    marginBottom: spacing.sm,
  },
  cardSubtitle: {
    fontSize: 16,
    color: colors.onSurface,
    textAlign: 'center',
    marginBottom: spacing.lg,
    opacity: 0.7,
  },
  divider: {
    marginVertical: spacing.md,
  },
  section: {
    marginBottom: spacing.md,
  },
  sectionTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: colors.onSurface,
    marginBottom: spacing.md,
  },
  detailRow: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: spacing.sm,
    paddingVertical: spacing.xs,
  },
  detailLabel: {
    marginLeft: spacing.sm,
    fontSize: 16,
    color: colors.onSurface,
    fontWeight: '500',
    minWidth: 100,
  },
  detailValue: {
    flex: 1,
    fontSize: 16,
    color: colors.onSurface,
    marginLeft: spacing.sm,
  },
  statusContainer: {
    alignItems: 'center',
    marginTop: spacing.lg,
  },
  statusChip: {
    backgroundColor: colors.success + '20',
  },
  statusText: {
    color: colors.success,
    fontWeight: 'bold',
  },
  securityCardContainer: {
    marginBottom: spacing.lg,
  },
  securityCard: {
    backgroundColor: colors.surface,
  },
  securityContent: {
    padding: spacing.lg,
  },
  securityTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: colors.onSurface,
    marginBottom: spacing.md,
    textAlign: 'center',
  },
  securityItem: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    marginBottom: spacing.sm,
  },
  securityText: {
    marginLeft: spacing.sm,
    fontSize: 14,
    color: colors.onSurface,
    flex: 1,
  },
  buttonContainer: {
    marginBottom: spacing.lg,
  },
  shareButton: {
    marginBottom: spacing.md,
  },
  finishButton: {
    borderColor: colors.onPrimary,
  },
  buttonContent: {
    paddingVertical: spacing.sm,
  },
  footer: {
    alignItems: 'center',
  },
  footerText: {
    fontSize: 14,
    color: colors.onPrimary,
    textAlign: 'center',
    opacity: 0.8,
  },
  footerSubtext: {
    fontSize: 12,
    color: colors.onPrimary,
    textAlign: 'center',
    marginTop: spacing.sm,
    opacity: 0.6,
  },
});

export default VoteReceiptScreen;
