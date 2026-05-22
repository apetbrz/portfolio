<script>
let { params, data } = $props();

let post_meta = data.entries?.find((element) => element[0] === params.slug)[1]
</script>

<svelte:head>
	<title>{post_meta.title} - Arthur's Blog</title>
</svelte:head>

<main class="mx-auto overflow-hidden text-stone-100 w-full bg-scenic-image bg-bottom h-min flex flex-col">

	<div class="pretty sans text-center page-head">
		{#if data.error}
			<h2>Oops!</h2>
		{:else}
			<h2>{ post_meta?.title }</h2>
		{/if}
	</div>

	<div id="maindiv" class="text-left w-full md:w-lg lg:w-4xl md:mx-auto pretty" >
		{#if data.error}
			<h3>{data.message}</h3>
			<p>{data.error}</p>
		{:else}
			{@html data.html }
		{/if}
	</div>

</main>

<style lang="postcss">
@reference "tailwindcss";
</style>
