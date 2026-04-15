import adapter from '@sveltejs/adapter-static';
import { mdsvex } from 'mdsvex';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    kit: {
        adapter: adapter({
            pages: '../dist',
            assets: '../dist',
            fallback: 'index.html',
            precompress: true,
            strict: true
        })
    },
    preprocess: [mdsvex()],
    extensions: ['.svelte', '.svx']
};

export default config;
