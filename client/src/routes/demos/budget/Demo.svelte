<script>
    import { Budget } from "$lib/pkg/nlbl.js";
    const beautify = (acc) => {
        return `Welcome, ${acc.account}!\n
                Current Balance: $${acc.current_balance / 100}
                Expected Income: $${acc.expected_income / 100}
                Expenses: ${JSON.stringify(acc.current_expenses)}
                Expenses: ${JSON.stringify(acc.expected_expenses)}`;
    };
    let account = $state(new Budget("Demo User"));
    let display = $derived(beautify(account.as_obj()));
    let input = $state("");

    const execute = () => {
        account = account.execute_string(input);
        input = "";
    };
</script>

<div class="flex flex-col gap-2 justify-center !m-8">
    <div class="island !bg-stone-800 min-h-96 whitespace-pre-line">
        {display}
    </div>
    <div class="flex gap-2 justify-center">
        <input
            type="text"
            bind:value={input}
            class="pretty bg-stone-800 md:w-[80%]"
        />
        <button
            type="submit"
            class="form-input max-w-24 !bg-stone-800"
            onclick={execute}
        >
            run
        </button>
    </div>
</div>
