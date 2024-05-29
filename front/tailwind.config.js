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
        border: 'var(--color-outline)',
        outline: 'var(--color-outline)',
        background: 'var(--color-background)',
        foreground: 'hsl(var(--foreground) / <alpha-value>)',
        muted: {
          DEFAULT: 'hsl(var(--muted) / <alpha-value>)',
          foreground: 'hsl(var(--muted-foreground) / <alpha-value>)',
        },
        primary: 'var(--color-primary)',
        decrease: 'var(--color-blue)',
        increase: 'var(--color-red)',
        warn: 'var(--color-red)',
      }
    },
  },
  plugins: [],
}
