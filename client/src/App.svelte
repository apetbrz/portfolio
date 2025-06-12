<script>
  import Counter from "./lib/Counter.svelte";
</script>

<main>
  <div class="header">
    <p>hi, i'm</p>
    <h1 class="name">Arthur Petroff</h1>
  </div>

  <div class="info">
    <p>this website is still in development! check back later</p>
  </div>

  <div class="card">
    <Counter />
  </div>
</main>

<style>
  .header {
    display: flex;
  }
</style>
