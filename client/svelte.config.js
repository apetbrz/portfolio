import adapter from '@sveltejs/adapter-static';
import { mdsvex } from 'mdsvex';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			pages: '../dist',
			assets: '../dist',
			fallback: '200.html',
			precompress: false,
			strict: true
		})
	},
};

export default config;
