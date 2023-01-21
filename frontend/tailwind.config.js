/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
    "./app/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      keyframes: {
        'yes-swipe': {
          '0%': {
            opacity: '1',
            transform: 'translateX(0px) rotate(0deg)',
          },
          '100%': {
            opacity: '0',
            transform: 'translateX(100px) rotate(4deg)'
          },
        },
        'no-swipe': {
          '0%': {
            opacity: '1',
            transform: 'translateX(0px) rotate(0deg)',
          },
          '100%': {
            opacity: '0',
            transform: 'translateX(-100px) rotate(-4deg)'
          },
        }
      },
      animation: {
        'yes-swipe': 'yes-swipe 0.5s ease-out',
        'no-swipe': 'no-swipe 0.5s ease-out'
      }
    },
  },
  plugins: [require("daisyui")],
};
