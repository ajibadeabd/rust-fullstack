module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./docs/**/*.html",
  ],
  theme: {
    extend: {
      // colors: {
      //   red: "#FF0000", // You can replace this with your desired shade of red
      // },
    },
  },
  plugins: [],
};
