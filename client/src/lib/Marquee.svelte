<script>
let props = $props();
</script>

<div class="marquee pretty flex {props.indent ? 'pl-4!' : ''}">
	<p class="my-0! text-nowrap smooth-theme-change">{props.static}</p>
	<p class="m-0!">&nbsp;</p>
	<div class="overflow-hidden">
		<div class="w-max slide">
			<span class="text-nowrap smooth-theme-change">
				{props.text}
			</span>
			<span class="text-nowrap smooth-theme-change">
				{props.text}
			</span>
		</div>
	</div>
</div>

<style lang="postcss">
@reference "tailwindcss";

.slide {
	animation: 20s slide linear infinite 2s;
}
@keyframes slide {
	0% {
		transform: translateX(0);
	}
	100% {
		transform: translateX(-50%);
	}
}
</style>
