<script>
  import { sleep } from "../../../helpers/CommonFunctions";

  export let languages;
  export let description;

  const speed = `${7 + Math.random() * 3}s`;

  let circleR = window.innerWidth > 1000 ? 100 : 75;
  const svgSize = circleR * 3;
  const centerAnchor = svgSize / 2;
  const imageSize = ((circleR * 2) / svgSize) * 100 + "%";
  const rotation = 0.2;

  const places = languages?.map((place, index) => {
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
      tX: 0,
      tY: 0,
    };

    return res;
  });

  // Each element needs to know the previous one
  places.forEach((element, index) => {
    places[(index + 1) % languages.length].previous = element;
  });


  // No, css animations are not enough performant
  new Promise(async () => {
    let stage = 1000;
    while (1) {
      await sleep(10);
      if (stage) {
        stage--;
      } else {
        stage = 1000;
      }
      let step=Math.abs(stage - 500) / 500 // 0 - 1
      step = step * step * (3 - 2*step); // bezier
      places.forEach((place) => {
        place.tX = (place.maxX - place.minX) * step;
        place.tY = (place.maxY - place.minY) * step;
        place.mX = (place.previous.maxX - place.previous.minX) * step;
        place.mY = (place.previous.maxY - place.previous.minY) * step;
      });
      places = places;
    }
  });
</script>

<div class="stack">
  <div class="description">
    <h2 class="defaultDescription">
      {description}
    </h2>
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
            class="maskVisible"
            style="transform: translate({place.tX}px, {place.tY}px);"
          />
          <circle
            cx={place.previous.minX}
            cy={place.previous.minY}
            r={circleR}
            fill="black"
            stroke="black"
            stroke-width="5px"
            class="maskInvisible"
            style="transform: translate({place.mX}px, {place.mY}px);"
          />
        </mask>
        <g id={place.key} mask="url(#key{place.key})" bind:this={place.elem}>
          <image
            href={place.image}
            width={imageSize}
            height={imageSize}
            x={place.defaultExtrusionX}
            y={place.defaultExtrusionY}
          />
        </g>
      {/each}
      {#each places as place}
        <circle
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

<style>
  @keyframes move {
    50% {
      transform: translate(var(--target-x), var(--target-y));
    }
    100% {
      transform: translate(0, 0);
    }
  }

  .maskVisible,
  .maskInvisible {
    animation: move var(--speed) infinite;
  }

  div.stack {
    width: min-content;
    min-width: 49%;
    margin: auto;
    display: inline-flex;
    justify-content: space-around;
  }

  @media only screen and (max-width: 1000px) {
    div.stack {
      display: block;
      min-width: 70%;
      margin-top: 4rem;
    }
  }

  @media only screen and (max-width: 768px) {
    div.stack {
      min-width: 90%;
    }
  }

  div.stack > div.description {
    text-align: center;
    margin-top: auto;
    margin-bottom: auto;
    max-width: 50%;
    height: fit-content;
  }

  @media only screen and (max-width: 1000px) {
    div.stack > div.description {
      max-width: 100%;
      margin: auto;
      margin-bottom: 0;
    }
  }

  div.stack > .svgContainer {
    will-change: transform;
    text-align: center;
    display: flex;
    justify-content: center;
  }
</style>
