import React, {useState, useEffect} from 'react';
import {
  View,
  Text,
  StyleSheet,
  FlatList,
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
  ProgressBar,
} from 'react-native-paper';
import Icon from 'react-native-vector-icons/MaterialIcons';
import LinearGradient from 'react-native-linear-gradient';
import {useVoting} from '../contexts/VotingContext';
import {useAuth} from '../contexts/AuthContext';
import {Candidate, VoteRequest, BiometricData} from '../types';
import {colors, spacing, shadows} from '../styles/theme';

const {width} = Dimensions.get('window');

interface VotingScreenProps {
  route: {
    params: {
      electionId: string;
      electionTitle: string;
    };
  };
  navigation: any;
}

const VotingScreen: React.FC<VotingScreenProps> = ({route, navigation}) => {
  const [selectedCandidate, setSelectedCandidate] = useState<Candidate | null>(null);
  const [isVoting, setIsVoting] = useState(false);
  const [timeRemaining, setTimeRemaining] = useState(300); // 5 minutos
  const [progressAnim] = useState(new Animated.Value(1));

  const {state: votingState, castVote, getElectionById} = useVoting();
  const {state: authState} = useAuth();

  const election = getElectionById(route.params.electionId);

  useEffect(() => {
    if (!election) {
      Alert.alert('Erro', 'Eleição não encontrada');
      navigation.goBack();
      return;
    }

    // Timer de votação
    const timer = setInterval(() => {
      setTimeRemaining((prev) => {
        if (prev <= 1) {
          Alert.alert(
            'Tempo Esgotado',
            'O tempo para votação expirou. Você será redirecionado.',
            [{text: 'OK', onPress: () => navigation.goBack()}]
          );
          return 0;
        }
        return prev - 1;
      });
    }, 1000);

    return () => clearInterval(timer);
  }, [election, navigation]);

  useEffect(() => {
    // Animação de progresso
    Animated.timing(progressAnim, {
      toValue: timeRemaining / 300,
      duration: 1000,
      useNativeDriver: false,
    }).start();
  }, [timeRemaining]);

  const handleCandidateSelect = (candidate: Candidate) => {
    setSelectedCandidate(candidate);
  };

  const handleVote = async () => {
    if (!selectedCandidate) {
      Alert.alert('Erro', 'Selecione um candidato para votar');
      return;
    }

    if (!election) {
      Alert.alert('Erro', 'Eleição não encontrada');
      return;
    }

    Alert.alert(
      'Confirmar Voto',
      `Tem certeza que deseja votar em ${selectedCandidate.name} (${selectedCandidate.number})?`,
      [
        {text: 'Cancelar', style: 'cancel'},
        {text: 'Confirmar', onPress: confirmVote},
      ]
    );
  };

  const confirmVote = async () => {
    if (!selectedCandidate || !election) return;

    setIsVoting(true);
    try {
      // Simular dados biométricos (em produção, usar biometria real)
      const biometricData: BiometricData = {
        type: 'fingerprint',
        data: 'simulated_biometric_data',
        timestamp: new Date().toISOString(),
      };

      const voteRequest: VoteRequest = {
        electionId: election.id,
        candidateId: selectedCandidate.id,
        biometricData,
        deviceInfo: {
          deviceId: 'mobile_device',
          model: 'Mobile Device',
          os: 'Mobile OS',
          version: '1.0.0',
          isJailbroken: false,
          isRooted: false,
          hasSecurityPatch: true,
          timestamp: new Date().toISOString(),
        },
      };

      const response = await castVote(voteRequest);
      
      // Navegar para tela de confirmação
      navigation.navigate('VoteConfirmation', {
        electionId: election.id,
        candidateId: selectedCandidate.id,
        candidateName: selectedCandidate.name,
        candidateNumber: selectedCandidate.number,
      });
    } catch (error) {
      Alert.alert(
        'Erro no Voto',
        error instanceof Error ? error.message : 'Erro desconhecido'
      );
    } finally {
      setIsVoting(false);
    }
  };

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const renderCandidate = ({item}: {item: Candidate}) => (
    <Card
      style={[
        styles.candidateCard,
        shadows.medium,
        selectedCandidate?.id === item.id && styles.selectedCard,
      ]}
      onPress={() => handleCandidateSelect(item)}>
      <Card.Content style={styles.candidateContent}>
        <View style={styles.candidateHeader}>
          <View style={styles.candidateNumber}>
            <Text style={styles.candidateNumberText}>{item.number}</Text>
          </View>
          <View style={styles.candidateInfo}>
            <Title style={styles.candidateName}>{item.name}</Title>
            <Paragraph style={styles.candidateParty}>
              {item.party} {item.coalition && `- ${item.coalition}`}
            </Paragraph>
          </View>
          {selectedCandidate?.id === item.id && (
            <Icon name="check-circle" size={24} color={colors.primary} />
          )}
        </View>
      </Card.Content>
    </Card>
  );

  const renderHeader = () => (
    <View style={styles.header}>
      <Title style={styles.headerTitle}>{election?.title}</Title>
      <Paragraph style={styles.headerDescription}>
        {election?.description}
      </Paragraph>
      
      {/* Timer */}
      <View style={styles.timerContainer}>
        <Icon name="timer" size={20} color={colors.primary} />
        <Text style={styles.timerText}>
          Tempo restante: {formatTime(timeRemaining)}
        </Text>
      </View>
      
      {/* Progress Bar */}
      <Animated.View style={styles.progressContainer}>
        <ProgressBar
          progress={progressAnim}
          color={colors.primary}
          style={styles.progressBar}
        />
      </Animated.View>
    </View>
  );

  const renderFooter = () => (
    <View style={styles.footer}>
      <Button
        mode="contained"
        onPress={handleVote}
        loading={isVoting}
        disabled={!selectedCandidate || isVoting}
        style={styles.voteButton}
        contentStyle={styles.buttonContent}
        icon="vote">
        {isVoting ? 'Registrando Voto...' : 'Confirmar Voto'}
      </Button>
      
      <Button
        mode="outlined"
        onPress={() => navigation.goBack()}
        disabled={isVoting}
        style={styles.cancelButton}
        contentStyle={styles.buttonContent}
        icon="arrow-back">
        Cancelar
      </Button>
    </View>
  );

  if (!election) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color={colors.primary} />
        <Text style={styles.loadingText}>Carregando eleição...</Text>
      </View>
    );
  }

  return (
    <LinearGradient
      colors={[colors.background, colors.surface]}
      style={styles.container}>
      <View style={styles.content}>
        <FlatList
          data={election.candidates}
          renderItem={renderCandidate}
          keyExtractor={(item) => item.id}
          ListHeaderComponent={renderHeader}
          contentContainerStyle={styles.listContent}
          showsVerticalScrollIndicator={false}
        />
      </View>
      
      {renderFooter()}
    </LinearGradient>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  content: {
    flex: 1,
  },
  listContent: {
    padding: spacing.lg,
  },
  header: {
    marginBottom: spacing.lg,
  },
  headerTitle: {
    fontSize: 24,
    fontWeight: 'bold',
    color: colors.onSurface,
    marginBottom: spacing.sm,
  },
  headerDescription: {
    fontSize: 16,
    color: colors.onSurface,
    opacity: 0.7,
    marginBottom: spacing.lg,
  },
  timerContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: spacing.sm,
  },
  timerText: {
    marginLeft: spacing.sm,
    fontSize: 16,
    color: colors.primary,
    fontWeight: '500',
  },
  progressContainer: {
    marginBottom: spacing.lg,
  },
  progressBar: {
    height: 8,
    borderRadius: 4,
  },
  candidateCard: {
    marginBottom: spacing.md,
    backgroundColor: colors.surface,
  },
  selectedCard: {
    borderColor: colors.primary,
    borderWidth: 2,
    backgroundColor: colors.primary + '10',
  },
  candidateContent: {
    padding: spacing.lg,
  },
  candidateHeader: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  candidateNumber: {
    width: 50,
    height: 50,
    borderRadius: 25,
    backgroundColor: colors.primary,
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: spacing.md,
  },
  candidateNumberText: {
    fontSize: 18,
    fontWeight: 'bold',
    color: colors.onPrimary,
  },
  candidateInfo: {
    flex: 1,
  },
  candidateName: {
    fontSize: 18,
    fontWeight: 'bold',
    color: colors.onSurface,
    marginBottom: spacing.xs,
  },
  candidateParty: {
    fontSize: 14,
    color: colors.onSurface,
    opacity: 0.7,
  },
  footer: {
    padding: spacing.lg,
    backgroundColor: colors.surface,
    borderTopWidth: 1,
    borderTopColor: colors.background,
  },
  voteButton: {
    marginBottom: spacing.md,
  },
  cancelButton: {
    borderColor: colors.error,
  },
  buttonContent: {
    paddingVertical: spacing.sm,
  },
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  loadingText: {
    marginTop: spacing.md,
    fontSize: 16,
    color: colors.onSurface,
  },
});

export default VotingScreen;
