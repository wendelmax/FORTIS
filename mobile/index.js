import {AppRegistry} from 'react-native';
import App from './src/App';
import {name as appName} from './package.json';

// Configuração de splash screen
import SplashScreen from 'react-native-splash-screen';
import {Platform} from 'react-native';

// Configuração de orientação
import Orientation from 'react-native-orientation-locker';

// Configuração de keep awake
import KeepAwake from 'react-native-keep-awake';

AppRegistry.registerComponent(appName, () => {
  // Configurações iniciais
  if (Platform.OS === 'android') {
    // Manter tela ligada durante votação
    KeepAwake.activate();
    
    // Bloquear orientação para portrait
    Orientation.lockToPortrait();
  }
  
  // Esconder splash screen após carregamento
  setTimeout(() => {
    SplashScreen.hide();
  }, 2000);
  
  return App;
});
