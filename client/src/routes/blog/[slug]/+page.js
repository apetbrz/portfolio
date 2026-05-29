import { error } from '@sveltejs/kit';

export const load = async ({ fetch, params }) => {
	const res = await fetch(`/blog/content/${params.slug}`)
	if(res.ok) {
		const html = await res.text()
		return { html };
	}
	else {
		let message
		if(error === 404) message = "Post not found!"
		else if(error === 503) message = "Blog is currently down for maintanence"
		else message = "Blog is currently under maintanence"
		return { error: res.status, message };
	}
}
