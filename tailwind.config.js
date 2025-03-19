// import daisyui from 'daisyui';
module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css}",
    "./dist/**/*.html",
    "./assets/**/*.css",
  ],
  theme: {
    extend: {
      fontFamily: {
        Nunito: ['nunito', 'sans-serif'],
        Jura: ['Jura', 'sans-serif'],
      },
    },
    

  },
  plugins: [
  ],
}