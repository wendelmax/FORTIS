import React, {useState, useEffect} from 'react';
import {
  View,
  Text,
  StyleSheet,
  FlatList,
  RefreshControl,
  Alert,
} from 'react-native';
import {
  Card,
  Title,
  Paragraph,
  Button,
  Chip,
  ActivityIndicator,
  FAB,
} from 'react-native-paper';
import Icon from 'react-native-vector-icons/MaterialIcons';
import {useVoting} from '../contexts/VotingContext';
import {useAuth} from '../contexts/AuthContext';
import {Election} from '../types';
import {colors, spacing, shadows} from '../styles/theme';

const ElectionListScreen: React.FC = () => {
  const [refreshing, setRefreshing] = useState(false);
  const [selectedElection, setSelectedElection] = useState<Election | null>(null);

  const {state: votingState, loadElections, selectElection, getActiveElections} = useVoting();
  const {state: authState} = useAuth();

  useEffect(() => {
    loadElections();
  }, []);

  const handleRefresh = async () => {
    setRefreshing(true);
    await loadElections();
    setRefreshing(false);
  };

  const handleElectionSelect = (election: Election) => {
    if (election.isVoted) {
      Alert.alert(
        'Voto Já Registrado',
        'Você já votou nesta eleição.',
        [{text: 'OK'}]
      );
      return;
    }

    if (!election.isActive) {
      Alert.alert(
        'Eleição Inativa',
        'Esta eleição não está ativa no momento.',
        [{text: 'OK'}]
      );
      return;
    }

    selectElection(election);
    // Navegar para tela de votação
    // navigation.navigate('Voting', { electionId: election.id, electionTitle: election.title });
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('pt-BR', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const getElectionStatus = (election: Election) => {
    if (election.isVoted) return 'Votado';
    if (election.isActive) return 'Ativa';
    return 'Inativa';
  };

  const getElectionStatusColor = (election: Election) => {
    if (election.isVoted) return colors.success;
    if (election.isActive) return colors.primary;
    return colors.error;
  };

  const getElectionTypeIcon = (type: string) => {
    switch (type) {
      case 'presidential':
        return 'account-star';
      case 'congressional':
        return 'account-group';
      case 'state':
        return 'map-marker';
      case 'municipal':
        return 'city';
      default:
        return 'vote';
    }
  };

  const getElectionTypeText = (type: string) => {
    switch (type) {
      case 'presidential':
        return 'Presidencial';
      case 'congressional':
        return 'Congresso';
      case 'state':
        return 'Estadual';
      case 'municipal':
        return 'Municipal';
      default:
        return 'Eleição';
    }
  };

  const renderElectionCard = ({item}: {item: Election}) => (
    <Card
      style={[
        styles.electionCard,
        shadows.medium,
        item.isVoted && styles.votedCard,
      ]}
      onPress={() => handleElectionSelect(item)}>
      <Card.Content style={styles.cardContent}>
        <View style={styles.cardHeader}>
          <View style={styles.electionInfo}>
            <Title style={styles.electionTitle}>{item.title}</Title>
            <Paragraph style={styles.electionDescription}>
              {item.description}
            </Paragraph>
          </View>
          <Icon
            name={getElectionTypeIcon(item.type)}
            size={32}
            color={colors.primary}
          />
        </View>

        <View style={styles.cardDetails}>
          <View style={styles.detailRow}>
            <Icon name="calendar" size={16} color={colors.onSurface} />
            <Text style={styles.detailText}>
              {formatDate(item.startDate)} - {formatDate(item.endDate)}
            </Text>
          </View>

          <View style={styles.detailRow}>
            <Icon name="account-group" size={16} color={colors.onSurface} />
            <Text style={styles.detailText}>
              {item.totalCandidates} candidatos
            </Text>
          </View>

          <View style={styles.detailRow}>
            <Icon name="vote" size={16} color={colors.onSurface} />
            <Text style={styles.detailText}>
              {item.totalVotes} votos registrados
            </Text>
          </View>
        </View>

        <View style={styles.cardFooter}>
          <Chip
            mode="outlined"
            textStyle={styles.chipText}
            style={[
              styles.statusChip,
              {borderColor: getElectionStatusColor(item)},
            ]}>
            {getElectionStatus(item)}
          </Chip>

          <Chip
            mode="outlined"
            textStyle={styles.chipText}
            style={styles.typeChip}>
            {getElectionTypeText(item.type)}
          </Chip>
        </View>

        {item.isVoted && (
          <View style={styles.votedIndicator}>
            <Icon name="check-circle" size={20} color={colors.success} />
            <Text style={styles.votedText}>Voto registrado</Text>
          </View>
        )}
      </Card.Content>
    </Card>
  );

  const renderEmptyState = () => (
    <View style={styles.emptyState}>
      <Icon name="ballot" size={64} color={colors.onSurface} />
      <Title style={styles.emptyTitle}>Nenhuma Eleição Disponível</Title>
      <Paragraph style={styles.emptyDescription}>
        Não há eleições ativas no momento. Verifique novamente mais tarde.
      </Paragraph>
    </View>
  );

  const renderHeader = () => (
    <View style={styles.header}>
      <Title style={styles.headerTitle}>Eleições Disponíveis</Title>
      <Paragraph style={styles.headerSubtitle}>
        Selecione uma eleição para votar
      </Paragraph>
    </View>
  );

  if (votingState.isLoading) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color={colors.primary} />
        <Text style={styles.loadingText}>Carregando eleições...</Text>
      </View>
    );
  }

  const activeElections = getActiveElections();

  return (
    <View style={styles.container}>
      <FlatList
        data={activeElections}
        renderItem={renderElectionCard}
        keyExtractor={(item) => item.id}
        ListHeaderComponent={renderHeader}
        ListEmptyComponent={renderEmptyState}
        refreshControl={
          <RefreshControl
            refreshing={refreshing}
            onRefresh={handleRefresh}
            colors={[colors.primary]}
            tintColor={colors.primary}
          />
        }
        contentContainerStyle={styles.listContent}
        showsVerticalScrollIndicator={false}
      />

      {/* FAB para atualizar */}
      <FAB
        icon="refresh"
        style={styles.fab}
        onPress={handleRefresh}
        loading={refreshing}
      />
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
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
  headerSubtitle: {
    fontSize: 16,
    color: colors.onSurface,
    opacity: 0.7,
  },
  electionCard: {
    marginBottom: spacing.md,
    backgroundColor: colors.surface,
  },
  votedCard: {
    backgroundColor: colors.success + '10',
    borderColor: colors.success,
    borderWidth: 1,
  },
  cardContent: {
    padding: spacing.lg,
  },
  cardHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    marginBottom: spacing.md,
  },
  electionInfo: {
    flex: 1,
    marginRight: spacing.sm,
  },
  electionTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: colors.onSurface,
    marginBottom: spacing.xs,
  },
  electionDescription: {
    fontSize: 14,
    color: colors.onSurface,
    opacity: 0.7,
  },
  cardDetails: {
    marginBottom: spacing.md,
  },
  detailRow: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: spacing.xs,
  },
  detailText: {
    marginLeft: spacing.sm,
    fontSize: 14,
    color: colors.onSurface,
    opacity: 0.8,
  },
  cardFooter: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  statusChip: {
    backgroundColor: 'transparent',
  },
  typeChip: {
    backgroundColor: 'transparent',
  },
  chipText: {
    fontSize: 12,
  },
  votedIndicator: {
    flexDirection: 'row',
    alignItems: 'center',
    marginTop: spacing.sm,
    paddingTop: spacing.sm,
    borderTopWidth: 1,
    borderTopColor: colors.success + '30',
  },
  votedText: {
    marginLeft: spacing.sm,
    fontSize: 14,
    color: colors.success,
    fontWeight: '500',
  },
  emptyState: {
    alignItems: 'center',
    paddingVertical: spacing.xxl,
  },
  emptyTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: colors.onSurface,
    marginTop: spacing.lg,
    marginBottom: spacing.sm,
  },
  emptyDescription: {
    fontSize: 16,
    color: colors.onSurface,
    opacity: 0.7,
    textAlign: 'center',
    paddingHorizontal: spacing.lg,
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
  fab: {
    position: 'absolute',
    margin: spacing.md,
    right: 0,
    bottom: 0,
    backgroundColor: colors.primary,
  },
});

export default ElectionListScreen;
