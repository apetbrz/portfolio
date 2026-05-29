<script>
let { data } = $props();
</script>

<svelte:head>
	<title>Arthur's Blog</title>
</svelte:head>

{#snippet post(entry)}
<li class="w-full py-4">
	<a href={"/blog/" + entry[0]} class="no-underline! font-normal! hover:bg-grv-bg2 focus:bg-grv-bg2 focus:outline-none outline-none">
		<div class="hover:bg-grv-bg2 focus:bg-grv-bg2">
			<span>
				<span class="pretty-link">
					{entry[1].title}
				</span>
				- {Temporal.PlainDate.from(entry[1].date).toLocaleString()}
			</span>
			<p>{entry[1].description}</p>
		</div>
	</a>
</li>
{/snippet}

<ul id="maindiv" class="flex flex-col justify-center items-center mx-auto w-md pretty divide-y divide-grv-gray-faded">
	{#if data.error}
		<h3>{data.message}</h3>
		<p>{data.error}</p>
	{:else}
		{#each data?.entries as entry}
			{@render post(entry)}
		{/each}
	{/if}
</ul>

<style lang="postcss">
@reference "tailwindcss";
</style>
