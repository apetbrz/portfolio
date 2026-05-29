<script>
</script>

<svelte:head>
	<title>Arthur's Projects</title>
</svelte:head>

{#snippet project(name, url, desc)}
<div class="w-full">
	<p>
		<a href={url} class="pretty-link">{name}</a>
		- {desc}
	</p>
</div>
{/snippet}

<div id="maindiv" class="pretty flex flex-col mx-auto w-lg justify-center items-center divide-y divide-grv-gray-faded" >
	{@render project("Void Seer", "https://relics.apetbrz.dev/", "Web utility for parsing a public API for game information.")}
	{@render project("Markerbag", "/blog", "Server-side markdown pre-processor microservice for blog authoring, built in Rust for Docker Compose deployments.")}
</div>

<style lang="postcss">
@reference "tailwindcss";
</style>
