<script>
import "../app.css";
import "@fontsource-variable/eb-garamond";
import "@fontsource-variable/geist";
import "@fontsource-variable/fira-code";

import NavMenu from "$lib/NavMenu.svelte";

let { children } = $props();
</script>

<NavMenu />
{@render children()}
