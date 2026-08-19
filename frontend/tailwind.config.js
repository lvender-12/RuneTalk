/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        background: "#09080e",
        foreground: "#f8fafc",
        sidebar: {
          DEFAULT: "#0f0c1a",
          border: "#241d3c",
          accent: "#1e1735",
          foreground: "#ded7e8",
        },
        card: {
          DEFAULT: "#141024",
          foreground: "#f8fafc",
          border: "#2a2245",
        },
        popover: {
          DEFAULT: "#18132c",
          foreground: "#f8fafc",
          border: "#342a55",
        },
        primary: {
          DEFAULT: "#d4af37",
          hover: "#e6c354",
          foreground: "#09080e",
          subtle: "rgba(212, 175, 55, 0.12)",
        },
        secondary: {
          DEFAULT: "#8b5cf6",
          hover: "#9d74f8",
          foreground: "#ffffff",
          subtle: "rgba(139, 92, 246, 0.14)",
        },
        muted: {
          DEFAULT: "#1b1630",
          foreground: "#94a3b8",
        },
        accent: {
          DEFAULT: "#06b6d4",
          foreground: "#ffffff",
          subtle: "rgba(6, 182, 212, 0.14)",
        },
        destructive: {
          DEFAULT: "#ef4444",
          foreground: "#ffffff",
          subtle: "rgba(239, 68, 68, 0.14)",
        },
        border: "#282046",
        input: {
          DEFAULT: "#120e22",
          border: "#2c234d",
          focus: "#d4af37",
        },
        rune: {
          gold: "#d4af37",
          amber: "#f59e0b",
          amethyst: "#a855f7",
          cyan: "#06b6d4",
          crimson: "#f43f5e",
          emerald: "#10b981",
          obsidian: "#09080e",
        },
      },
      fontFamily: {
        display: ["Cinzel", "serif"],
        heading: ["Plus Jakarta Sans", "sans-serif"],
        sans: ["Inter", "Plus Jakarta Sans", "sans-serif"],
        mono: ["JetBrains Mono", "monospace"],
      },
      boxShadow: {
        rune: "0 0 25px -5px rgba(212, 175, 55, 0.25)",
        amethyst: "0 0 25px -5px rgba(168, 85, 247, 0.25)",
        cyan: "0 0 25px -5px rgba(6, 182, 212, 0.25)",
        glow: "0 0 15px rgba(212, 175, 55, 0.35)",
        card: "0 4px 20px -2px rgba(0, 0, 0, 0.5)",
      },
      animation: {
        "pulse-slow": "pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        "fade-in": "fadeIn 0.2s ease-out forwards",
        "slide-up": "slideUp 0.25s cubic-bezier(0.16, 1, 0.3, 1) forwards",
        "scale-in": "scaleIn 0.2s cubic-bezier(0.16, 1, 0.3, 1) forwards",
      },
      keyframes: {
        fadeIn: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
        slideUp: {
          "0%": { opacity: "0", transform: "translateY(8px)" },
          "100%": { opacity: "1", transform: "translateY(0)" },
        },
        scaleIn: {
          "0%": { opacity: "0", transform: "scale(0.96)" },
          "100%": { opacity: "1", transform: "scale(1)" },
        },
      },
    },
  },
  plugins: [],
};
