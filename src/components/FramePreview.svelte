<script lang="ts">
  export let index: number;

  //TODO: load frame from ray
</script>

<div class="preview">
    <img src="vite.svg" alt="Frame {{index}}"/>
  </div>

<style lang="scss">

  .preview {
    display: flex;
    align-content: center;
    justify-content: center;
    border-radius: 10px;
    background-color: rgba(150, 150, 150, 0.5);
    width: 15vh;
    height: 15vh;
    img {
      width: 75%;
}
  }
</style>
