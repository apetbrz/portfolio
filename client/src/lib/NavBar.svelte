<script>
    let props = $props();
</script>

{#snippet link(display = "home", dest = "/" + display)}
    <a
        href={dest}
        class="hover:text-stone-400 rounded-xs focus:outline-2 focus:outline-stone-100 focus:outline-offset-2 {props.active ==
        display
            ? 'underline'
            : '!no-underline'}"
    >
        {display}
    </a>
{/snippet}

<div class="pretty continent px-16 py-4 md:py-8 sans">
    {@render link("home", "/")}
    {@render link("projects")}
</div>
