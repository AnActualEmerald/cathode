<script lang="ts">
  import { appWindow, PhysicalSize } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import FramePreview from "../components/FramePreview.svelte";
  import Tuber from "../components/tube.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let transparent = false;
  let threshold = 0.0;
  let level = 0.0;

  onMount(async () => {
    await appWindow.setMinSize(new PhysicalSize(720, 600));
    await appWindow.onFocusChanged(({ payload: focused }) => {
      transparent = !focused;
    });

    setInterval(async () => {
      threshold = await invoke("get_mic_threshold");
    }, 500);

    setInterval(async () => {
      level = await invoke("get_audio_level");
    }, 50);
  });
</script>

<div class="container" class:transparent>
  <Tuber />
  {#if !transparent}
    <div
      transition:fly={{ duration: 200, x: -200, opacity: 100 }}
      class="frames"
    >
      {#each [0, 1, 2, 3] as i}
        <FramePreview index={i} />
      {/each}
    </div>

    <div transition:fly={{ duration: 200, x: 200, opacity: 100 }} class="audio">
      <p>{threshold.toPrecision(2).toString()}</p>
      <p>{(level * 100).toPrecision(3).toString()}</p>
    </div>
  {/if}
</div>

<svelte:body on:contextmenu|preventDefault />

<style lang="scss">
  .frames {
    align-items: left;
    position: absolute;
    top: 5vh;
    left: 20px;
    bottom: 5vh;
    display: flex;
    justify-content: space-between;
    flex-direction: column;
  }

  .audio {
    position: absolute;
    display: flex;
    gap: 10px;
    bottom: 5vh;
    top: 5vh;
    right: 30px;
  }

  @keyframes fade-out {
    0% {
      background-color: inherit;
    }
    100% {
      background-color: transparent;
    }
  }

  @keyframes fade-in {
    0% {
      background-color: transparent;
    }
    100% {
      background-color: inherit;
    }
  }
  .container {
    background-color: inherit;
    animation-name: fade-in;
    animation-duration: 0.2s;
  }

  .container.transparent {
    animation-name: fade-out;
    animation-duration: 0.2s;
    background-color: transparent;
  }
</style>
