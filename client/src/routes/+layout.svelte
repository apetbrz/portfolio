<script>
import "../app.css";
import "@fontsource-variable/eb-garamond";
import "@fontsource-variable/geist";
import "@fontsource-variable/fira-code";

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

<div class="h-screen max-w-screen flex gap-4 xl:gap-8 not-md:flex-col md:justify-center md:items-center px-4 xl:px-8 md:max-h-screen">
	<aside class="pretty relative not-md:w-full">
		<ThemeSelector />
		<h2 class="w-max mx-auto">
			apetbrz.dev<span>
				<a href="/{page.url.pathname.split("/")[1]}" class="md:not-xl:block md:not-xl:px-1 pretty-link p-0! font-bold!">
					/{page.url.pathname.split("/")[1]}
				</a>
			</span>
		</h2>
		<div class="flex gap-2 md:flex-col not-md:justify-center mx-auto p-4">
			{@render link("home", "/")}
			{@render link("projects")}
			{@render link("blog")}
		</div>
	</aside>
	<main class="h-min max-h-full max-w-screen not-md:pt-8 md:max-h-full md:max-w-xl xl:max-w-4xl md:px-4 xl:px-8 border-grv-bg4 not-md:border-t md:border-l md:overflow-y-scroll overscroll-none">
		{@render children()}
	</main>
</div>

<style lang="postcss">
@reference "tailwindcss";
</style>
