<script>
    import NavBar from "$lib/NavBar.svelte";
</script>

<main
    class="mx-auto overflow-hidden text-stone-100 w-full bg-scenic-image bg-bottom h-min flex flex-col"
>
    <div class="pretty flex flex-row justify-center items-end">
        <div class="w-min md:w-max page-head">
            <h1 class="!m-0">Projects</h1>
        </div>
    </div>

    <NavBar active="projects" />

    <div
		id="maindiv"
        class="flex flex-col md:flex-row flex-wrap justify-center items-stretch py-8 md:mx-auto w-full md:max-w-4xl xl:w-5xl bg-grv-bg0"
    >
        <div class="pretty sans hover:bg-grv-bg2 w-sm p-4 xl:px-16">
            <a href="https://github.com/apetbrz/nlbt" class="underline">
                nlbt
            </a>
            <p>
                A simple, in-development command-line utility for budget
                tracking.
            </p>
        </div>
        <div class="pretty sans hover:bg-grv-bg2 w-sm p-4 xl:px-16">
            <a href="https://relics.apetbrz.dev/">Void Seer</a>
            <p>A web utility for parsing a public API for game information.</p>
        </div>
        <div class="pretty sans hover:bg-grv-bg2 w-sm p-4 xl:px-16">
            <a href="https://apetbrz.dev">apetbrz.dev</a>
            <p>This website! A SvelteKit app.</p>
        </div>
        <div class="pretty sans hover:bg-grv-bg2 w-sm p-4 xl:px-16">
            <a href="https://github.com/apetbrz/framework-project">
                Web Framework Performance Study
            </a>
            <p>
                A comparison in speed and efficiency of four different web
                back-ends, across four programming languages.
            </p>
        </div>
        <div class="pretty sans hover:bg-grv-bg2 w-sm p-4 xl:px-16">
            <a href="https://github.com/apetbrz/budget-app-project"
                >Experimental Rust Webserver</a
            >
            <p>
                A fully custom multi-threaded web server, for studying threading
                and the internet.
            </p>
        </div>
    </div>
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>
