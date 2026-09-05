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
        paper: '#0B0E14',
        surface: {
          DEFAULT: '#141721',
          elevated: '#1C202C',
        },
        border: {
          DEFAULT: '#252B3B',
          subtle: '#1B202D',
        },
        ink: {
          DEFAULT: '#F9FAFB',
          secondary: '#9CA3AF',
          muted: '#6B7280',
        },
        signal: {
          DEFAULT: '#10B981',
          hover: '#059669',
          glow: 'rgba(16, 185, 129, 0.15)',
        },
        idle: {
          DEFAULT: '#374151',
          surface: '#1F2430',
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
