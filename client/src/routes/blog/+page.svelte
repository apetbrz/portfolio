<script>
let { data } = $props();
</script>

<svelte:head>
	<title>~/blog</title>
</svelte:head>

{#snippet post(entry)}
<div class="w-full">
	<a href={"/blog/" + entry[0]} class="no-underline! focus:outline-none group">
		<div class="p-4 group-hocus:bg-grv-bg2">
			<span>
				<span class="pretty-link group-hocus:text-grv-green-bright!">
					{entry[1].title}
				</span>
				{Temporal.PlainDate.from(entry[1].date)?.toLocaleString()}
			</span>
			<p class="mb-0! mt-2! font-normal">{entry[1].description}</p>
		</div>
	</a>
</div>
{/snippet}

<div id="maindiv" class="pretty flex flex-col mx-auto w-lg justify-center items-center divide-y divide-grv-gray-faded" >
	{#if data.error}
		{@render post(["", { title: data.message, description: data.error } ])}
	{:else}
		{#each data?.entries as entry}
			{@render post(entry)}
		{/each}
	{/if}
</div>

<style lang="postcss">
@reference "tailwindcss";
</style>
