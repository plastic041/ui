<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import Icon from "~/components/icon.svelte";
</script>

<div class="title-bar" data-tauri-drag-region>
  <nav data-tauri-drag-region>
    <button on:click={appWindow.close}>
      <Icon name="x" />
    </button>
  </nav>
</div>

<style>
  .title-bar {
    width: 100wh;
    background-color: beige;
  }

  button {
    border: none;
    background-color: #fff;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 1.5rem;
    width: 1.5rem;
  }

  button:hover {
    background-color: #eee;
  }
</style>
