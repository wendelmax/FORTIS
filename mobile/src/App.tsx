import React, {useEffect, useState} from 'react';
import {
  StatusBar,
  StyleSheet,
  View,
  Alert,
  BackHandler,
  Platform,
} from 'react-native';
import {NavigationContainer} from '@react-navigation/native';
import {createStackNavigator} from '@react-navigation/stack';
import {Provider as PaperProvider} from 'react-native-paper';
import Toast from 'react-native-toast-message';
import {QueryClient, QueryClientProvider} from 'react-query';
import Orientation from 'react-native-orientation-locker';
import KeepAwake from 'react-native-keep-awake';

// Contexts
import {AuthProvider} from './contexts/AuthContext';
import {VotingProvider} from './contexts/VotingContext';
import {SecurityProvider} from './contexts/SecurityContext';

// Screens
import SplashScreen from './screens/SplashScreen';
import LoginScreen from './screens/LoginScreen';
import BiometricAuthScreen from './screens/BiometricAuthScreen';
import ElectionListScreen from './screens/ElectionListScreen';
import VotingScreen from './screens/VotingScreen';
import VoteConfirmationScreen from './screens/VoteConfirmationScreen';
import VoteReceiptScreen from './screens/VoteReceiptScreen';
import SettingsScreen from './screens/SettingsScreen';
import HelpScreen from './screens/HelpScreen';

// Services
import {SecurityService} from './services/SecurityService';
import {VotingService} from './services/VotingService';

// Types
import {RootStackParamList} from './types/navigation';

// Theme
import {theme} from './styles/theme';

const Stack = createStackNavigator<RootStackParamList>();
const queryClient = new QueryClient();

const App: React.FC = () => {
  const [isInitialized, setIsInitialized] = useState(false);
  const [isSecure, setIsSecure] = useState(false);

  useEffect(() => {
    initializeApp();
    
    // Configurações de segurança
    if (Platform.OS === 'android') {
      // Manter tela ligada durante votação
      KeepAwake.activate();
      
      // Bloquear orientação para portrait
      Orientation.lockToPortrait();
    }

    // Prevenir voltar durante votação
    const backHandler = BackHandler.addEventListener('hardwareBackPress', () => {
      Alert.alert(
        'Sair do Sistema',
        'Tem certeza que deseja sair do sistema de votação?',
        [
          {text: 'Cancelar', style: 'cancel'},
          {text: 'Sair', onPress: () => BackHandler.exitApp()},
        ]
      );
      return true;
    });

    return () => {
      backHandler.remove();
      KeepAwake.deactivate();
    };
  }, []);

  const initializeApp = async () => {
    try {
      // Verificar segurança do dispositivo
      const securityService = new SecurityService();
      const securityCheck = await securityService.performSecurityCheck();
      
      if (!securityCheck.isSecure) {
        Alert.alert(
          'Dispositivo Inseguro',
          'Este dispositivo não atende aos requisitos de segurança para votação eletrônica.',
          [{text: 'OK', onPress: () => BackHandler.exitApp()}]
        );
        return;
      }

      setIsSecure(true);
      setIsInitialized(true);
    } catch (error) {
      console.error('Erro na inicialização:', error);
      Alert.alert(
        'Erro de Inicialização',
        'Não foi possível inicializar o aplicativo. Tente novamente.',
        [{text: 'OK', onPress: () => BackHandler.exitApp()}]
      );
    }
  };

  if (!isInitialized) {
    return <SplashScreen />;
  }

  if (!isSecure) {
    return (
      <View style={styles.container}>
        <StatusBar barStyle="light-content" backgroundColor="#d32f2f" />
      </View>
    );
  }

  return (
    <QueryClientProvider client={queryClient}>
      <PaperProvider theme={theme}>
        <AuthProvider>
          <VotingProvider>
            <SecurityProvider>
              <NavigationContainer>
                <StatusBar
                  barStyle="light-content"
                  backgroundColor={theme.colors.primary}
                />
                <Stack.Navigator
                  initialRouteName="Login"
                  screenOptions={{
                    headerShown: false,
                    gestureEnabled: false,
                    animationEnabled: false,
                  }}>
                  <Stack.Screen name="Login" component={LoginScreen} />
                  <Stack.Screen name="BiometricAuth" component={BiometricAuthScreen} />
                  <Stack.Screen name="ElectionList" component={ElectionListScreen} />
                  <Stack.Screen name="Voting" component={VotingScreen} />
                  <Stack.Screen name="VoteConfirmation" component={VoteConfirmationScreen} />
                  <Stack.Screen name="VoteReceipt" component={VoteReceiptScreen} />
                  <Stack.Screen name="Settings" component={SettingsScreen} />
                  <Stack.Screen name="Help" component={HelpScreen} />
                </Stack.Navigator>
                <Toast />
              </NavigationContainer>
            </SecurityProvider>
          </VotingProvider>
        </AuthProvider>
      </PaperProvider>
    </QueryClientProvider>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#d32f2f',
    justifyContent: 'center',
    alignItems: 'center',
  },
});

export default App;
