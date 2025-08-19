<script>
    import NavBar from "$lib/NavBar.svelte";
</script>

<main
    class="mx-auto overflow-hidden text-stone-100 lg:max-w-4xl md:max-w-2xl bg-scenic-image md:inset-shadow-sm/50 h-[150vh] flex flex-col"
>
    <div class="pretty continent page-head">
        <h1 class="!mb-0">Projects</h1>
    </div>

    <NavBar active="projects" />

    <div class="pretty grid gap-8 !mb-0 grid-cols-2 float-large">
        <div class="flex flex-col gap-8">
            <div class="island">
                <a href="https://github.com/apetbrz/nclbt" class="underline">
                    nclbt
                </a>
                <p>
                    A simple, in-development command-line utility for budget
                    tracking.
                </p>
            </div>
            <div class="island">
                <a href="https://github.com/apetbrz/budget-app-project"
                    >Experimental Rust Webserver</a
                >
                <p>
                    A fully custom multi-threaded web server, for studying
                    threading and the internet.
                </p>
            </div>
        </div>
        <div class="flex flex-col gap-8">
            <div class="island">
                <a href="https://relics.apetbrz.dev/">QuickRelics</a>
                <p>
                    A web utility for parsing a public API for game information.
                </p>
            </div>
            <div class="island">
                <a href="https://apetbrz.dev">apetbrz.dev</a>
                <p>This website! A SvelteKit app.</p>
            </div>
            <div class="island">
                <a href="https://github.com/apetbrz/framework-project">
                    Web Framework Performance Study
                </a>
                <p>
                    A comparison in speed and efficiency of four different web
                    back-ends, across four programming languages.
                </p>
            </div>
        </div>
    </div>
    <div class="mt-auto self-end pretty continent px-8">
        <p>photograph: Arthur Petroff 2024</p>
    </div>
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
