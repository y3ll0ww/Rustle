import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/postcss'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    tailwindcss()
  ],

  // This will refresh automatically in Docker container
  server: {
    watch: {
      usePolling: true,
      interval: 100
    },
    host: true,
    strictPort: true,
    port: 5173
  }
})
