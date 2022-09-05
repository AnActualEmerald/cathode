<script lang="ts">
  import { frames } from "../store";
  import { invoke } from "@tauri-apps/api";
  export let index: number;

  let menuTimeout: NodeJS.Timeout | null = null;
  let showMenu = false;
  let src = "";

  $: {
    src = $frames[index];
  }

  const openImage = async () => {
    const path = (await invoke("open_image")) as string;
    if (path) {
      $frames[index] = path;
    }
  };

  //TODO: load frame from ray
</script>

<div
  class="preview"
  on:click={openImage}
  on:contextmenu={openImage}
  on:mouseenter={() =>
    (menuTimeout = setTimeout(() => (showMenu = true), 1000))}
  on:mouseleave={() => {
    if (menuTimeout) {
      clearTimeout(menuTimeout);
    }
    showMenu = false;
  }}
>
  <p>{showMenu}</p>
  {#if src}
    <img {src} alt="Frame {{ index }}" />
  {/if}
</div>

<style lang="scss">
  $bg: rgba(150, 150, 150, 0.5);
  .preview {
    &:hover {
      background-color: darken($color: $bg, $amount: 5%);
    }
    display: flex;
    align-content: center;
    justify-content: center;
    border-radius: 10px;
    background-color: $bg;
    width: 20vh;
    height: 20vh;
  }

  img {
    width: 75%;
  }
</style>
