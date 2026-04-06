import topLevelAwait from 'vite-plugin-top-level-await'
import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { enhancedImages } from '@sveltejs/enhanced-img';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [tailwindcss(), enhancedImages(), sveltekit(), topLevelAwait()],
    build: {
        outDir: "../dist",
        emptyOutDir: true,
    },
    server: {
        proxy: {
            '/*': 'http://localhost:3000'
        }
    }
});
