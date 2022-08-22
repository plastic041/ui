<script lang="ts">
  import TitleBar from "~/components/title-bar.svelte";
  import { searchQuery } from "~/stores/query";
</script>

<TitleBar />
<main>
  <input type="text" bind:value={$searchQuery} />
</main>

<style>
  main {
    width: 100vw;
    height: 100vh;
    margin: 0 auto;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4rem;
    font-weight: 50;
    font-variant-numeric: tabular-nums;
  }
</style>
