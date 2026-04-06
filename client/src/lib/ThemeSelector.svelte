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
	<details class="group" onclick={() => {
		let anim = this.open? "arrow-anim-close" : "arrow-anim-open";
		let el = document.getElementById(anim)
		el.beginElement();
	}}>
		<summary class="list-none px-8! h-16 content-center pretty-button-nofocus">
			<a class="my-auto no-underline!">Theme</a>
			<svg class="my-auto rotate-0 group-open:-rotate-0 transform transition-all duration-300 inline" fill="none" height="24" width="24" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24">
				<polyline points="6 9 12 15 18 9">
					<animate id="arrow-anim-close" attributeName="points" dur="0.1s" fill="freeze" begin="indefinite" to="6 9 12 15 18 9" />
					<animate id="arrow-anim-open" attributeName="points" dur="0.1s" fill="freeze" begin="indefinite" to="6 15 12 9 18 15" />
				</polyline>
			</svg>
		</summary>
		<div class="grid grid-cols-2 hover:bg-grv-bg2 transition-all duration-300">
			<!-- {@render option("System", "")} -->
			{@render option("Light", "light")}
			{@render option("Dark", "dark")}
		</div>
	</details>
</div>
