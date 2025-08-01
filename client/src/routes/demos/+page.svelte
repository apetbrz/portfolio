<script>
    import NavBar from "$lib/NavBar.svelte";
</script>

<main
    class="mx-auto overflow-hidden text-stone-100 font-mono lg:max-w-4xl md:max-w-2xl bg-scenic-image md:inset-shadow-sm/50"
>
    <div class="pretty continent page-head">
        <h1 class="!mb-0">Projects and Demos</h1>
    </div>

    <NavBar active="demos" />

    <div class="pretty grid gap-8 grid-cols-2 float-large">
        <div class="island">
            <a href="https://github.com/apetbrz/nclbt" class="underline">
                nclbt
            </a>
            <p>A simple command-line utility for budget assistance.</p>
        </div>
        <div class="island">
            <a href="https://relics.apetbrz.dev/">QuickRelics</a>
            <p>A web utility that parses a public API for the game Warframe.</p>
        </div>
    </div>

    <div class="h-[64rem]"></div>
</main>

<style lang="postcss">
    @reference "tailwindcss";
    .bg-scenic-image {
        background-image: url("$lib/assets/bg-scenic-image.jpg");
        background-size: cover;
        background-repeat: no-repeat;
        background-position-y: -24rem;
    }
</style>
