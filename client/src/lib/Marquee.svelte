<script>
    let props = $props();
</script>

<div class="pretty flex {props.indent ? '!pl-4' : ''}">
    <p class="!my-0">{props.static}</p>
    <p class="!m-0">&nbsp;</p>
    <div class="overflow-hidden flex space-x-[1ch]">
        <span class="titles text-nowrap">
            {props.text}
        </span>
        <span class="titles text-nowrap">
            {props.text}
        </span>
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
