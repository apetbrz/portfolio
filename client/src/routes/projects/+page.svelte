<script>
    import NavBar from "$lib/NavBar.svelte";
</script>

<main
    class="mx-auto overflow-hidden text-stone-100 w-full bg-scenic-image bg-bottom h-min flex flex-col"
>
    <div class="pretty continent page-head">
        <h1 class="!mb-0">Projects</h1>
    </div>

    <NavBar active="projects" />

    <div
        class="flex flex-col flex-wrap items-center py-8 md:bg-stone-700 md:mx-auto md:w-2xl xl:w-5xl"
    >
        <div
            class="pretty sans !w-full p-4 not-md:my-4 not-md:bg-stone-700 xl:px-16"
        >
            <a href="https://github.com/apetbrz/nlbt" class="underline">
                nlbt
            </a>
            <p>
                A simple, in-development command-line utility for budget
                tracking.
            </p>
        </div>
        <div
            class="pretty sans !w-full p-4 not-md:my-4 not-md:bg-stone-700 xl:px-16"
        >
            <a href="https://relics.apetbrz.dev/">Void Seer</a>
            <p>A web utility for parsing a public API for game information.</p>
        </div>
        <div
            class="pretty sans !w-full p-4 not-md:my-4 not-md:bg-stone-700 xl:px-16"
        >
            <a href="https://apetbrz.dev">apetbrz.dev</a>
            <p>This website! A SvelteKit app.</p>
        </div>
        <div
            class="pretty sans !w-full p-4 not-md:my-4 not-md:bg-stone-700 xl:px-16"
        >
            <a href="https://github.com/apetbrz/framework-project">
                Web Framework Performance Study
            </a>
            <p>
                A comparison in speed and efficiency of four different web
                back-ends, across four programming languages.
            </p>
        </div>
        <div
            class="pretty sans !w-full p-4 not-md:my-4 not-md:bg-stone-700 xl:px-16"
        >
            <a href="https://github.com/apetbrz/budget-app-project"
                >Experimental Rust Webserver</a
            >
            <p>
                A fully custom multi-threaded web server, for studying threading
                and the internet.
            </p>
        </div>
    </div>
    <div class="mt-auto self-end pretty continent px-8">
        <p>photograph: Arthur Petroff 2024</p>
    </div>
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>
