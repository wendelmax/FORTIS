import React, {useState, useEffect} from 'react';
import {
  View,
  Text,
  StyleSheet,
  Alert,
  Animated,
  Dimensions,
} from 'react-native';
import {
  Card,
  Title,
  Paragraph,
  Button,
  Chip,
  ActivityIndicator,
} from 'react-native-paper';
import Icon from 'react-native-vector-icons/MaterialIcons';
import LinearGradient from 'react-native-linear-gradient';
import {colors, spacing, shadows} from '../styles/theme';

const {width} = Dimensions.get('window');

interface VoteConfirmationScreenProps {
  route: {
    params: {
      electionId: string;
      candidateId: string;
      candidateName: string;
      candidateNumber: string;
    };
  };
  navigation: any;
}

const VoteConfirmationScreen: React.FC<VoteConfirmationScreenProps> = ({
  route,
  navigation,
}) => {
  const [isProcessing, setIsProcessing] = useState(false);
  const [voteId, setVoteId] = useState<string | null>(null);
  const [receiptHash, setReceiptHash] = useState<string | null>(null);
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

    // Simular processamento do voto
    processVote();
  }, []);

  const processVote = async () => {
    setIsProcessing(true);
    
    // Simular delay de processamento
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    // Simular geração de ID e hash
    const generatedVoteId = `VOTE_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    const generatedReceiptHash = `HASH_${Date.now()}_${Math.random().toString(36).substr(2, 16)}`;
    
    setVoteId(generatedVoteId);
    setReceiptHash(generatedReceiptHash);
    setIsProcessing(false);
  };

  const handleConfirm = () => {
    if (!voteId || !receiptHash) {
      Alert.alert('Erro', 'Voto ainda está sendo processado');
      return;
    }

    navigation.navigate('VoteReceipt', {
      voteId,
      electionId: route.params.electionId,
      candidateName: route.params.candidateName,
      candidateNumber: route.params.candidateNumber,
      timestamp: new Date().toISOString(),
      receiptHash,
    });
  };

  const handleCancel = () => {
    Alert.alert(
      'Cancelar Voto',
      'Tem certeza que deseja cancelar este voto?',
      [
        {text: 'Não', style: 'cancel'},
        {text: 'Sim', onPress: () => navigation.goBack()},
      ]
    );
  };

  return (
    <LinearGradient
      colors={[colors.primary, colors.primaryVariant]}
      style={styles.container}>
      <View style={styles.content}>
        
        {/* Header */}
        <Animated.View
          style={[
            styles.header,
            {
              opacity: fadeAnim,
              transform: [{scale: scaleAnim}],
            },
          ]}>
          <Icon name="check-circle" size={80} color={colors.onPrimary} />
          <Title style={styles.title}>Voto Confirmado</Title>
          <Paragraph style={styles.subtitle}>
            Seu voto foi registrado com sucesso
          </Paragraph>
        </Animated.View>

        {/* Card de Confirmação */}
        <Animated.View
          style={[
            styles.cardContainer,
            {
              opacity: fadeAnim,
            },
          ]}>
          <Card style={[styles.confirmationCard, shadows.large]}>
            <Card.Content style={styles.cardContent}>
              <Title style={styles.cardTitle}>Detalhes do Voto</Title>
              
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
                <Text style={styles.detailLabel}>Horário:</Text>
                <Text style={styles.detailValue}>
                  {new Date().toLocaleString('pt-BR')}
                </Text>
              </View>

              {voteId && (
                <View style={styles.detailRow}>
                  <Icon name="fingerprint" size={20} color={colors.primary} />
                  <Text style={styles.detailLabel}>ID do Voto:</Text>
                  <Text style={styles.detailValue}>{voteId}</Text>
                </View>
              )}

              {receiptHash && (
                <View style={styles.detailRow}>
                  <Icon name="security" size={20} color={colors.primary} />
                  <Text style={styles.detailLabel}>Hash:</Text>
                  <Text style={styles.detailValue}>{receiptHash}</Text>
                </View>
              )}

              <View style={styles.statusContainer}>
                <Chip
                  mode="outlined"
                  textStyle={styles.statusText}
                  style={[styles.statusChip, {borderColor: colors.success}]}>
                  ✓ Voto Registrado
                </Chip>
              </View>
            </Card.Content>
          </Card>
        </Animated.View>

        {/* Processamento */}
        {isProcessing && (
          <Animated.View
            style={[
              styles.processingContainer,
              {
                opacity: fadeAnim,
              },
            ]}>
            <ActivityIndicator size="large" color={colors.onPrimary} />
            <Text style={styles.processingText}>
              Processando voto...
            </Text>
          </Animated.View>
        )}

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
            onPress={handleConfirm}
            disabled={isProcessing || !voteId}
            style={styles.confirmButton}
            contentStyle={styles.buttonContent}
            icon="receipt">
            Ver Comprovante
          </Button>
          
          <Button
            mode="outlined"
            onPress={handleCancel}
            disabled={isProcessing}
            style={styles.cancelButton}
            contentStyle={styles.buttonContent}
            icon="arrow-back">
            Voltar
          </Button>
        </Animated.View>

        {/* Informações de Segurança */}
        <Animated.View
          style={[
            styles.securityInfo,
            {
              opacity: fadeAnim,
            },
          ]}>
          <View style={styles.securityItem}>
            <Icon name="shield" size={16} color={colors.onPrimary} />
            <Text style={styles.securityText}>
              Seu voto é criptografado e auditável
            </Text>
          </View>
          
          <View style={styles.securityItem}>
            <Icon name="verified" size={16} color={colors.onPrimary} />
            <Text style={styles.securityText}>
              Integridade verificada por blockchain
            </Text>
          </View>
        </Animated.View>
      </View>
    </LinearGradient>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  content: {
    flex: 1,
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.xl,
    justifyContent: 'center',
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
    marginBottom: spacing.xl,
  },
  confirmationCard: {
    backgroundColor: colors.surface,
  },
  cardContent: {
    padding: spacing.lg,
  },
  cardTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: colors.onSurface,
    marginBottom: spacing.lg,
    textAlign: 'center',
  },
  detailRow: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: spacing.md,
    paddingVertical: spacing.sm,
  },
  detailLabel: {
    marginLeft: spacing.sm,
    fontSize: 16,
    color: colors.onSurface,
    fontWeight: '500',
    minWidth: 80,
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
    paddingTop: spacing.lg,
    borderTopWidth: 1,
    borderTopColor: colors.background,
  },
  statusChip: {
    backgroundColor: colors.success + '20',
  },
  statusText: {
    color: colors.success,
    fontWeight: 'bold',
  },
  processingContainer: {
    alignItems: 'center',
    marginBottom: spacing.xl,
  },
  processingText: {
    marginTop: spacing.md,
    fontSize: 16,
    color: colors.onPrimary,
    textAlign: 'center',
  },
  buttonContainer: {
    marginBottom: spacing.lg,
  },
  confirmButton: {
    marginBottom: spacing.md,
  },
  cancelButton: {
    borderColor: colors.onPrimary,
  },
  buttonContent: {
    paddingVertical: spacing.sm,
  },
  securityInfo: {
    alignItems: 'center',
  },
  securityItem: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: spacing.sm,
  },
  securityText: {
    marginLeft: spacing.sm,
    fontSize: 14,
    color: colors.onPrimary,
    opacity: 0.8,
  },
});

export default VoteConfirmationScreen;
