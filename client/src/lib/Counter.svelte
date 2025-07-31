<script>
    import { Budget } from "$lib/pkg/nlbl.js";
    let account = $state(new Budget("Test"));
    const increment = () => {
        account = account.execute_string("paid 10");
    };
</script>

<button
    class="p-2 bg-stone-700 rounded-md w-full border-stone-700 border-2 hover:bg-stone-800 active:bg-stone-900 focus:outline-2 focus:outline-stone-700 focus:outline-offset-2"
    onclick={increment}
>
    {account.stringify()}
</button>
