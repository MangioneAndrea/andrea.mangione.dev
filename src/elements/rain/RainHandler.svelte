<script>
  import RainDrop from "./RainDrop.svelte";

  let maxHeight;
  let rainDrops = [];

  const addRaindrop = (key) => {
    const elem = {
      onDestroy: () => {
        rainDrops = rainDrops.filter((el) => el !== elem);
      },
      coordinates: {
        x: Math.random() * 150,
        y: -50,
      },
      maxHeight,
    };
    rainDrops = [...rainDrops, elem];
  };
  const initRain = (elem) => {
    let key = 0;
    setInterval(() => {
      if (Math.random() < 0.01) {
        addRaindrop(++key);
      }
    }, 16);
  };
</script>

<div bind:offsetHeight={maxHeight} use:initRain>
  {#each rainDrops as props (props)}
    <RainDrop {...props} />
  {/each}
</div>

<style lang="scss">
  div {
    height: inherit;
  }
</style>
