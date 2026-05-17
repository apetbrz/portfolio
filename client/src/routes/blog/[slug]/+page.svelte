<script>
let { data } = $props();
</script>

<svelte:head>
	<title>Arthur's Blog - </title>
</svelte:head>

<main class="mx-auto overflow-hidden text-stone-100 w-full bg-scenic-image bg-bottom h-min flex flex-col">

	<div class="pretty sans text-center page-head">
		<h2>{ data.metadata.title }</h2>
	</div>

	<div id="maindiv" class="text-left w-full md:w-lg lg:w-4xl md:mx-auto pretty" >
		{@html data.html }
	</div>

</main>

<style lang="postcss">
@reference "tailwindcss";
</style>
