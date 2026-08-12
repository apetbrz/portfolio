<script>
let { data } = $props();
</script>

<svelte:head>
	<title>Arthur's Blog</title>
</svelte:head>

{#snippet post(entry)}
<li class="w-full py-4">
	<a href={"/blog/" + entry[0]} class="no-underline! focus:outline-none group">
		<div class="p-4 group-hocus:bg-grv-bg2">
			<span>
				<span class="pretty-link-plain group-hocus:text-grv-green-bright!">
					{entry[1].title}
				</span>
				- {Temporal.PlainDate.from(entry[1].date).toLocaleString()}
			</span>
			<p class="mb-0!">{entry[1].description}</p>
		</div>
	</a>
</li>
{/snippet}

<ul id="maindiv" class="flex flex-col mx-auto pretty border-y border-grv-gray-faded divide-y divide-grv-gray-faded w-lg">
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
