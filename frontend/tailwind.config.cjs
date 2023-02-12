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
        'amongus-title': ['Impostograph-Regular'],
        'amongus-text' : ['Inyourfacejoffrey'],
        'amongus-log' : ['VCR_OSD_MONO_1.001']
      }
    },
  },
  plugins: [],
}
