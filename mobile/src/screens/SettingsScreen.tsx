import React, {useState} from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  Alert,
  Switch,
} from 'react-native';
import {
  Card,
  Title,
  Paragraph,
  List,
  Switch as PaperSwitch,
  Button,
  Divider,
} from 'react-native-paper';
import Icon from 'react-native-vector-icons/MaterialIcons';
import {useAuth} from '../contexts/AuthContext';
import {useSecurity} from '../contexts/SecurityContext';
import {colors, spacing, shadows} from '../styles/theme';

const SettingsScreen: React.FC = () => {
  const [notifications, setNotifications] = useState(true);
  const [biometric, setBiometric] = useState(true);
  const [hapticFeedback, setHapticFeedback] = useState(true);
  const [soundEffects, setSoundEffects] = useState(true);

  const {logout, state: authState} = useAuth();
  const {state: securityState, getSecurityScore, getSecurityLevel} = useSecurity();

  const handleLogout = () => {
    Alert.alert(
      'Sair do Sistema',
      'Tem certeza que deseja sair do sistema de votação?',
      [
        {text: 'Cancelar', style: 'cancel'},
        {text: 'Sair', onPress: logout},
      ]
    );
  };

  const handleChangePassword = () => {
    Alert.alert(
      'Alterar Senha',
      'Funcionalidade em desenvolvimento',
      [{text: 'OK'}]
    );
  };

  const handleSecurityInfo = () => {
    const score = getSecurityScore();
    const level = getSecurityLevel();
    
    Alert.alert(
      'Informações de Segurança',
      `Nível de Segurança: ${level.toUpperCase()}\nPontuação: ${score}/100`,
      [{text: 'OK'}]
    );
  };

  const getSecurityColor = () => {
    const level = getSecurityLevel();
    switch (level) {
      case 'high': return colors.success;
      case 'medium': return colors.warning;
      case 'low': return colors.warning;
      case 'critical': return colors.error;
      default: return colors.warning;
    }
  };

  return (
    <ScrollView style={styles.container} showsVerticalScrollIndicator={false}>
      {/* Header */}
      <View style={styles.header}>
        <Title style={styles.headerTitle}>Configurações</Title>
        <Paragraph style={styles.headerSubtitle}>
          Personalize sua experiência de votação
        </Paragraph>
      </View>

      {/* Informações do Usuário */}
      <Card style={[styles.card, shadows.medium]}>
        <Card.Content style={styles.cardContent}>
          <Title style={styles.cardTitle}>Informações do Usuário</Title>
          
          <View style={styles.userInfo}>
            <Icon name="person" size={24} color={colors.primary} />
            <View style={styles.userDetails}>
              <Text style={styles.userName}>{authState.voter?.name}</Text>
              <Text style={styles.userCpf}>CPF: {authState.voter?.cpf}</Text>
            </View>
          </View>
          
          <Divider style={styles.divider} />
          
          <View style={styles.userInfo}>
            <Icon name="ballot" size={24} color={colors.primary} />
            <View style={styles.userDetails}>
              <Text style={styles.userLabel}>Zona Eleitoral</Text>
              <Text style={styles.userValue}>{authState.voter?.zone}</Text>
            </View>
          </View>
          
          <View style={styles.userInfo}>
            <Icon name="location-on" size={24} color={colors.primary} />
            <View style={styles.userDetails}>
              <Text style={styles.userLabel}>Seção</Text>
              <Text style={styles.userValue}>{authState.voter?.section}</Text>
            </View>
          </View>
        </Card.Content>
      </Card>

      {/* Configurações de Segurança */}
      <Card style={[styles.card, shadows.medium]}>
        <Card.Content style={styles.cardContent}>
          <Title style={styles.cardTitle}>Segurança</Title>
          
          <List.Item
            title="Status de Segurança"
            description={`Nível: ${getSecurityLevel().toUpperCase()}`}
            left={() => <Icon name="security" size={24} color={getSecurityColor()} />}
            right={() => (
              <Text style={[styles.securityScore, {color: getSecurityColor()}]}>
                {getSecurityScore()}/100
              </Text>
            )}
            onPress={handleSecurityInfo}
          />
          
          <List.Item
            title="Autenticação Biométrica"
            description="Usar biometria para votação"
            left={() => <Icon name="fingerprint" size={24} color={colors.primary} />}
            right={() => (
              <PaperSwitch
                value={biometric}
                onValueChange={setBiometric}
                color={colors.primary}
              />
            )}
          />
          
          <List.Item
            title="Alterar Senha"
            description="Modificar senha de acesso"
            left={() => <Icon name="lock" size={24} color={colors.primary} />}
            right={() => <Icon name="chevron-right" size={24} color={colors.onSurface} />}
            onPress={handleChangePassword}
          />
        </Card.Content>
      </Card>

      {/* Configurações de Interface */}
      <Card style={[styles.card, shadows.medium]}>
        <Card.Content style={styles.cardContent}>
          <Title style={styles.cardTitle}>Interface</Title>
          
          <List.Item
            title="Notificações"
            description="Receber notificações do sistema"
            left={() => <Icon name="notifications" size={24} color={colors.primary} />}
            right={() => (
              <PaperSwitch
                value={notifications}
                onValueChange={setNotifications}
                color={colors.primary}
              />
            )}
          />
          
          <List.Item
            title="Feedback Háptico"
            description="Vibração ao tocar na tela"
            left={() => <Icon name="vibration" size={24} color={colors.primary} />}
            right={() => (
              <PaperSwitch
                value={hapticFeedback}
                onValueChange={setHapticFeedback}
                color={colors.primary}
              />
            )}
          />
          
          <List.Item
            title="Efeitos Sonoros"
            description="Sons de confirmação"
            left={() => <Icon name="volume-up" size={24} color={colors.primary} />}
            right={() => (
              <PaperSwitch
                value={soundEffects}
                onValueChange={setSoundEffects}
                color={colors.primary}
              />
            )}
          />
        </Card.Content>
      </Card>

      {/* Informações do Sistema */}
      <Card style={[styles.card, shadows.medium]}>
        <Card.Content style={styles.cardContent}>
          <Title style={styles.cardTitle}>Sistema</Title>
          
          <List.Item
            title="Versão do App"
            description="1.0.0"
            left={() => <Icon name="info" size={24} color={colors.primary} />}
          />
          
          <List.Item
            title="Versão do Sistema"
            description="FORTIS v1.0.0"
            left={() => <Icon name="build" size={24} color={colors.primary} />}
          />
          
          <List.Item
            title="Última Atualização"
            description="15/12/2025"
            left={() => <Icon name="update" size={24} color={colors.primary} />}
          />
        </Card.Content>
      </Card>

      {/* Botão de Logout */}
      <View style={styles.logoutContainer}>
        <Button
          mode="contained"
          onPress={handleLogout}
          style={styles.logoutButton}
          contentStyle={styles.buttonContent}
          icon="logout"
          buttonColor={colors.error}>
          Sair do Sistema
        </Button>
      </View>

      {/* Rodapé */}
      <View style={styles.footer}>
        <Text style={styles.footerText}>
          FORTIS - Sistema de Votação Eletrônica Seguro
        </Text>
        <Text style={styles.footerSubtext}>
          Desenvolvido com tecnologia blockchain e criptografia de ponta
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
    marginBottom: spacing.md,
  },
  userInfo: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: spacing.md,
  },
  userDetails: {
    marginLeft: spacing.md,
    flex: 1,
  },
  userName: {
    fontSize: 16,
    fontWeight: 'bold',
    color: colors.onSurface,
  },
  userCpf: {
    fontSize: 14,
    color: colors.onSurface,
    opacity: 0.7,
  },
  userLabel: {
    fontSize: 14,
    color: colors.onSurface,
    opacity: 0.7,
  },
  userValue: {
    fontSize: 16,
    color: colors.onSurface,
    fontWeight: '500',
  },
  divider: {
    marginVertical: spacing.sm,
  },
  securityScore: {
    fontSize: 16,
    fontWeight: 'bold',
  },
  logoutContainer: {
    padding: spacing.lg,
  },
  logoutButton: {
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

export default SettingsScreen;
