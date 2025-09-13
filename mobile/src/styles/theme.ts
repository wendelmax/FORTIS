import {DefaultTheme} from 'react-native-paper';

export const theme = {
  ...DefaultTheme,
  colors: {
    ...DefaultTheme.colors,
    primary: '#1976d2', // Azul TSE
    primaryVariant: '#1565c0',
    secondary: '#4caf50', // Verde confirmação
    secondaryVariant: '#388e3c',
    surface: '#ffffff',
    background: '#f5f5f5',
    error: '#d32f2f', // Vermelho erro
    onPrimary: '#ffffff',
    onSecondary: '#ffffff',
    onSurface: '#212121',
    onBackground: '#212121',
    onError: '#ffffff',
    // Cores específicas do FORTIS
    success: '#4caf50',
    warning: '#ff9800',
    info: '#2196f3',
    // Cores de votação
    voteButton: '#4caf50',
    voteButtonPressed: '#388e3c',
    cancelButton: '#f44336',
    cancelButtonPressed: '#d32f2f',
    // Cores de segurança
    securityHigh: '#4caf50',
    securityMedium: '#ff9800',
    securityLow: '#f44336',
    securityCritical: '#d32f2f',
  },
  fonts: {
    ...DefaultTheme.fonts,
    regular: {
      fontFamily: 'System',
      fontWeight: '400' as const,
    },
    medium: {
      fontFamily: 'System',
      fontWeight: '500' as const,
    },
    light: {
      fontFamily: 'System',
      fontWeight: '300' as const,
    },
    thin: {
      fontFamily: 'System',
      fontWeight: '100' as const,
    },
  },
  roundness: 8,
  spacing: {
    xs: 4,
    sm: 8,
    md: 16,
    lg: 24,
    xl: 32,
    xxl: 48,
  },
  shadows: {
    small: {
      shadowColor: '#000',
      shadowOffset: {
        width: 0,
        height: 1,
      },
      shadowOpacity: 0.22,
      shadowRadius: 2.22,
      elevation: 3,
    },
    medium: {
      shadowColor: '#000',
      shadowOffset: {
        width: 0,
        height: 2,
      },
      shadowOpacity: 0.25,
      shadowRadius: 3.84,
      elevation: 5,
    },
    large: {
      shadowColor: '#000',
      shadowOffset: {
        width: 0,
        height: 4,
      },
      shadowOpacity: 0.30,
      shadowRadius: 4.65,
      elevation: 8,
    },
  },
};

export const colors = theme.colors;
export const spacing = theme.spacing;
export const shadows = theme.shadows;
