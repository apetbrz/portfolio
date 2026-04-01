<script>
    import ThemeSelector from "$lib/ThemeSelector.svelte";
    let props = $props();
	let open = $state(false);
</script>

{#snippet link(display = "DEFAULT_FIXME", dest = "/" + display.toLowerCase())}
    <a
        href={dest}
        class="pretty-button-nofocus h-16 content-center {props.active == display.toLowerCase()
            ? 'underline'
            : 'no-underline!'}"
        draggable="false"
    >
        {display}
    </a>
{/snippet}

<div on:mouseleave={() => open = false} class="fixed w-64 top-0 bottom-0 min-h-[100vh] transition-all duration-300 z-10 {open ? 'left-0':'-left-48'}">
	<button on:click={() => open = !open} class="pretty-button-nofocus absolute left-48 h-16 w-16">
		<svg class="mx-auto" xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="24" height="24" viewBox="0 0 24 24">
			<line x1="2" x2="22" y1="6" y2="6" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="1"></line>
			<line x1="2" x2="22" y1="12" y2="12" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="1"></line>
			<line x1="2" x2="22" y1="18" y2="18" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="1"></line>
		</svg>
	</button>
	<div class="pretty sans flex flex-col bg-grv-bg3 h-full relative transition-all duration-300 w-48">
		{@render link("Home", "/")}
		{@render link("Projects")}
		<ThemeSelector />
	</div>
</div>
