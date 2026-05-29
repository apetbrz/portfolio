<script>
import { browser } from "$app/environment";
let dark_mode = $state(true);
if (browser) {
	let classList = document.documentElement.classList;
	$effect(() => {
		classList.toggle(
			"dark",
			dark_mode
		);
	});
}
</script>

<button
	class="bg-none text-2xl py-2 absolute not-md:right-0 outline-0 focus:outline-0 focus:text-grv-fg0 text-fg4"
	role="switch"
	aria-checked={dark_mode}
	aria-labelledby={`theme-switch`}
	onclick={() => (dark_mode = !dark_mode)}
>
	🌔︎
</button>
