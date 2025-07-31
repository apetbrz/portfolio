<script>
    import { Budget, format_dollars } from "$lib/pkg/nlbl.js";

    const beautify = (acc) => {
        let ce = acc.current_expenses
            .entries()
            .toArray()
            .map((e) => {
                return `\n${e[0]}: ${format_dollars(e[1])}`;
            });
        let ee = acc.expected_expenses
            .entries()
            .toArray()
            .map((e) => {
                return `\n${e[0]}: ${format_dollars(e[1])}`;
            });

        return `Welcome, ${acc.account}!\n
                Current Balance: ${format_dollars(acc.current_balance)}
                Expected Income: ${format_dollars(acc.expected_income)}
                Current Expenses: ${ce}
                Expense Limits: ${ee}`;
    };

    let account = $state(new Budget("Demo User"));
    let display = $derived(beautify(account.as_obj()));
    let input = $state("");

    const execute = () => {
        console.log(input);
        console.log(account);
        console.log(account.__wbg_ptr);
        try {
            account = account.execute_string_immut(input);
        } catch (err) {
            console.error(err);
        }
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
            onkeypress={(ev) => {
                if (ev.key === "Enter") {
                    execute();
                }
            }}
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
