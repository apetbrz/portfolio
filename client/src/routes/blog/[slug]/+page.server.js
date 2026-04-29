import { render } from 'svelte/server';
import { error } from '@sveltejs/kit';

import { readFileSync } from 'node:fs';

export const prerender = true;
// export const csr = true;
// export const ssr = true;

export const load = (async ({ params }) => {
	const modules = import.meta.glob("$lib/blog/*/main.svx");

	const contentModule = modules[slugToPath(params.slug)];

	if (!contentModule) {
		error(404, "Can't find content");
	}

	const component = await contentModule().then();
	const metadata = component.metadata

	console.log("comp?")
	console.table(component)
	console.log({props: metadata})
	const data = render(component.default, {
		props: metadata
	})
	console.log("data")
	console.log(data)
	console.table(data)
	console.log("metadata")
	console.table(metadata)

	return { data, metadata };
})

export const entries = async () => {
	const modules = import.meta.glob("$lib/blog/*/main.svx");

	const entries = Object.keys(modules).map((path) => {
		return { slug: pathToSlug(path) };
	});

	return entries;
};

function pathToSlug(path) {
	return path.replace("/src/lib/blog/", "").replace("/main.svx", "");
}

function slugToPath(slug) {
	return `/src/lib/blog/${slug}/main.svx`;
}
