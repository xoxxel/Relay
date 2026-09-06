/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        paper: '#0A0A0C',
        surface: {
          DEFAULT: '#121215',
          elevated: '#1A1B20',
        },
        border: {
          DEFAULT: '#26272D',
          subtle: '#18181C',
        },
        ink: {
          DEFAULT: '#F2F3F5',
          secondary: '#A2A6AE',
          muted: '#6E737A',
        },
        signal: {
          DEFAULT: '#10B981',
          hover: '#059669',
          glow: 'rgba(16, 185, 129, 0.15)',
        },
        idle: {
          DEFAULT: '#2C2D33',
          surface: '#14151A',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        mono: ['"JetBrains Mono"', 'monospace'],
      },
    },
  },
  plugins: [],
}
