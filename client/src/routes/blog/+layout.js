import { error } from '@sveltejs/kit';

// export const prerender = true;
// export const csr = true;
export const ssr = false;

export const load = async ({ fetch }) => {
	const res = await fetch(`/blog/metadata`)
	if(res.ok) {
		const data = await res.json()
		return { entries: data };
	}
	else {
		return { error: res.status, message: "Currently under maintanence!" };
	}
}
