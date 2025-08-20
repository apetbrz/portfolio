<script>
    import Marquee from "$lib/Marquee.svelte";
    import NavBar from "$lib/NavBar.svelte";
</script>

<main
    class="mx-auto overflow-hidden text-stone-100 lg:max-w-4xl md:max-w-2xl bg-scenic-image md:inset-shadow-sm/50 h-[150vh] flex flex-col"
>
    <div class="pretty continent page-head">
        <p class="!m-0">hi, I'm</p>
        <h1 class="!m-0">Arthur Petroff</h1>
    </div>

    <NavBar active="home" />

    <div class="lg:grid lg:grid-cols-4 gap-4 w-[100%] px-8">
        <div class="lg:flex flex-col justify-evenly">
            <img
                class="island-mid !mx-auto lg:!mx-0 !p-2 h-min w-full aspect-square"
                src="/workpfp.jpg"
                alt="me"
            />
            <div class="island sans pretty block">
                <p class="!mt-0 w-full block">Reach out to me!</p>
                <a href="mailto:apetbrz@gmail.com" class="w-full block">gmail</a
                >
                <a href="https://github.com/apetbrz" class="w-full block"
                    >github</a
                >
                <a
                    href="https://www.linkedin.com/in/arthur-t-petroff/"
                    class="block"
                >
                    linkedin
                </a>
            </div>
        </div>
        <div
            class="lg:flex col-span-3 space-y-4 flex-col justify-evenly h-min w-full p-8"
        >
            <div class="island pretty sans mt-4">
                <Marquee
                    static="I'm"
                    indent="4"
                    text="a software developer, a tinkerer, a back-end engineer, a systems
            programmer, a nerd, a tutor, a supporter, a rustacean, an arch user btw,"
                />
                <p class="!mt-0">
                    based out of Columbia, South Carolina, with a lifelong
                    passion for knowledge and growth.
                </p>
                <p class="indent-4">
                    Currently deep in the worlds of <bold>Rust</bold>,
                    <bold>Linux</bold>, <bold>JavaScript</bold>, and
                    <bold>shell scripting</bold>, but happy and excited to talk
                    about anything in the world of software!
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
    .bg-scenic-image {
        background-image: url("$lib/assets/bg-scenic-image.jpg");
        background-size: cover;
        background-repeat: no-repeat;
        background-position-y: -24rem;
    }
</style>
