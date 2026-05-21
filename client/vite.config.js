import { enhancedImages } from '@sveltejs/enhanced-img';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';
import topLevelAwait from 'vite-plugin-top-level-await'

export default defineConfig({
	plugins: [tailwindcss(), enhancedImages(), sveltekit(), topLevelAwait()],
	build: {
		outDir: "../dist",
		emptyOutDir: true,
	},
	server: {
		proxy: {
			'/r': 'http://localhost:3000',
			'/blog/content': 'http://localhost:3000',
			'/blog/assets': 'http://localhost:3000',
			'/blog/metadata': 'http://localhost:3000'
		}
	}
});
