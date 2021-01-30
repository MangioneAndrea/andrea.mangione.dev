<script>
  export let languages;
  export let title;
  export let description;
  import anime from "animejs";
  import { onMount } from "svelte";
  import { oneRandomEase } from "../../helpers/AnimeExtention";

  const maxWiggleX = 3;
  const maxWiggleY = 20;
  const imageSize = 75;
  const svgSize = imageSize * 2.5;
  const imageSizePerc = ((imageSize * 2) / svgSize) * 100 + "%";

  const places = languages.map((language) => {
    const res = {
      ...language,
      x: 0,
      y: 0,
    };
    Object.assign(res, {
      wiggleX: maxWiggleX,
      wiggleY: maxWiggleX,
    });
    return res;
  });

  onMount(() => {
    places.forEach((place) => {
      anime({
        targets: place.elem,
        translateY: {
          value: anime.random(0, place.wiggleY),
          easing: oneRandomEase(),
        },
        translateX: {
          value: anime.random(0, place.wiggleX),
          easing: oneRandomEase(),
        },
        duration: 3000,
        loop: true,
        direction: "alternate",
      });
    });
  });
</script>

<div class="singleLanguages">
  <div class="description">
    <h3>{title}</h3>
    <p>
      {description}
    </p>
  </div>
  {#each places as place}
    <svg width={svgSize} height={svgSize}>
      <g bind:this={place.elem}>
        <svelte:component
          this={place.image}
          width={imageSizePerc}
          height={imageSizePerc}
          x={place.x}
          y={place.y}
        /></g
      ></svg
    >
  {/each}
</div>

<style type="text/scss">
  div.singleLanguages {
    margin: auto;
    width: fit-content;
    div.description {
      text-align: center;
    }
  }
</style>
