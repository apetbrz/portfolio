<script>
import "../app.css";
import "@fontsource-variable/eb-garamond";
import "@fontsource-variable/geist";
import "@fontsource-variable/fira-code";

import NavMenu from "$lib/NavMenu.svelte";
import Header from "$lib/Header.svelte";

let { children } = $props();
</script>

<NavMenu />
<Header />
{@render children()}
