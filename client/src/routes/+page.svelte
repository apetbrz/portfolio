<script>
    import Marquee from "$lib/Marquee.svelte";
    import NavBar from "$lib/NavBar.svelte";
</script>

<main
    class="mx-auto overflow-hidden text-stone-100 w-full bg-scenic-image bg-bottom h-min flex flex-col"
>
    <div class="pretty continent flex flex-row justify-center items-end">
        <div class="w-min md:w-max page-head">
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
        class="flex flex-col flex-wrap gap-4 items-center py-4 md:bg-stone-700 md:mx-auto md:w-2xl xl:w-5xl"
    >
        <div class="pretty sans !w-full py-4 px-8 not-md:bg-stone-700 xl:px-16">
            <Marquee
                static="I'm"
                text="a software developer, a systems engineer, a tinkerer, a back-end specialist, a nerd, a tutor, a supporter, a rustacean, an arch user btw,"
            />
            <p>
                Currently deep in the worlds of
                <bold>Linux</bold>, <bold>Rust</bold>, <bold>C</bold>,
                <bold>JavaScript</bold>, and <bold>shell scripting</bold>.
            </p>
            <div class="mx-auto w-max">
                <a href="/projects" class="pretty-button" draggable="false">
                    Check out some things I'm working on!
                </a>
            </div>
        </div>
        <div class="pretty sans p-4 not-md:bg-stone-700 flex flex-row gap-8">
            <a
                href="https://github.com/apetbrz"
                class="pretty-button bg-stone-600 !no-underline"
                draggable="false"
            >
                GitHub
            </a>
            <a
                href="https://www.linkedin.com/in/arthur-t-petroff/"
                class="pretty-button bg-stone-600 !no-underline"
                draggable="false"
            >
                LinkedIn
            </a>
            <a
                href="/resume.pdf"
                class="pretty-button bg-stone-600 !no-underline"
                draggable="false"
            >
                Resume (pdf)
            </a>
        </div>
        <div class="pretty sans p-4 not-md:bg-stone-700">
            <a
                href="mailto:apetbrz@gmail.com"
                class="pretty-button bg-stone-600 !no-underline"
                draggable="false"
            >
                Email me!
            </a>
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
