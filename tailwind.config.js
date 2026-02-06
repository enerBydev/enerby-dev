/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html"
  ],
  safelist: [
    {
      pattern: /prose/,
      variants: ['lg', 'md', 'sm', 'hover'],
    },
    'prose-invert',
    'max-w-none',
  ],
  theme: {
    extend: {
      // ========================================
      // Colores Cyberpunk Neon
      // ========================================
      colors: {
        // Primario - Cyan Neon
        'primary': '#00FFFF',
        'primary-dark': '#00CCCC',
        'primary-light': '#66FFFF',
        
        // Secundarios - Acentos Neon
        'secondary-pink': '#FF00FF',
        'secondary-purple': '#9D00FF',
        'secondary-orange': '#FF6600',
        
        // Fondos - Dark Mode
        'bg-primary': '#0A0A0F',
        'bg-secondary': '#12121A',
        'bg-tertiary': '#1A1A25',
        'bg-card': 'rgba(20, 20, 30, 0.7)',
        'bg-elevated': '#1E1E2E',
        'bg-element': '#1E1E2E',
        
        // Texto
        'text-primary': '#FFFFFF',
        'text-secondary': '#B0B0C0',
        'text-muted': '#6B6B7B',
        'secondary': '#B0B0C0',
        'muted': '#6B6B7B',
        
        // Estados
        'success': '#00FF88',
        'warning': '#FFB800',
        'error': '#FF3366',
      },
      
      // ========================================
      // Tipografía Cyberpunk
      // ========================================
      fontFamily: {
        'display': ['Orbitron', 'sans-serif'],
        'body': ['Inter', 'system-ui', 'sans-serif'],
        'mono': ['JetBrains Mono', 'Fira Code', 'monospace'],
      },
      
      // ========================================
      // Sombras Glow Neon
      // ========================================
      boxShadow: {
        'glow-sm': '0 0 10px rgba(0, 255, 255, 0.3)',
        'glow-md': '0 0 20px rgba(0, 255, 255, 0.4)',
        'glow-lg': '0 0 30px rgba(0, 255, 255, 0.5), 0 0 60px rgba(0, 255, 255, 0.3)',
        'neon': '0 0 10px #00FFFF40, inset 0 0 10px #00FFFF20',
      },
      
      // ========================================
      // Animaciones Cyberpunk
      // ========================================
      animation: {
        'glitch': 'glitch 3s infinite',
        'glitch-1': 'glitch-1 0.3s infinite',
        'glitch-2': 'glitch-2 0.3s infinite',
        'glow-pulse': 'glow-pulse 2s ease-in-out infinite',
        'text-glow': 'text-glow-pulse 2s ease-in-out infinite',
        'float': 'float 3s ease-in-out infinite',
        'float-delayed': 'float 3s ease-in-out infinite 1.5s',
        'spin-slow': 'spin 8s linear infinite',
        'pulse-slow': 'pulse 3s ease-in-out infinite',
        'shine': 'shine 0.5s ease-out forwards',
        'scroll-indicator': 'scroll-indicator 1.5s ease-in-out infinite',
        'flicker': 'flicker 5s linear infinite',
      },
      
      keyframes: {
        'glitch': {
          '0%, 90%, 100%': { transform: 'translate(0)', filter: 'none' },
          '92%': { transform: 'translate(-2px, 1px)', filter: 'hue-rotate(90deg)' },
          '94%': { transform: 'translate(2px, -1px)', filter: 'hue-rotate(-90deg)' },
          '96%': { transform: 'translate(-1px, 2px)' },
          '98%': { transform: 'translate(1px, -2px)', filter: 'saturate(2)' },
        },
        'glitch-1': {
          '0%, 100%': { transform: 'translate(0)' },
          '20%': { transform: 'translate(-2px, 2px)' },
          '40%': { transform: 'translate(-2px, -2px)' },
          '60%': { transform: 'translate(2px, 2px)' },
          '80%': { transform: 'translate(2px, -2px)' },
        },
        'glitch-2': {
          '0%, 100%': { transform: 'translate(0)' },
          '20%': { transform: 'translate(2px, -2px)' },
          '40%': { transform: 'translate(2px, 2px)' },
          '60%': { transform: 'translate(-2px, -2px)' },
          '80%': { transform: 'translate(-2px, 2px)' },
        },
        'glow-pulse': {
          '0%, 100%': { boxShadow: '0 0 10px #00FFFF40, 0 0 20px #00FFFF20' },
          '50%': { boxShadow: '0 0 20px #00FFFF60, 0 0 40px #00FFFF40, 0 0 60px #00FFFF20' },
        },
        'text-glow-pulse': {
          '0%, 100%': { textShadow: '0 0 10px rgba(0, 255, 255, 0.5)' },
          '50%': { textShadow: '0 0 20px rgba(0, 255, 255, 0.5), 0 0 40px rgba(0, 255, 255, 0.5)' },
        },
        'float': {
          '0%, 100%': { transform: 'translateY(0)' },
          '50%': { transform: 'translateY(-10px)' },
        },
        'shine': {
          'from': { transform: 'translateX(-100%)' },
          'to': { transform: 'translateX(100%)' },
        },
        'scroll-indicator': {
          '0%, 100%': { opacity: '1', transform: 'translateY(0)' },
          '50%': { opacity: '0.5', transform: 'translateY(4px)' },
        },
        'flicker': {
          '0%, 19.999%, 22%, 62.999%, 64%, 64.999%, 70%, 100%': { opacity: '1' },
          '20%, 21.999%, 63%, 63.999%, 65%, 69.999%': { opacity: '0.4' },
        },
      },
      
      // Backdrop blur
      backdropBlur: {
        xs: '2px',
      },
    },
  },
  plugins: [],
}
