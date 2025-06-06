<script>
  import { greet } from "./pkg/portfolio"
  let count = $state(0)
  const increment = () => {
    count += 1
    alert(greet())
  }
</script>

<button onclick={increment}>
  count is {count}
</button>
