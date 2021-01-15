<script>
  import anime from "animejs";
  export let coordinates = { x: 0, y: 0 };
  export let onDestroy = () => {};
  export let maxHeight = 700;
  const colors = ["red", "orange", "yellow", "green"];
  const color = colors[Math.round(Math.random() * colors.length)];

  const onCreation = (elem) => {
    anime({
      targets: elem,
      opacity:0,
      translateY: maxHeight,
      duration: 3000,
      easing: "linear",
      complete: onDestroy,
    });
  };
</script>

{#if coordinates >= maxHeight}
  <div>done</div>
{:else}
  <div
    use:onCreation
    class="raindrop"
    style="background-color:{color}; left:{coordinates.x}px; top:{coordinates.y}px"
  />
{/if}

<style lang="scss">
  div {
    width: 10px;
    height: 50px;
    position: absolute;
    border-radius: 5px;
  }
</style>
