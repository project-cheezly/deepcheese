export default {
  content: ['./src/**/*.{html,js,svelte}'],
  theme: {
    container: {
      center: true,
      screens: {
        sm: '100%',
        md: '100%',
        lg: '100%',
        xl: '1280px'
      }
    },
    extend: {
      colors: {
        primary: 'var(--md-sys-color-primary)',
        background: 'var(--md-sys-color-background)',
        border: 'var(--md-sys-color-outline)',
        text: 'var(--md-sys-color-on-background)',
        foreground: 'var(--md-sys-color-on-background)',

        surface: 'var(--md-sys-color-surface-container)',
        
        muted: {
          DEFAULT: 'hsl(var(--muted) / <alpha-value>)',
          foreground: 'hsl(var(--muted-foreground) / <alpha-value>)',
        }
      }
    },
  },
  plugins: [],
}
