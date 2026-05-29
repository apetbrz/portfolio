<script>
import Marquee from "$lib/Marquee.svelte";
</script>

<svelte:head>
	<title>apetbrz.dev</title>
</svelte:head>

{#snippet link(display = "DEFAULT_FIXME", dest = "/" + display.toLowerCase())}
	<a
		href={dest}
		class="pretty-link w-max"
	>
		{display}
	</a>
{/snippet}

<div id="maindiv" class="flex flex-col mx-auto w-lg pretty px-2" >
	<div class="pretty mx-auto w-max text-2xl! font-serif">
		<p class="m-0! not-md:invisible not-md:h-0">hi, I'm</p>
		<h2 class="m-0! p-2">
			Arthur Taschetto <span class="not-lg:hidden">Petroff</span>
		</h2>
	</div>
	<Marquee
		static="I'm a(n)"
		text="software developer, systems engineer, tinkerer, back-end specialist, nerd, tutor, supporter, rustacean, arch user btw, minimalist, maximalist, "
	/>
	<p class="my-0!">
		and I'm happy to share my little corner of the internet with you.
		<!-- or -->
		<!-- <a href="/blog" class="pretty-button box-decoration-clone p-1! -m-1!" draggable="false">take a peek at what I've been up to recently.</a> -->
	</p>
	<div class="pretty text-center">
		<h3 class="">Contact Me</h3>
	</div>
	<div class="flex flex-row flex-wrap justify-center mx-auto w-sm md:w-2xl pretty sans">
		{@render link("Resume", "/assets/resume.pdf")}
		{@render link("Github", "https://github.com/apetbrz")}
		{@render link("LinkedIn", "https://www.linkedin.com/in/arthur-t-petroff/")}
		{@render link("Email", "mailto:apetbrz@gmail.com")}
	</div>
</div>

<style lang="postcss">
@reference "tailwindcss";
</style>
