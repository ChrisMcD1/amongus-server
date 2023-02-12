/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
      "./index.html",
      "./src/**/*.{js,ts,jsx,tsx}"
  ],
  theme: {
    extend: {
        backgroundImage: {
            'space-stars': "url('/Pictures/amongusbackground2.jpg')",
            'lobby': "url('/Pictures/amonguslobby.jpg')"
        },
        fontFamily: {
        'amongus-title': ['amongus-title'],
        'amongus-text' : ['amongus-text'],
        'amongus-log' : ['amongus-log']
      }
    },
  },
  plugins: [],
}
