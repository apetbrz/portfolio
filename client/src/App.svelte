<script>
  import Counter from "./lib/Counter.svelte";
  import NameCard from "./lib/NameCard.svelte";
  import Bio from "./lib/Bio.svelte";
  import Links from "./lib/Links.svelte";
</script>

<main
  class="mx-auto overflow-hidden text-stone-200 font-mono lg:max-w-4xl md:max-w-2xl bg-scenic-image md:inset-shadow-sm/50"
>
  <NameCard />

  <Navbar />

  <Bio />

  <Links />

  <div class="p-4 mx-auto w-48">
    <Counter />
  </div>

  <div class="h-[64rem]"></div>
</main>

<style lang="postcss">
  @reference "tailwindcss";
  .bg-scenic-image {
    background-image: url("assets/bg-scenic-image.jpg");
    background-size: cover;
    background-repeat: no-repeat;
    background-position-y: -24rem;
  }
</style>
