<script>
  export let languages;
  export let title;
  export let description;
  import anime from "animejs";
  import { onMount } from "svelte";

  let showndescription = description;

  const maxWiggleX = 3;
  const maxWiggleY = 5;
  const imageSize = 75;
  const svgSize = imageSize * 2.6;
  const imageSizePerc = ((imageSize * 2) / svgSize) * 100 + "%";
  const imageSizePercFocussed = ((imageSize * 2) / svgSize) * 110 + "%";

  const places = languages?.map((language) => {
    const res = {
      ...language,
      x: svgSize / 2 - imageSize,
      y: svgSize / 2 - imageSize,
    };
    Object.assign(res, {
      wiggleX: maxWiggleX,
      wiggleY: maxWiggleY,
    });
    return res;
  });

  onMount(() => {
    places.forEach((place) => {
      anime({
        targets: place.elem,
        translateY: {
          value: anime.random(0, place.wiggleY),
          easing: "easeInOutSine",
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
      {showndescription}
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
        />

        <rect
          x={place.x}
          y={place.y}
          width={imageSizePerc}
          height={imageSizePerc}
          fill="none"
          pointer-events="visible"
        />
      </g>
    </svg>
  {/each}
</div>

<style type="text/scss">
  div.singleLanguages {
    margin: auto;
    width: fit-content;
    div.description {
      text-align: center;
      p {
        min-height: 5rem;
      }
    }
  }
</style>
