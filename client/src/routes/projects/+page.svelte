<script>
</script>

<svelte:head>
	<title>~/projects</title>
</svelte:head>

{#snippet project(name, url, desc)}
<div class="w-full">
	<a href={url} class="no-underline! focus:outline-none group">
		<div class="p-4 group-hocus:bg-grv-bg2">
			<span>
				<span class="pretty-link-plain group-hocus:font-bold group-hocus:text-grv-green-bright! group-hocus:underline">
					{name}
				</span>
			</span>
			<p class="mb-0! mt-2! font-normal">{desc}</p>
		</div>
	</a>
</div>
{/snippet}

<div id="maindiv" class="pretty flex flex-col mx-auto w-lg justify-center items-center divide-y divide-grv-gray-faded" >
	{@render project("Void Seer", "https://relics.apetbrz.dev/", "Web utility for parsing a public API for game information.")}
	{@render project("Markerbag", "/blog", "Server-side markdown pre-processor microservice for blog authoring, built in Rust for Docker Compose deployments.")}
</div>

<style lang="postcss">
@reference "tailwindcss";
</style>
