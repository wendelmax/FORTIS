import React, {useEffect} from 'react';
import {
  View,
  Text,
  StyleSheet,
  Image,
  Animated,
  Dimensions,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import {colors, spacing} from '../styles/theme';

const {width, height} = Dimensions.get('window');

const SplashScreen: React.FC = () => {
  const fadeAnim = new Animated.Value(0);
  const scaleAnim = new Animated.Value(0.8);

  useEffect(() => {
    // Animação de entrada
    Animated.parallel([
      Animated.timing(fadeAnim, {
        toValue: 1,
        duration: 1000,
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

  return (
    <LinearGradient
      colors={[colors.primary, colors.primaryVariant]}
      style={styles.container}>
      <View style={styles.content}>
        <Animated.View
          style={[
            styles.logoContainer,
            {
              opacity: fadeAnim,
              transform: [{scale: scaleAnim}],
            },
          ]}>
          {/* Logo do FORTIS */}
          <View style={styles.logo}>
            <Text style={styles.logoText}>FORTIS</Text>
            <Text style={styles.logoSubtext}>Sistema de Votação Eletrônica</Text>
          </View>
        </Animated.View>

        <Animated.View
          style={[
            styles.footer,
            {
              opacity: fadeAnim,
            },
          ]}>
          <Text style={styles.footerText}>
            Seguro • Transparente • Auditável
          </Text>
          <Text style={styles.versionText}>Versão 1.0.0</Text>
        </Animated.View>
      </View>

      {/* Indicador de carregamento */}
      <Animated.View
        style={[
          styles.loadingContainer,
          {
            opacity: fadeAnim,
          },
        ]}>
        <View style={styles.loadingBar}>
          <Animated.View style={[styles.loadingProgress, {opacity: fadeAnim}]} />
        </View>
        <Text style={styles.loadingText}>Inicializando sistema...</Text>
      </Animated.View>
    </LinearGradient>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  content: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    paddingHorizontal: spacing.lg,
  },
  logoContainer: {
    alignItems: 'center',
    marginBottom: spacing.xxl,
  },
  logo: {
    alignItems: 'center',
    marginBottom: spacing.lg,
  },
  logoText: {
    fontSize: 48,
    fontWeight: 'bold',
    color: colors.onPrimary,
    textAlign: 'center',
    letterSpacing: 2,
  },
  logoSubtext: {
    fontSize: 16,
    color: colors.onPrimary,
    textAlign: 'center',
    marginTop: spacing.sm,
    opacity: 0.9,
  },
  footer: {
    position: 'absolute',
    bottom: 100,
    alignItems: 'center',
  },
  footerText: {
    fontSize: 14,
    color: colors.onPrimary,
    textAlign: 'center',
    opacity: 0.8,
    marginBottom: spacing.sm,
  },
  versionText: {
    fontSize: 12,
    color: colors.onPrimary,
    textAlign: 'center',
    opacity: 0.6,
  },
  loadingContainer: {
    position: 'absolute',
    bottom: 50,
    alignItems: 'center',
    width: '100%',
    paddingHorizontal: spacing.lg,
  },
  loadingBar: {
    width: '100%',
    height: 4,
    backgroundColor: 'rgba(255, 255, 255, 0.3)',
    borderRadius: 2,
    overflow: 'hidden',
    marginBottom: spacing.sm,
  },
  loadingProgress: {
    width: '100%',
    height: '100%',
    backgroundColor: colors.onPrimary,
    borderRadius: 2,
  },
  loadingText: {
    fontSize: 12,
    color: colors.onPrimary,
    textAlign: 'center',
    opacity: 0.8,
  },
});

export default SplashScreen;
