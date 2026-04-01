<script>
    import { browser } from "$app/environment";
    let theme = $state("dark");
    if (browser) {
        let classList = document.documentElement.classList;
        $effect(() => {
            classList.toggle(
                "dark",
                theme === "dark" ||
                    (!theme &&
                        window.matchMedia("(prefers-color-scheme: dark)")
                            .matches),
            );
        });
    }
</script>

{#snippet option(name = "", themesetting = "")}
    <button
        onclick={() => (theme = themesetting)}
        class="h-12 w-full active:bg-grv-bg0 transition-all duration-300{theme == themesetting
            ? ' font-bold'
            : ''}"
        draggable="false"
        tabindex="-1"
    >
        {name}
    </button>
{/snippet}

<div class="pretty sans">
	<details class="group h-16">
		<summary class="list-none h-full content-center pretty-button-nofocus">
			<a class="inline no-underline!">Theme</a>
			<!-- <svg class="inline" fill="currentColor" width="24" height="24" viewBox="0 0 24 24"> -->
			<!-- 	<path d="M12 8a4 4 0 1 1-8 0 4 4 0 0 1 8 0M8 0a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 0m0 13a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 13m8-5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2a.5.5 0 0 1 .5.5M3 8a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2A.5.5 0 0 1 3 8m10.657-5.657a.5.5 0 0 1 0 .707l-1.414 1.415a.5.5 0 1 1-.707-.708l1.414-1.414a.5.5 0 0 1 .707 0m-9.193 9.193a.5.5 0 0 1 0 .707L3.05 13.657a.5.5 0 0 1-.707-.707l1.414-1.414a.5.5 0 0 1 .707 0m9.193 2.121a.5.5 0 0 1-.707 0l-1.414-1.414a.5.5 0 0 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .707M4.464 4.465a.5.5 0 0 1-.707 0L2.343 3.05a.5.5 0 1 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .708"/> -->
			<!-- </svg> -->
			<svg class="rotate-0 group-open:-rotate-180 transform -translate-y-0.5 transition-all duration-300 inline" fill="none" height="24" width="24" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24">
				<polyline points="6 9 12 15 18 9"></polyline>
			</svg>
		</summary>
		<div class="grid grid-cols-2 hover:bg-grv-bg2 transition-all duration-300">
			<!-- {@render option("System", "")} -->
			{@render option("Light", "light")}
			{@render option("Dark", "dark")}
		</div>
	</details>
</div>
