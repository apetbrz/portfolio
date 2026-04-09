<script>
import ThemeSelector from "$lib/ThemeSelector.svelte";
let props = $props();
let open = $state(false);

$effect(() => {
	let anim = open? "arrow-anim-open" : "arrow-anim-close";

	let els = document.getElementsByClassName(anim)
	console.table(els)
	for(const el in els){
		els.item(el)?.beginElement();
	}
})

let tray_timeout_id;
const tray_timeout_ms = 3_000
const tray_icon_animation_dur = "0.2s"

let handle_tray_toggle = () => {
	reset_tray_timeout()
	open = !open
}
let reset_tray_timeout = () => {
	if(tray_timeout_id) clearTimeout(tray_timeout_id)
}
let handle_tray_timeout = () => {
	tray_timeout_id = setTimeout(() => {open = false}, tray_timeout_ms)
}

</script>

{#snippet link(display = "DEFAULT_FIXME", dest = "/" + display.toLowerCase())}
	<a
		href={dest}
		class="pretty-button smooth-theme-change px-8! h-16 content-center {props.active == display.toLowerCase()
			? 'underline'
			: 'no-underline!'}"
		draggable="false"
	>
		{display}
	</a>
{/snippet}

<div
	on:mouseenter={reset_tray_timeout}
	on:mouseleave={handle_tray_timeout}
	on:focusout={(event) => {
		if (
			this.contains(event.relatedTarget) ||
			!document.hasFocus()
		) {
			return;
		}
		open = false;
	}}
	class="fixed w-64 top-0 bottom-0 min-h-[100vh] transition-all duration-300 z-10 {open ? 'left-0':'-left-48'}"
>
	<button
		on:click={handle_tray_toggle}
		class="pretty-button-nofocus absolute left-48 h-16 w-16"
		tabindex="-1"
	>
		<svg class="mx-auto smooth-theme-change" xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="24" height="24" viewBox="0 0 24 24">
			<line x1="2" y1="6" x2="22" y2="6" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="2">
				<animate class="arrow-anim-open" attributeName="x1" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="4" />
				<animate class="arrow-anim-open" attributeName="x2" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="20" />
				<animate class="arrow-anim-close" attributeName="x1" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="2" />
				<animate class="arrow-anim-close" attributeName="x2" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="22" />

				<animate class="arrow-anim-open" attributeName="y1" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="4" />
				<animate class="arrow-anim-close" attributeName="y1" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="6" />
				<animate class="arrow-anim-open" attributeName="y2" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="20" />
				<animate class="arrow-anim-close" attributeName="y2" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="6" />
			</line>
			<line x1="2" y1="12" x2="22" y2="12" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="2">
				<animate class="arrow-anim-open" attributeName="x1" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="12" />
				<animate class="arrow-anim-open" attributeName="x2" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="12" />
				<animate class="arrow-anim-close" attributeName="x1" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="2" />
				<animate class="arrow-anim-close" attributeName="x2" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="22" />
			</line>
			<line x1="2" y1="18" x2="22" y2="18" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="2">
				<animate class="arrow-anim-open" attributeName="x1" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="4" />
				<animate class="arrow-anim-open" attributeName="x2" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="20" />
				<animate class="arrow-anim-close" attributeName="x1" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="2" />
				<animate class="arrow-anim-close" attributeName="x2" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="22" />

				<animate class="arrow-anim-open" attributeName="y1" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="20" />
				<animate class="arrow-anim-close" attributeName="y1" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="18" />
				<animate class="arrow-anim-open" attributeName="y2" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="4" />
				<animate class="arrow-anim-close" attributeName="y2" dur={tray_icon_animation_dur} fill="freeze" begin="indefinite" to="18" />
			</line>
		</svg>
	</button>

	<div class="pretty sans flex flex-col py-4 bg-grv-bg2 h-full relative transition-all duration-300 w-48" tabindex="-1" inert={!open}>
		{@render link("Home", "/")}
		{@render link("Projects")}
		{@render link("Blog")}
		<ThemeSelector />
	</div>
</div>
