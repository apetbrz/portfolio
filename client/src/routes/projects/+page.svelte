<script>
import NavBar from "$lib/NavBar.svelte";
import Header from "$lib/Header.svelte";
</script>

<main
	class="mx-auto overflow-hidden text-stone-100 w-full bg-scenic-image bg-bottom h-min flex flex-col"
>
	<Header />

	<div
		id="maindiv"
		class="flex flex-col md:flex-row flex-wrap justify-center items-stretch md:mx-auto w-full md:max-w-4xl xl:w-5xl"
	>
		<div class="pretty sans text-center w-full">
			<h2>Projects</h2>
		</div>
		<div class="pretty sans hover:bg-grv-bg2 w-sm p-4 xl:px-16">
			<a href="https://github.com/apetbrz/nlbt" class="underline">
				nlbt
			</a>
			<p>
				A simple, in-development command-line utility for budget
				tracking.
			</p>
		</div>
		<div class="pretty sans hover:bg-grv-bg2 w-sm p-4 xl:px-16">
			<a href="https://relics.apetbrz.dev/">Void Seer</a>
			<p>A web utility for parsing a public API for game information.</p>
		</div>
		<div class="pretty sans hover:bg-grv-bg2 w-sm p-4 xl:px-16">
			<a href="https://apetbrz.dev">apetbrz.dev</a>
			<p>This website! A SvelteKit app.</p>
		</div>
		<div class="pretty sans hover:bg-grv-bg2 w-sm p-4 xl:px-16">
			<a href="https://github.com/apetbrz/framework-project">
				Web Framework Performance Study
			</a>
			<p>
				A comparison in speed and efficiency of four different web
				back-ends, across four programming languages.
			</p>
		</div>
		<div class="pretty sans hover:bg-grv-bg2 w-sm p-4 xl:px-16">
			<a href="https://github.com/apetbrz/budget-app-project"
			>Experimental Rust Webserver</a
			>
			<p>
				A fully custom multi-threaded web server, for studying threading
				and the internet.
			</p>
		</div>
	</div>
</main>

<style lang="postcss">
@reference "tailwindcss";
</style>
