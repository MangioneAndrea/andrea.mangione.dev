<script>
  export let languages;
  export let title;
  export let description;
  import anime from "animejs";
  import { onMount } from "svelte";

  let showndescription = description;

  const maxWiggleX = 3;
  const maxWiggleY = 20;
  const imageSize = 75;
  const svgSize = imageSize * 2.6;
  const imageSizePerc = ((imageSize * 2) / svgSize) * 100 + "%";
  const imageSizePercFocussed = ((imageSize * 2) / svgSize) * 110 + "%";

  const places = languages.map((language) => {
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

  const focusElement = (place) => {
    showndescription = place.description;
    place.imageAnim = anime({
      targets: place.elem.firstChild,
      x: svgSize / 2 - imageSize * 1.1,
      width: imageSizePercFocussed,
      height: imageSizePercFocussed,
      opacity: 1,
      easing: "easeInQuad",
      duration: 500,
    });
    places.forEach((other) => {
      if (other !== place) {
        place.imageAnim = anime({
          targets: other.elem.firstChild,
          opacity: 0.6,
          easing: "easeInQuad",
          duration: 500,
        });
      }
    });
  };
  const unfocusElement = (place) => {
    showndescription = description;
    place.imageAnim = anime({
      targets: place.elem.firstChild,
      x: svgSize / 2 - imageSize,
      width: imageSizePerc,
      height: imageSizePerc,
      easing: "easeInQuad",
      duration: 500,
    });
  };
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
          on:mouseenter={() => focusElement(place)}
          on:mouseleave={() => unfocusElement(place)}
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
