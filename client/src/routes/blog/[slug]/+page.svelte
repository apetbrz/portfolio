<script>
let { params, data } = $props();

let post_meta = data.entries.find((element) => element[0] === params.slug)[1]
</script>

<svelte:head>
	<title>{post_meta.title} - Arthur's Blog</title>
</svelte:head>

<main class="mx-auto overflow-hidden text-stone-100 w-full bg-scenic-image bg-bottom h-min flex flex-col">

	<div class="pretty sans text-center page-head">
		<h2>{ post_meta.title }</h2>
	</div>

	<div id="maindiv" class="text-left w-full md:w-lg lg:w-4xl md:mx-auto pretty" >
		{@html data.html }
	</div>

</main>

<style lang="postcss">
@reference "tailwindcss";
</style>
