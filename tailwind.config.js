const plugin = require('tailwindcss/plugin')
const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  content: [
    "./*.{html,css,scss}",
    "./src/**/*.rs"
  ],
  darkMode: 'class',
  theme: {
    colors: {
      primary: {
        DEFAULT: "hsl(170, 100%, 20%)", // #006655
        light: "hsl(170, 100%, 35%)", // #00B394
        black: "hsl(160, 100%, 3%)", // #001A15
      },
      blue: {
        100: "hsl(192, 100%, 5%)", // #001419
        200: "hsl(192, 100%, 10%)", // #002933
        300: "hsl(192, 100%, 15%)", // #003D4D
        400: "hsl(192, 100%, 20%)", // #005266
        500: "hsl(198, 100%, 25%)", // #005980
        600: "hsl(210, 100%, 30%)", // #004D99
        700: "hsl(210, 100%, 40%)", // #0066CC
        800: "hsl(210, 100%, 50%)", // #0080FF
      },
      light: {
        300: "hsl(0, 1%, 33%)", // #555353
        500: "hsl(170, 70%, 70%)", // #7DE8D6
        600: "hsl(160, 65%, 85%)", // #C0F2E1
        700: "hsl(120, 60%, 90%)", // #D6F5D6
        800: "hsl(47, 48%, 90%)", // #F2EDDA
        900: "hsl(44, 85%, 95%)", // #FDF7E7
      },
      black: '#010306',
      star: '#ffd700',
      error: '#c70039',
      "yt-red": '#F00',
      white: '#fdf6f3',
      transparent: "transparent"
    },
    fontFamily: {
      sans: ['Lato', ...defaultTheme.fontFamily.sans],
      serif: ['Lato', ...defaultTheme.fontFamily.serif],
      mono: ['Inconsolata', ...defaultTheme.fontFamily.mono],
    },
    extend: {
      spacing: {
        '8xl': '96rem',
        '9xl': '128rem',
        '1/10vw': '10vw',
        '1/12vw': '8.33333vw',
      },
      borderRadius: {
        '4xl': '2rem',
      }
    }
  },
  plugins: [
    plugin(function({ addVariant }) {
      addVariant('parent-sibling-checked', 'input:checked~* &')
    })
  ],
}
