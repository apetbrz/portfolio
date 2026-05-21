import { error } from '@sveltejs/kit';

// export const prerender = true;
// export const csr = true;
export const ssr = false;

export const load = async ({ fetch }) => {
	const res = await fetch(`/blog/metadata`)
	const data = await res.json()

	if (!data) {
		error(404, "Can't find content");
	}

	return { entries: data };
}
