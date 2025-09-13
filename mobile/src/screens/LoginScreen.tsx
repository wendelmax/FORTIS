import React, {useState, useEffect} from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  Alert,
  KeyboardAvoidingView,
  Platform,
  TouchableOpacity,
} from 'react-native';
import {TextInput, Button, Card, Title, Paragraph} from 'react-native-paper';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/MaterialIcons';
import {useAuth} from '../contexts/AuthContext';
import {useSecurity} from '../contexts/SecurityContext';
import {colors, spacing, shadows} from '../styles/theme';

const LoginScreen: React.FC = () => {
  const [cpf, setCpf] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [isSecure, setIsSecure] = useState(false);

  const {login, state: authState} = useAuth();
  const {state: securityState, isDeviceSecure} = useSecurity();

  useEffect(() => {
    // Verificar segurança do dispositivo
    const checkSecurity = async () => {
      const secure = await isDeviceSecure();
      setIsSecure(secure);
    };
    checkSecurity();
  }, []);

  const handleLogin = async () => {
    if (!cpf.trim() || !password.trim()) {
      Alert.alert('Erro', 'Por favor, preencha todos os campos');
      return;
    }

    if (!isSecure) {
      Alert.alert(
        'Dispositivo Inseguro',
        'Este dispositivo não atende aos requisitos de segurança para votação eletrônica.',
        [{text: 'OK'}]
      );
      return;
    }

    setIsLoading(true);
    try {
      await login(cpf, password);
    } catch (error) {
      Alert.alert(
        'Erro no Login',
        error instanceof Error ? error.message : 'Erro desconhecido'
      );
    } finally {
      setIsLoading(false);
    }
  };

  const formatCpf = (text: string) => {
    // Remove caracteres não numéricos
    const numbers = text.replace(/\D/g, '');
    
    // Aplica máscara do CPF
    if (numbers.length <= 11) {
      const formatted = numbers.replace(
        /(\d{3})(\d{3})(\d{3})(\d{2})/,
        '$1.$2.$3-$4'
      );
      setCpf(formatted);
    }
  };

  const getSecurityStatus = () => {
    if (!securityState.securityCheck) return 'Verificando...';
    
    const score = securityState.securityCheck.score;
    if (score >= 90) return 'Seguro';
    if (score >= 70) return 'Moderadamente Seguro';
    if (score >= 50) return 'Pouco Seguro';
    return 'Inseguro';
  };

  const getSecurityColor = () => {
    if (!securityState.securityCheck) return colors.warning;
    
    const score = securityState.securityCheck.score;
    if (score >= 90) return colors.success;
    if (score >= 70) return colors.warning;
    if (score >= 50) return colors.warning;
    return colors.error;
  };

  return (
    <LinearGradient
      colors={[colors.primary, colors.primaryVariant]}
      style={styles.container}>
      <KeyboardAvoidingView
        behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
        style={styles.keyboardView}>
        <ScrollView
          contentContainerStyle={styles.scrollContent}
          showsVerticalScrollIndicator={false}>
          
          {/* Header */}
          <View style={styles.header}>
            <Icon name="how-to-vote" size={64} color={colors.onPrimary} />
            <Title style={styles.title}>FORTIS</Title>
            <Paragraph style={styles.subtitle}>
              Sistema de Votação Eletrônica Seguro
            </Paragraph>
          </View>

          {/* Card de Login */}
          <Card style={[styles.loginCard, shadows.medium]}>
            <Card.Content style={styles.cardContent}>
              <Title style={styles.cardTitle}>Acesso ao Sistema</Title>
              
              {/* Status de Segurança */}
              <View style={styles.securityStatus}>
                <Icon 
                  name={isSecure ? "security" : "security"} 
                  size={20} 
                  color={getSecurityColor()} 
                />
                <Text style={[styles.securityText, {color: getSecurityColor()}]}>
                  {getSecurityStatus()}
                </Text>
              </View>

              {/* Campo CPF */}
              <TextInput
                label="CPF"
                value={cpf}
                onChangeText={formatCpf}
                mode="outlined"
                keyboardType="numeric"
                maxLength={14}
                style={styles.input}
                left={<TextInput.Icon name="account" />}
                error={authState.error?.code === 'LOGIN_FAILED'}
              />

              {/* Campo Senha */}
              <TextInput
                label="Senha"
                value={password}
                onChangeText={setPassword}
                mode="outlined"
                secureTextEntry={!showPassword}
                style={styles.input}
                left={<TextInput.Icon name="lock" />}
                right={
                  <TextInput.Icon
                    name={showPassword ? "eye-off" : "eye"}
                    onPress={() => setShowPassword(!showPassword)}
                  />
                }
                error={authState.error?.code === 'LOGIN_FAILED'}
              />

              {/* Botão de Login */}
              <Button
                mode="contained"
                onPress={handleLogin}
                loading={isLoading}
                disabled={isLoading || !isSecure}
                style={styles.loginButton}
                contentStyle={styles.buttonContent}>
                {isLoading ? 'Entrando...' : 'Entrar'}
              </Button>

              {/* Mensagem de Erro */}
              {authState.error && (
                <Text style={styles.errorText}>
                  {authState.error.message}
                </Text>
              )}

              {/* Aviso de Segurança */}
              {!isSecure && (
                <View style={styles.securityWarning}>
                  <Icon name="warning" size={20} color={colors.error} />
                  <Text style={styles.warningText}>
                    Dispositivo não atende aos requisitos de segurança
                  </Text>
                </View>
              )}
            </Card.Content>
          </Card>

          {/* Informações Adicionais */}
          <View style={styles.footer}>
            <Text style={styles.footerText}>
              Sistema seguro e auditável para votação eletrônica
            </Text>
            <Text style={styles.footerSubtext}>
              Desenvolvido com tecnologia blockchain e criptografia de ponta
            </Text>
          </View>
        </ScrollView>
      </KeyboardAvoidingView>
    </LinearGradient>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  keyboardView: {
    flex: 1,
  },
  scrollContent: {
    flexGrow: 1,
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.xl,
  },
  header: {
    alignItems: 'center',
    marginBottom: spacing.xl,
    marginTop: spacing.xxl,
  },
  title: {
    fontSize: 32,
    fontWeight: 'bold',
    color: colors.onPrimary,
    marginTop: spacing.md,
  },
  subtitle: {
    fontSize: 16,
    color: colors.onPrimary,
    textAlign: 'center',
    marginTop: spacing.sm,
    opacity: 0.9,
  },
  loginCard: {
    marginBottom: spacing.xl,
    backgroundColor: colors.surface,
  },
  cardContent: {
    padding: spacing.lg,
  },
  cardTitle: {
    textAlign: 'center',
    marginBottom: spacing.lg,
    color: colors.onSurface,
  },
  securityStatus: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    marginBottom: spacing.lg,
    paddingVertical: spacing.sm,
    paddingHorizontal: spacing.md,
    backgroundColor: colors.background,
    borderRadius: 20,
  },
  securityText: {
    marginLeft: spacing.sm,
    fontWeight: '500',
  },
  input: {
    marginBottom: spacing.md,
  },
  loginButton: {
    marginTop: spacing.md,
    marginBottom: spacing.sm,
  },
  buttonContent: {
    paddingVertical: spacing.sm,
  },
  errorText: {
    color: colors.error,
    textAlign: 'center',
    marginTop: spacing.sm,
  },
  securityWarning: {
    flexDirection: 'row',
    alignItems: 'center',
    marginTop: spacing.md,
    padding: spacing.sm,
    backgroundColor: colors.error + '20',
    borderRadius: 8,
  },
  warningText: {
    marginLeft: spacing.sm,
    color: colors.error,
    flex: 1,
  },
  footer: {
    alignItems: 'center',
    marginTop: spacing.lg,
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

export default LoginScreen;
