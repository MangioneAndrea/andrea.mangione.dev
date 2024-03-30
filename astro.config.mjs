import { defineConfig } from 'astro/config'

// https://astro.build/config
import svelte from '@astrojs/svelte'
import tailwind from '@astrojs/tailwind'
import basicSsl from '@vitejs/plugin-basic-ssl'

// https://astro.build/config
export default defineConfig({
  vite: {
    plugins: [basicSsl()],
  },
  output: 'static',
  integrations: [svelte(), tailwind()],
})
