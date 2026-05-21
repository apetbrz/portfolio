<script>
let { data } = $props();
</script>

{#snippet post(entry)}
	<a href={"/blog/" + entry[0]}
		class="w-xl pretty sans pretty-button-nofocus">
		<div class="sm:px-8">
			<h3>{entry[1].title}</h3>
			<p>{Temporal.PlainDate.from(entry[1].date).toLocaleString()}</p>
			<p>{entry[1].author}</p>
			<p>{entry[1].description}</p>
			<p>tags: {#each entry[1].tags as tag} <span class="italic">{tag + " "}</span> {/each}</p>
		</div>
	</a>
{/snippet}

<svelte:head>
	<title>Arthur's Blog</title>
</svelte:head>

<main class="mx-auto overflow-hidden text-stone-100 w-full bg-scenic-image bg-bottom h-min flex flex-col">

	<div class="pretty sans text-center page-head">
		<h2>Blog</h2>
	</div>

	<div id="maindiv" class="flex flex-col justify-center items-center w-full" >
		{#each data.entries as entry}
			{@render post(entry)}
		{/each}
	</div>

</main>

<style lang="postcss">
@reference "tailwindcss";
</style>
