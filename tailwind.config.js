/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/templates/**/*.rs"],
  theme: {
    screens: {
      sm: "700px",
      // => @media (min-width: 640px) { ... }

      md: "990px",
      // => @media (min-width: 768px) { ... }

      lg: "1024px",
      // => @media (min-width: 1024px) { ... }

      xl: "1280px",
      // => @media (min-width: 1280px) { ... }

      "2xl": "1536px",
      // => @media (min-width: 1536px) { ... }
    },
    extend: {},
  },
  plugins: [],
};
