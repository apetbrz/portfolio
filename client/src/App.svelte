<script>
    import Counter from "./lib/Counter.svelte";
</script>

<main
    class="mx-auto overflow-hidden text-stone-200 font-mono lg:max-w-4xl md:max-w-2xl"
>
    <div
        class="px-16 py-8 bg-stone-700 my-20 md:rounded-lg md:max-w-max mx-auto justify-center"
    >
        <p class="text-sm italic">hi, i'm</p>
        <h1 class="text-4xl font-extrabold">Arthur_Petroff</h1>
    </div>

    <div class="w-full p-8 border-y-2 border-stone-800">
        <p>this website is still in development! check back later &lt;3</p>
    </div>

    <div class="p-4 mx-auto w-48">
        <Counter />
    </div>
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>
