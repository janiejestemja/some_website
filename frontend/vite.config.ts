import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  server: {
    fs: {
      allow: [ '.', '..' ],
    },
//    proxy: {
//     '/test': {
//       target: 'http://127.0.0.100:12345',
//       changeOrigin: true,
//     }
//   },
    },
  base: '/static/',
})
