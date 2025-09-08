import topLevelAwait from 'vite-plugin-top-level-await'
import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [tailwindcss(), sveltekit(), topLevelAwait()],
    build: {
        outDir: "../dist",
        emptyOutDir: true,
    },
});
