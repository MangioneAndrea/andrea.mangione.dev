<script>
  export let languages;
  export let title;
  export let description;
  import anime from "animejs";
  import { onMount } from "svelte";

  let circleR = window.innerWidth > 1000 ? 100 : 75;
  const svgSize = circleR * 5;
  const centerAnchor = svgSize / 2;
  const imageSize = ((circleR * 2) / svgSize) * 100 + "%";
  const rotation = 0.2;
  let onGoingAnims = [];
  let defaultDescription, properDescription;

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
      duration: anime.random(4000,6000),
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
    onGoingAnims.forEach((a) => a.pause());
    properDescription.innerHTML = place.description;
    place.imageAnim = anime({
      targets: place.elem.firstChild,
      x: place.extrusionX,
      y: place.extrusionY,
      easing: "easeInQuad",
      duration: 500,
    });

    onGoingAnims = [
      anime({
        targets: properDescription,
        opacity: 1,
        duration: 2000,
      }),
      anime({
        targets: defaultDescription,
        opacity: 0,
        duration: 1500,
      }),
    ];
  };
  const restoreElement = (place) => {
    place.imageAnim?.pause();
    onGoingAnims.forEach((a) => a.pause());

    place.imageAnim = anime({
      targets: place.elem.firstChild,
      x: place.defaultExtrusionX,
      y: place.defaultExtrusionY,
      easing: "easeInOutCubic",
      duration: 500,
    });
    onGoingAnims = [
      anime({
        targets: properDescription,
        opacity: 0,
        duration: 1500,
      }),
      anime({
        targets: defaultDescription,
        opacity: 1,
        duration: 2000,
      }),
    ];
  };
</script>

<div class="stack">
  <div class="description">
    <h3>{title}</h3>
    <p class="defaultDescription" bind:this={defaultDescription}>
      {description}
    </p>
    <p class="properDescription" bind:this={properDescription} />
  </div>
  <div class="svgContainer">
    <svg width={svgSize} height={svgSize}>
      {#each places as place}
        <mask id="key{place.key}">
          <rect width={svgSize} height={svgSize} fill="white" />
          {#each places as other}
            {#if other !== place}
              <circle cx={other.x} cy={other.y} r={circleR} fill="black" />
              <circle
                cx={other.minX}
                cy={other.minY}
                r={circleR}
                fill="black"
              />
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
        <g id={place.key} mask="url(#key{place.key})" bind:this={place.elem}>
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
</div>

<style type="text/scss">
  div.stack {
    width: min-content;
    min-width: 49%;
    margin: auto;
    display: inline-flex;
    @media only screen and (max-width: 1000px) {
      display: block;
      min-width: 70%;
      @media only screen and (max-width: 768px) {
        min-width: 90%;
      }
    }
    p {
      height: 0px;
    }
    div.description {
      text-align: center;
      margin-top: auto;
      margin-bottom: auto;
      max-width: 50%;
      height: fit-content;
      @media only screen and (max-width: 1500px) {
        margin-bottom: 50%;
      }
      @media only screen and (max-width: 1000px) {
        max-width: 100%;
        margin: auto;
        margin-bottom: 50px;
      }
    }
    .svgContainer {
      text-align: center;
    }
  }
</style>
