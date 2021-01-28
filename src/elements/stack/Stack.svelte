<script>
  export let languages;
  export let title;
  import anime from "animejs";
  import { onMount } from "svelte";

  const circleR = 75;
  const svgSize = circleR * 5;
  const centerAnchor = svgSize / 2;
  const imageSize = ((circleR * 2) / svgSize) * 100 + "%";
  const rotation = 0.2;

  const places = languages.map((place, index) => {
    const proportion = (index * Math.PI * 2) / languages.length;
    const rotationProLanguage = rotation; // languages.length
    let res = {
      ...place,
      minX: centerAnchor + circleR * Math.cos(proportion - rotationProLanguage),
      minY: centerAnchor + circleR * Math.sin(proportion - rotationProLanguage),
      x: centerAnchor + circleR * Math.cos(proportion),
      y: centerAnchor + circleR * Math.sin(proportion),
      maxX: centerAnchor + circleR * Math.cos(proportion + rotationProLanguage),
      maxY: centerAnchor + circleR * Math.sin(proportion + rotationProLanguage),
    };
    const imageCenterX = (res.x + centerAnchor) / 2;
    const imageCenterY = (res.y + centerAnchor) / 2;
    res = {
      ...res,
      defaultExtrusionX: imageCenterX - circleR,
      defaultExtrusionY: imageCenterY - circleR,
      extrusionX: centerAnchor + (res.x - centerAnchor) * 1.5 - circleR,
      extrusionY: centerAnchor + (res.y - centerAnchor) * 1.5 - circleR,
    };

    return res;
  });
  // Each element needs to know the previous one
  places.forEach((element, index) => {
    places[(index + 1) % languages.length].previous = element;
  });
  onMount(() => {
    const props = {
      easing: "easeInOutCubic",
      direction: "alternate",
      loop: true,
      duration: 5000,
    };
    places.forEach((place) => {
      anime({
        targets: place.maskVisible,
        cx: place.maxX,
        cy: place.maxY,
        ...props,
      });
      anime({
        targets: place.maskHidden,
        cx: place.previous.maxX,
        cy: place.previous.maxY,
        ...props,
      });
    });
  });

  const extrudeElement = (place) => {
    place.imageAnim?.pause();
    place.imageAnim = anime({
      targets: place.elem.firstChild,
      x: place.extrusionX,
      y: place.extrusionY,
      easing: "easeInQuad",
      duration: 500,
    });
  };
  const restoreElement = (place) => {
    place.imageAnim?.pause();
    place.imageAnim = anime({
      targets: place.elem.firstChild,
      x: place.defaultExtrusionX,
      y: place.defaultExtrusionY,
      easing: "easeInOutCubic",
      duration: 500,
    });
  };
</script>

<div class="stack">
  <h4>{title}</h4>
  <svg width={svgSize} height={svgSize}>
    {#each places as place}
      <mask id="key{place.description}">
        <rect width={svgSize} height={svgSize} fill="white" />
        {#each places as other}
          {#if other !== place}
            <circle cx={other.x} cy={other.y} r={circleR} fill="black" />
            <circle cx={other.minX} cy={other.minY} r={circleR} fill="black" />
          {/if}
        {/each}
        <circle
          cx={place.minX}
          cy={place.minY}
          r={circleR}
          fill="white"
          bind:this={place.maskVisible}
        />
        <circle
          cx={place.previous.minX}
          cy={place.previous.minY}
          r={circleR}
          fill="black"
          stroke="black"
          stroke-width="5px"
          bind:this={place.maskHidden}
        />
      </mask>
      <g
        id={place.description}
        mask="url(#key{place.description})"
        bind:this={place.elem}>
        <svelte:component
          this={place.image}
          width={imageSize}
          height={imageSize}
          x={place.defaultExtrusionX}
          y={place.defaultExtrusionY}
        />
      </g>
    {/each}
    {#each places as place}
      <circle
        on:mouseenter={() => extrudeElement(place)}
        on:mouseleave={() => restoreElement(place)}
        cx={place.x}
        cy={place.y}
        r={circleR}
        fill="none"
        pointer-events="visible"
      />
    {/each}
  </svg>
</div>

<style type="text/scss">
  div.stack {
    width: fit-content;
    h4{
      text-align: center;
    }
  }
</style>
