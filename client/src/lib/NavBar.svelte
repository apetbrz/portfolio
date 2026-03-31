<script>
    import ThemeSelector from "$lib/ThemeSelector.svelte";
    let props = $props();
</script>

{#snippet link(display = "DEFAULT_FIXME", dest = "/" + display.toLowerCase())}
    <a
        href={dest}
        class="mx-2 pretty-button {props.active == display.toLowerCase()
            ? 'underline'
            : 'no-underline!'}"
        draggable="false"
    >
        {display}
    </a>
{/snippet}

<div class="pretty pt-4! pb-8! sans flex flex-wrap justify-between">
    <div class="w-max px-8 content-center">
        {@render link("Home", "/")}
        {@render link("Projects")}
    </div>
    <!-- <ThemeSelector /> -->
</div>
