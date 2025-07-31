<script>
    import Marquee from "$lib/Marquee.svelte";
    import NavBar from "$lib/NavBar.svelte";
    import Links from "./Links.svelte";
</script>

<main
    class="mx-auto overflow-hidden text-stone-100 font-mono lg:max-w-4xl md:max-w-2xl bg-scenic-image md:inset-shadow-sm/50"
>
    <div class="pretty continent page-head">
        <p class="italic !m-0">hi, i'm</p>
        <h1 class="!m-0">Arthur Petroff</h1>
    </div>

    <NavBar active="home" />

    <div class="island-mid">
        <Marquee
            static="im"
            text="a software developer, a back-end engineer, a tinkerer, a systems
            programmer, a nerd, a tutor, a supporter, a rustacean, an arch user
            btw,"
        />
    </div>

    <Links />

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
