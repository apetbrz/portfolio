<script>
    let props = $props();
</script>

<div class="pretty flex {props.indent ? '!pl-4' : ''}">
    <p class="!my-0">{props.static}</p>
    <p class="!m-0">&nbsp;</p>
    <div class="overflow-hidden flex">
        <span class="slide text-nowrap">
            {props.text}&nbsp;
        </span>
        <span class="slide text-nowrap">
            {props.text}&nbsp;
        </span>
    </div>
</div>

<style lang="postcss">
    @reference "tailwindcss";

    .slide {
        animation: 30s slide linear infinite 2s;
    }
    @keyframes slide {
        0% {
            transform: translateX(0);
        }
        100% {
            transform: translateX(-100%);
        }
    }
</style>
