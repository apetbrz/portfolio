<script>
    import Counter from "./lib/Counter.svelte";
</script>

<main
    class="mx-auto overflow-hidden text-stone-200 font-mono lg:max-w-4xl md:max-w-2xl bg-scenic-image inset-shadow-sm/50"
>
    <div
        class="my-20 px-16 py-8 bg-stone-700 md:rounded-lg md:max-w-max mx-auto justify-center drop-shadow-md shadow-stone-900/50 text-shadow-stone-900 text-shadow-md/50"
    >
        <p class="text-sm italic">hi, i'm</p>
        <h1 class="text-4xl font-extrabold">Arthur Petroff</h1>
    </div>
    <div
        class="my-8 md:mx-8 p-8 bg-stone-700 md:rounded-lg justify-center drop-shadow-md shadow-stone-900/50 text-shadow-stone-900 text-shadow-md/50"
    >
        <p>this website is still in development! check back later &lt;3</p>
    </div>

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
