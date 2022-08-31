<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";

  let transparent = false;

  onMount(async () => {
    await appWindow.onFocusChanged(({ payload: focused }) => {
      transparent = !focused;
    });
  });
</script>

<div class="container" class:transparent>
  {#if !transparent}
  <div transition:fly="{{ duration: 200, x: -200, opacity: 100}}" class="frames" > 
    <p> 1 </p>
    <p> 2 </p>
    <p> 3 </p>
    <p> 4 </p>
  </div>
  {/if}
</div>

<style>
  .frames {
    position: absolute;
    top: 20px;
    left: 20px;
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
