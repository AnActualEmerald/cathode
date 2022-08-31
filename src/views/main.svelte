<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { onDestroy, onMount } from "svelte";

  let transparent = false;

  onMount(async () => {
    await appWindow.onFocusChanged(({ payload: focused }) => {
      transparent = !focused;
    });
  });
</script>

<div class="container" class:transparent >
<h1> Hell world</h1>
</div>

<style>
  .container {
    background-color: red;
  }
  
  .container.transparent {
    background-color: rgba(0,0,0,0);
}
  

</style>
