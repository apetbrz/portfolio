<script>
    let props = $props();
</script>

{#snippet link(display = "home", dest = "/" + display.toLowerCase())}
    <a
        href={dest}
        class="mx-2 pretty-button {props.active == display.toLowerCase()
            ? 'underline'
            : '!no-underline'}"
        draggable="false"
    >
        {display}
    </a>
{/snippet}

<div class="pretty continent !pt-4 !pb-8 sans">
    <div class="mx-auto md:w-2xl xl:w-5xl px-8">
        {@render link("Home", "/")}
        {@render link("Projects")}
    </div>
</div>
