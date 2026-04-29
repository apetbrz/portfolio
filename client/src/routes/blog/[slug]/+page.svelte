<script>
export let data, metadata;
console.table(data)
console.table(metadata)
</script>

<svelte:head>
	<title>Arthur's Blog - </title>
</svelte:head>

<main class="mx-auto overflow-hidden text-stone-100 w-full bg-scenic-image bg-bottom h-min flex flex-col">

	<div class="pretty sans text-center page-head">
		<h2>Blog</h2>
	</div>

	<div id="maindiv" class="flex flex-col justify-center items-center w-full" >
		{data}
	</div>

</main>

<style lang="postcss">
@reference "tailwindcss";
</style>
