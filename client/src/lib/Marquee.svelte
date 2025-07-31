<script>
    let props = $props();
</script>

<div class="pretty flex space-x-[1ch]">
    <p class="">{props.static}</p>
    <div class="overflow-hidden flex space-x-[1ch]">
        <p class="text-nowrap titles">
            {props.text}
        </p>
        <p class="text-nowrap titles">
            {props.text}
        </p>
    </div>
</div>

<style lang="postcss">
    @reference "tailwindcss";

    .titles {
        animation: 30s slide-titles linear infinite 2s;
    }
    @keyframes slide-titles {
        0% {
            transform: translateX(0);
        }
        100% {
            transform: translateX(calc(-100% - 1ch));
        }
    }
</style>
