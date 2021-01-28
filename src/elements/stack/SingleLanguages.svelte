<script>
  export let languages;
  export let title;
  import anime from "animejs";
  import { onMount } from "svelte";
  import { oneRandomEase } from "../../helpers/AnimeExtention";

  const maxWiggleX = 3;
  const maxWiggleY = 10;
  const imageSize = 75;
  const svgSize = imageSize * 2.5;
  const imageSizePerc = ((imageSize * 2) / svgSize) * 100 + "%";

  const places = languages.map((language) => {
    const res = {
      ...language,
      x: Math.random() * maxWiggleX * 2,
      y: Math.random() * maxWiggleY * 2,
    };
    Object.assign(res, {
      wiggleX: res.x > maxWiggleX ? res.x - maxWiggleX : res.x + maxWiggleX,
      wiggleY: res.y > maxWiggleX ? res.y - maxWiggleY : res.y + maxWiggleY,
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
  <h4>{title}</h4>
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
    width: fit-content;
    h4 {
      text-align: center;
    }
  }
</style>
