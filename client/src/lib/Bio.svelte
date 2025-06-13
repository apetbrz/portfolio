<script>
</script>

<div
  class="my-8 mx-auto p-8 bg-stone-700 md:rounded-lg md:w-[32rem] justify-center drop-shadow-md shadow-stone-900/50 text-shadow-stone-900 text-shadow-md/50 flex space-y-2 flex-col"
>
  <div class="flex space-x-[1ch]">
    <p class="">i'm</p>
    <div class="overflow-hidden flex space-x-[1ch]">
      <p class="text-nowrap titles">
        a software developer, a back-end engineer, a tinkerer, a systems
        programmer, a nerd, a tutor, a supporter, a rustacean, an arch user btw,
      </p>
      <p class="text-nowrap titles">
        a software developer, a back-end engineer, a tinkerer, a systems
        programmer, a nerd, a tutor, a supporter, a rustacean, an arch user btw,
      </p>
    </div>
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
