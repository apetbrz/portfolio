import { error } from '@sveltejs/kit';

// export const prerender = true;
// export const csr = true;
export const ssr = false;

export const load = async ({ fetch, params }) => {
	const res = await fetch(`/blog/content/${params.slug}`)
	const data = await res.json()

	console.table(data);

	if (!data) {
		error(404, "Can't find content");
	}

	return data;
}
