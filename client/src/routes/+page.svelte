<script>
    import Marquee from "$lib/Marquee.svelte";
    import NavBar from "$lib/NavBar.svelte";
</script>

<main
    class="mx-auto overflow-hidden text-stone-100 w-full bg-scenic-image bg-bottom h-min flex flex-col"
>
    <div class="pretty continent flex flex-row justify-between items-end">
        <div class="max-w-max page-head">
            <p class="!m-0 invisible md:visible">hi, I'm</p>
            <h1 class="!m-0">Arthur Taschetto Petroff</h1>
        </div>
        <img
            class="w-32 aspect-square mr-16 !mb-0 rounded-xl"
            src="/assets/workpfp.jpg"
            alt="me"
        />
    </div>

    <NavBar active="home" />

    <div
        class="flex flex-col flex-wrap items-center py-8 md:bg-stone-700 md:mx-auto md:w-2xl xl:w-5xl"
    >
        <div
            class="pretty sans !w-full p-4 not-md:my-4 not-md:bg-stone-700 xl:px-16"
        >
            <Marquee
                static="I'm"
                text="a software developer, a systems engineer, a tinkerer, a back-end specialist, a nerd, a tutor, a supporter, a rustacean, an arch user btw,"
            />
            <p>
                Currently deep in the worlds of
                <bold>Linux</bold>, <bold>Rust</bold>, <bold>C</bold>,
                <bold>JavaScript</bold>, and <bold>shell scripting</bold>.
            </p>
            <a href="/projects">Check out some things I'm working on!</a>
        </div>
        <div class="pretty sans p-4 not-md:my-4 not-md:bg-stone-700">
            <a href="https://github.com/apetbrz" class="mt-4">GitHub</a>
            <a href="mailto:apetbrz@gmail.com" class="">Gmail</a>
            <a href="https://www.linkedin.com/in/arthur-t-petroff/">LinkedIn</a>
        </div>
        <div class="h-128"></div>
    </div>
    <div class="mt-auto self-end pretty continent px-8">
        <p>photograph: Arthur Petroff 2024</p>
    </div>
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>
