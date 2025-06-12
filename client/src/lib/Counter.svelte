<script>
    import { greet } from "./pkg/portfolio";
    let count = $state(0);
    const increment = () => {
        count += 1;
        console.log(greet());
    };
</script>

<button
    class="p-2 bg-stone-700 rounded-md w-full border-stone-700 border-2 hover:bg-stone-800 active:bg-stone-900 focus:outline-2 focus:outline-stone-700 focus:outline-offset-2"
    onclick={increment}
>
    count is {count}
</button>
