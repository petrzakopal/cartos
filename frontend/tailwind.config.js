/** @type {import('tailwindcss').Config} */
export default {
    darkMode: ["class"],
    content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
  	extend: {
        width: {
            "inside-full": "1400px",
        },
  		borderRadius: {
  			lg: 'var(--radius)',
  			md: 'calc(var(--radius) - 2px)',
  			sm: 'calc(var(--radius) - 4px)'
  		},
    colors: {
                primary: "#3A2AD0",
                secondary: "#C60C30",
                tertiary: "#00B2A9",
                quaternary: "#F0AB00",
                "ctu-original-blue": "#0065BD",
                "dark-black": "#0A0A0A",
                "dark-black-hover": "#1F1F1F",
                "white-lightest": "#F9F9F9",
                "neutral-light": "#eeeeee",
                "neutral-dark": "#161616"
            },
  	}
  },
  plugins: [require("tailwindcss-animate")],
}

