<script>
import { page } from "$app/state";
import ThemeSelector from "$lib/ThemeSelector.svelte";

let { children } = $props();
</script>

{#snippet link(display = "DEFAULT_FIXME", dest = "/" + display.toLowerCase())}
	<a
		href={dest}
		class="pretty-link w-max"
	>
		{display}
	</a>
{/snippet}

<div class="h-screen max-w-screen flex not-md:flex-col md:justify-center md:items-center px-4 xl:px-8 md:max-h-screen">
	<aside class="pretty relative not-md:w-full border-grv-bg4 not-md:border-b md:border-r md:px-4 xl:px-8 not-md:pb-8">
		<ThemeSelector />
		<h2 class="w-max mx-auto">
			<a href="/" class="pretty-link text-grv-fg0! hover:text-grv-fg4! focus:text-grv-fg4! p-0! font-bold!">apetbrz.dev</a>
			<a href="/{page.url.pathname.split("/")[1]}" class="block pretty-link p-0! font-bold!">
				/{page.url.pathname.split("/")[1]}
			</a>
		</h2>
		<div class="flex gap-2 md:flex-col not-md:justify-center mx-auto px-4">
			{@render link("home", "/")}
			{@render link("projects")}
			{@render link("blog")}
		</div>
	</aside>
	<main class="h-min max-h-full max-w-screen not-md:pt-8 md:max-h-full md:max-w-xl xl:max-w-4xl md:overflow-y-scroll overscroll-none md:px-4 xl:px-8">
		{@render children()}
	</main>
</div>

<style lang="postcss">
@import '../app.css';
@reference "tailwindcss";
:root {
	--default-transition-duration: 300ms;
}
</style>
