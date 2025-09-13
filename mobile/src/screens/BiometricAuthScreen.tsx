import React, {useState, useEffect} from 'react';
import {
  View,
  Text,
  StyleSheet,
  Alert,
  Animated,
  Dimensions,
} from 'react-native';
import {Button, Card, Title, Paragraph} from 'react-native-paper';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/MaterialIcons';
import ReactNativeBiometrics from 'react-native-biometrics';
import {useAuth} from '../contexts/AuthContext';
import {colors, spacing, shadows} from '../styles/theme';

const {width, height} = Dimensions.get('window');

interface BiometricAuthScreenProps {
  route: {
    params: {
      cpf: string;
      accessToken: string;
    };
  };
  navigation: any;
}

const BiometricAuthScreen: React.FC<BiometricAuthScreenProps> = ({
  route,
  navigation,
}) => {
  const [isLoading, setIsLoading] = useState(false);
  const [biometricType, setBiometricType] = useState<string>('');
  const [isAvailable, setIsAvailable] = useState(false);
  const [pulseAnim] = useState(new Animated.Value(1));

  const {loginWithBiometric, state: authState} = useAuth();

  useEffect(() => {
    checkBiometricAvailability();
    startPulseAnimation();
  }, []);

  const checkBiometricAvailability = async () => {
    try {
      const rnBiometrics = new ReactNativeBiometrics();
      const {available, biometryType} = await rnBiometrics.isSensorAvailable();
      
      setIsAvailable(available);
      setBiometricType(biometryType || '');
    } catch (error) {
      console.error('Erro ao verificar biometria:', error);
      setIsAvailable(false);
    }
  };

  const startPulseAnimation = () => {
    Animated.loop(
      Animated.sequence([
        Animated.timing(pulseAnim, {
          toValue: 1.2,
          duration: 1000,
          useNativeDriver: true,
        }),
        Animated.timing(pulseAnim, {
          toValue: 1,
          duration: 1000,
          useNativeDriver: true,
        }),
      ])
    ).start();
  };

  const handleBiometricAuth = async () => {
    if (!isAvailable) {
      Alert.alert('Erro', 'Biometria não disponível neste dispositivo');
      return;
    }

    setIsLoading(true);
    try {
      const rnBiometrics = new ReactNativeBiometrics();
      const {success, signature} = await rnBiometrics.createSignature({
        promptMessage: 'Autentique-se para votar',
        payload: route.params.cpf,
      });

      if (success && signature) {
        const biometricData = {
          type: biometricType.toLowerCase() as 'fingerprint' | 'face' | 'voice',
          data: signature,
          timestamp: new Date().toISOString(),
        };

        await loginWithBiometric(biometricData);
        navigation.navigate('ElectionList');
      }
    } catch (error) {
      console.error('Erro na autenticação biométrica:', error);
      Alert.alert(
        'Erro na Autenticação',
        'Falha na autenticação biométrica. Tente novamente.'
      );
    } finally {
      setIsLoading(false);
    }
  };

  const handleSkipBiometric = () => {
    Alert.alert(
      'Pular Autenticação Biométrica',
      'Você tem certeza que deseja pular a autenticação biométrica? Isso pode comprometer a segurança da sua votação.',
      [
        {text: 'Cancelar', style: 'cancel'},
        {
          text: 'Pular',
          style: 'destructive',
          onPress: () => navigation.navigate('ElectionList'),
        },
      ]
    );
  };

  const getBiometricIcon = () => {
    switch (biometricType) {
      case 'TouchID':
        return 'fingerprint';
      case 'FaceID':
        return 'face';
      case 'Biometrics':
        return 'security';
      default:
        return 'security';
    }
  };

  const getBiometricText = () => {
    switch (biometricType) {
      case 'TouchID':
        return 'Toque no sensor de impressão digital';
      case 'FaceID':
        return 'Olhe para a câmera';
      case 'Biometrics':
        return 'Use sua biometria';
      default:
        return 'Autenticação biométrica';
    }
  };

  return (
    <LinearGradient
      colors={[colors.primary, colors.primaryVariant]}
      style={styles.container}>
      <View style={styles.content}>
        
        {/* Header */}
        <View style={styles.header}>
          <Animated.View
            style={[
              styles.iconContainer,
              {
                transform: [{scale: pulseAnim}],
              },
            ]}>
            <Icon
              name={getBiometricIcon()}
              size={80}
              color={colors.onPrimary}
            />
          </Animated.View>
          <Title style={styles.title}>Autenticação Biométrica</Title>
          <Paragraph style={styles.subtitle}>
            {getBiometricText()}
          </Paragraph>
        </View>

        {/* Card de Informações */}
        <Card style={[styles.infoCard, shadows.medium]}>
          <Card.Content style={styles.cardContent}>
            <View style={styles.infoItem}>
              <Icon name="person" size={24} color={colors.primary} />
              <Text style={styles.infoText}>CPF: {route.params.cpf}</Text>
            </View>
            
            <View style={styles.infoItem}>
              <Icon name="security" size={24} color={colors.primary} />
              <Text style={styles.infoText}>
                Tipo: {biometricType || 'Não disponível'}
              </Text>
            </View>

            <View style={styles.infoItem}>
              <Icon name="check-circle" size={24} color={colors.primary} />
              <Text style={styles.infoText}>
                Status: {isAvailable ? 'Disponível' : 'Indisponível'}
              </Text>
            </View>
          </Card.Content>
        </Card>

        {/* Botões de Ação */}
        <View style={styles.buttonContainer}>
          <Button
            mode="contained"
            onPress={handleBiometricAuth}
            loading={isLoading}
            disabled={!isAvailable || isLoading}
            style={styles.biometricButton}
            contentStyle={styles.buttonContent}
            icon="fingerprint">
            {isLoading ? 'Autenticando...' : 'Autenticar com Biometria'}
          </Button>

          <Button
            mode="outlined"
            onPress={handleSkipBiometric}
            disabled={isLoading}
            style={styles.skipButton}
            contentStyle={styles.buttonContent}
            icon="skip-next">
            Pular (Não Recomendado)
          </Button>
        </View>

        {/* Avisos de Segurança */}
        <View style={styles.warningContainer}>
          <View style={styles.warningItem}>
            <Icon name="info" size={20} color={colors.warning} />
            <Text style={styles.warningText}>
              A autenticação biométrica garante que apenas você possa votar
            </Text>
          </View>
          
          <View style={styles.warningItem}>
            <Icon name="shield" size={20} color={colors.info} />
            <Text style={styles.warningText}>
              Seus dados biométricos são criptografados e não armazenados
            </Text>
          </View>
        </View>

        {/* Mensagem de Erro */}
        {authState.error && (
          <View style={styles.errorContainer}>
            <Icon name="error" size={20} color={colors.error} />
            <Text style={styles.errorText}>
              {authState.error.message}
            </Text>
          </View>
        )}
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
  iconContainer: {
    marginBottom: spacing.lg,
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    color: colors.onPrimary,
    textAlign: 'center',
    marginBottom: spacing.sm,
  },
  subtitle: {
    fontSize: 16,
    color: colors.onPrimary,
    textAlign: 'center',
    opacity: 0.9,
  },
  infoCard: {
    marginBottom: spacing.xl,
    backgroundColor: colors.surface,
  },
  cardContent: {
    padding: spacing.lg,
  },
  infoItem: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: spacing.md,
  },
  infoText: {
    marginLeft: spacing.md,
    fontSize: 16,
    color: colors.onSurface,
  },
  buttonContainer: {
    marginBottom: spacing.xl,
  },
  biometricButton: {
    marginBottom: spacing.md,
  },
  skipButton: {
    borderColor: colors.onPrimary,
  },
  buttonContent: {
    paddingVertical: spacing.sm,
  },
  warningContainer: {
    marginBottom: spacing.lg,
  },
  warningItem: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    marginBottom: spacing.sm,
    paddingHorizontal: spacing.sm,
  },
  warningText: {
    marginLeft: spacing.sm,
    fontSize: 14,
    color: colors.onPrimary,
    opacity: 0.8,
    flex: 1,
  },
  errorContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.error + '20',
    padding: spacing.sm,
    borderRadius: 8,
  },
  errorText: {
    marginLeft: spacing.sm,
    color: colors.error,
    flex: 1,
  },
});

export default BiometricAuthScreen;
