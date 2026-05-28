<script>
let { params, data } = $props();

let post_meta = data.entries?.find((element) => element[0] === params.slug)[1]
</script>

<svelte:head>
	<title>{post_meta.title} - Arthur's Blog</title>
</svelte:head>

<div id="maindiv" class="text-left w-full md:w-lg lg:w-4xl md:mx-auto pretty" >
	<div class="pretty sans text-center w-full my-16">
		{#if data.error}
			<h1>Oops!</h1>
		{:else}
			<h1>{ post_meta?.title }</h1>
		{/if}
	</div>

	{#if data.error}
		<h3>{data.message}</h3>
		<p>{data.error}</p>
	{:else}
		{@html data.html }
	{/if}
</div>

<style lang="postcss">
@reference "tailwindcss";
</style>
