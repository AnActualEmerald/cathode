<script lang="ts">
  import { appWindow, PhysicalSize } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import FramePreview from "../components/FramePreview.svelte";

  let transparent = false;

  onMount(async () => {
    await appWindow.setMinSize(new PhysicalSize(720, 600));
    await appWindow.onFocusChanged(({ payload: focused }) => {
      transparent = !focused;
    });
  });
</script>

<div class="container" class:transparent>
  {#if !transparent}
  <div transition:fly="{{ duration: 200, x: -200, opacity: 100}}" class="frames" > 
      {#each [0, 1, 2, 3] as i}
        <FramePreview index={i} />
      {/each}
  </div>
  {/if}
</div>

<style>
  .frames {
    align-items: left;
    position: absolute;
    top: 5vh;
    left: 30px;
    bottom: 5vh;
    display: flex;
    justify-content: space-between;
    flex-direction: column;
}


  @keyframes fade-out {
   0% {background-color: inherit;}
   100% {background-color: transparent;}
}

  @keyframes fade-in {
   0% {background-color: transparent;}
   100% {background-color: inherit;}
}
  .container {
    background-color: inherit;
    animation-name: fade-in;
    animation-duration: 0.25s;
  }
  
  .container.transparent {
    animation-name: fade-out;
    animation-duration: 0.25s;
    background-color: transparent;


}
  

</style>
