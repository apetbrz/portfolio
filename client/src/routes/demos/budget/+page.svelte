<script>
    import NavBar from "$lib/NavBar.svelte";
    import Demo from "./Demo.svelte";
</script>

<main
    class="mx-auto overflow-hidden font-mono lg:max-w-4xl md:max-w-2xl bg-scenic-image md:inset-shadow-sm/50"
>
    <div class="pretty continent page-head">
        <h1 class="!mb-0">Demos</h1>
    </div>

    <NavBar active="demos" />

    <div class="m-8"></div>

    <div class="pretty continent px-8 flex flex-col">
        <h2>nclbt</h2>
        <h4>nice command-line budget tool</h4>
        <Demo />
        <p class="indent-8">
            Going through university teaches you a lot about yourself. I found
            out that I am not as good with money as I'd like to be. To help with
            this, I wrote a little budgeting tool to help me keep track of how
            much I spend on things. It's got proper command-line support, as
            well as a little interactive REPL mode, which is usable above.
        </p>
        <p class="indent-8">
            It's written in pure Rust, and can be compiled as either a binary
            for the command-line or as a library for use in other projects, such
            as WASM for this demo. Hope you find it interesting!
        </p>
        <p class="indent-8">
            Here's the commands for the interactive text interface: (everything
            in [square brackets] is a value)
        </p>
        <ul>
            <li>income set [amount]: sets your expected income</li>
            <li>income raise [amount]: adds to your income</li>
            <li>paid: receive your income</li>
            <li>paid [amount]: receive some amount</li>
            <li>
                new [name] [amount]: create a new expenditure (overrides
                existing copies)
            </li>
            <li>pay [name]: pay a static expenditure</li>
            <li>pay [name] [amount]: pay some amount to an expenditure</li>
            <li>save [amount]: add an amount into savings [unimpl.]</li>
            <li>save all: add the remaining balance into savings [unimpl.]</li>
        </ul>
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
