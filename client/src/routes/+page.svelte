<script>
    import Marquee from "$lib/Marquee.svelte";
    import NavBar from "$lib/NavBar.svelte";

    //TODO: HOW TO MAKE tabindex DYNAMIC WITH SIZE? for maindiv ordering (content -> links on <md, links -> content on >=md)
</script>

<main class="mx-auto overflow-hidden h-min flex flex-col min-h-[100vh]">
    <div class="pretty flex flex-row justify-center items-end">
        <div class="w-min md:w-max page-head">
            <p class="!m-0 invisible md:visible">hi, I'm</p>
            <h1 class="!m-0">Arthur Taschetto</h1>
        </div>
    </div>

    <NavBar active="home" />

    <div
        id="maindiv"
        class="grid md:grid-cols-[1fr_5fr] grid-cols-1 w-full grow"
    >
        <div
            class="md:col-start-2 flex flex-col flex-wrap gap-4 min-w-0 p-8 pt-4"
        >
            <div class="pretty sans w-4xl">
                <Marquee
                    static="I'm"
                    text="a software developer, a systems engineer, a tinkerer, a back-end specialist, a nerd, a tutor, a supporter, a rustacean, an arch user btw,"
                />
                <p>
                    Currently deep in the worlds of
                    <bold>Linux</bold>, <bold>Rust</bold>, <bold>C</bold>,
                    <bold>JavaScript</bold>, and <bold>shell scripting</bold>.
                </p>
                <div class="mx-auto">
                    <a href="/projects" class="pretty-button" draggable="false">
                        Check out some things I'm working on!
                    </a>
                </div>
            </div>
        </div>
        <div
            class="col-start-1 md:row-start-1 h-full md:w-2xs pretty sans px-8 bg-grv-bg0"
        >
            <div class="mx-auto flex flex-col flex-wrap items-center md:items-start">
                <div class="pretty"><h3>My Info</h3></div>
                <a
                    href="/assets/resume.pdf"
                    class="pretty-button !no-underline"
                    draggable="false"
                >
                    Resume
                </a>
                <div class="pretty"><h3>Contact Me!</h3></div>
                <a
                    href="https://github.com/apetbrz"
                    class="pretty-button !no-underline"
                    draggable="false"
                >
                    GitHub
                </a>
                <a
                    href="https://www.linkedin.com/in/arthur-t-petroff/"
                    class="pretty-button !no-underline"
                    draggable="false"
                >
                    LinkedIn
                </a>
                <a
                    href="mailto:apetbrz@gmail.com"
                    class="pretty-button !no-underline"
                    draggable="false"
                >
                    Email
                </a>
            </div>
        </div>
    </div>
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>
