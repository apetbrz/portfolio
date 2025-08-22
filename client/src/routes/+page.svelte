<script>
    import Marquee from "$lib/Marquee.svelte";
    import NavBar from "$lib/NavBar.svelte";
</script>

<main
    class="mx-auto overflow-hidden text-stone-100 lg:max-w-4xl md:max-w-3xl bg-scenic-image bg-bottom md:inset-shadow-sm/50 h-min md:h-[150vh] flex flex-col"
>
    <div class="pretty continent page-head">
        <p class="!m-0 invisible md:visible">hi, I'm</p>
        <h1 class="!m-0">Arthur Taschetto Petroff</h1>
    </div>

    <NavBar active="home" />

    <div class="md:grid grid-cols-7 gap-4 w-[100%] px-8">
        <div
            class="my-8 flex h-min md:flex-col gap-8 justify-evenly col-span-2"
        >
            <img
                class="island !p-2 max-md:!max-h-[12em] !w-auto aspect-square"
                src="/assets/workpfp.jpg"
                alt="me"
            />
            <div class="island sans pretty flex flex-col max-md:!max-w-48">
                <bold class="!mt-0">Links</bold>
                <a href="https://github.com/apetbrz" class="mt-4"> github </a>
                <a href="mailto:apetbrz@gmail.com" class=""> gmail </a>
                <a href="https://www.linkedin.com/in/arthur-t-petroff/">
                    linkedin
                </a>
            </div>
        </div>
        <div
            class="md:flex col-span-5 col-start-3 space-y-4 flex-col justify-evenly h-min w-full md:p-8 mb-8"
        >
            <div class="island pretty sans md:mt-4">
                <Marquee
                    static="I'm"
                    indent="true"
                    text="a software developer, a tinkerer, a back-end engineer, a systems
                        programmer, a nerd, a tutor, a supporter, a rustacean, an arch user btw,"
                />
                <p class="!mt-0">
                    based out of Columbia, South Carolina, with a deep lifelong
                    passion for tinkering.
                </p>
                <p class="indent-4">
                    I'm dedicated to writing high quality, efficient,
                    maintainable software. Currently deep in the worlds of
                    <bold>Rust</bold>, <bold>Linux</bold>, <bold>C</bold>,
                    <bold>JavaScript</bold>, and <bold>shell scripting</bold>,
                    but excited about everything in the world of software!
                </p>
                <a href="/projects">Check out some things I've made!</a>
            </div>
        </div>
    </div>
    <div class="mt-auto self-end pretty continent px-8">
        <p>photograph: Arthur Petroff 2024</p>
    </div>
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>
