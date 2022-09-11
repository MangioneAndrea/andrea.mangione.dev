<script>
  import { onMount } from "svelte";

  import {
    debounce,
    clamp,
    getIdealCurve,
    deferAsync,
  } from "../../helpers/CommonFunctions.js";

  let leftPupil, rightPupil;
  let leftEye, rightEye;
  let rightHand;
  let rightArm, leftArm;
  let body, main;
  const adaptArm = () => {
    rightArm.setAttribute(
      "d",
      getIdealCurve({
        p1x:
          parseFloat(body.getAttribute("x")) +
          body.getBoundingClientRect().width,
        p1y:
          parseFloat(body.getAttribute("y")) +
          body.getBoundingClientRect().height / 3,
        p2x:
          parseFloat(rightHand.getAttribute("x")) +
          rightHand.getBoundingClientRect().width / 2,
        p2y:
          parseFloat(rightHand.getAttribute("y")) +
          rightHand.getBoundingClientRect().height,
      })
    );
    leftArm.setAttribute(
      "d",
      getIdealCurve({
        p1x: parseFloat(body.getAttribute("x")),
        p1y:
          parseFloat(body.getAttribute("y")) +
          body.getBoundingClientRect().height / 3,
        p2x: parseFloat(body.getAttribute("x")),
        p2y:
          parseFloat(body.getAttribute("y")) +
          body.getBoundingClientRect().height / 1.3,
        minCurve: -50,
      })
    );
  };

  const wink = deferAsync(async () => {
    /*
    await asyncAnime({
      targets: rightHand,
      direction: "alternate",
      x: 190,
      easing: "linear",
      duration: 200,
      rotate: 9,
      loop: 10,
      update: adaptArm,
    });
    */
  });

  onMount(() => {
    adaptArm();
    wink();
  });

  let pupilLeft = `--pupil-x: ${0}px; --pupil-y: ${0}px;`;
  let pupilRight = `--pupil-x: ${0}px; --pupil-y: ${0}px;`;

  const resetEyes = debounce(() => {
    console.log("resetting")
    pupilLeft = `--pupil-x: ${0}px; --pupil-y: ${0}px;`;
    pupilRight = `--pupil-x: ${0}px; --pupil-y: ${0}px;`;
  }, 1000);

  $: console.log(pupilLeft, pupilRight);

  const moveEye = (_x, _y, eye, pup) => {
    const differenceX = eye.getBoundingClientRect().x;
    const differenceY = eye.getBoundingClientRect().y;

    const width = eye.getBoundingClientRect().width;
    const height = eye.getBoundingClientRect().height;

    const centerX = width / 2 + differenceX;
    const centerY = height / 2 + differenceY;

    const pWidth = pup.getBoundingClientRect().width;
    const pHeight = pup.getBoundingClientRect().height;

    const xBound = (width - pWidth) / 2;
    const yBound = height - pHeight;

    const tx = Math.round(clamp(_x - centerX, -xBound, xBound));
    const ty = Math.round(clamp(_y - centerY + pHeight / 2, 0, yBound));

    return `--pupil-x: ${tx}px; --pupil-y: ${ty}px;`;
  };

  document.addEventListener("mousemove", (evt) => {
    pupilLeft = moveEye(evt.clientX, evt.clientY, leftEye, leftPupil);
    pupilRight = moveEye(evt.clientX, evt.clientY, rightEye, rightPupil);
    resetEyes();
  });
  document.addEventListener("touchstart", (e) => {
    /*
    moveEye(
      e.touches[0].clientX,
      e.touches[0].clientY,
      leftEye,
      leftPupil,
      315
    );
    moveEye(
      e.touches[0].clientX,
      e.touches[0].clientY,
      rightEye,
      rightPupil,
      357
    );
    */
  });

  let width;
</script>

<svelte:window bind:innerWidth={width} />
<svg
  version="1.1"
  xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://www.w3.org/1999/xlink"
  width="200"
  height="200"
  bind:this={main}
  on:click={wink}
>
  <defs>
    <!-- Head-->
    <g id="head">
      <!-- Head contour -->
      <rect width="103" height="65" x="0" y="24" class="skin" rx="7" />
      <!-- Face -->
      <rect width="88" height="50" x="8" y="32" class="face" rx="5" />
      <!-- Left eye -->
      <g style={pupilLeft}>
        <rect x="17" y="43" class="eye" bind:this={leftEye} rx="5" />
        <!-- Left pupil -->
        <rect x="25" y="43" class="pupil" bind:this={leftPupil} />
      </g>
      <!-- Right eye -->
      <g style={pupilRight}>
        <rect x="59" y="43" class="eye" bind:this={rightEye} rx="5" />
        <!-- Right pupil -->
        <rect x="67" y="43" class="pupil" bind:this={rightPupil} />
      </g>
      <!-- 1 Hair -->
      <g>
        <rect width="10" height="30" x="30" y="2" class="skin hair" />
        <rect width="10" height="30" x="50" y="2" class="skin hair" />
        <rect width="10" height="30" x="70" y="2" class="skin hair" />
      </g>
    </g>
    <g id="armOrigin" />
    <!-- Hand -->
    <g id="hand">
      <rect x="3" width="5" height="15" rx="5" />
      <rect x="3" width="5" height="15" rx="5" />
      <rect x="22" width="5" height="15" rx="5" />
      <polygon points="0,13 30,13 20,28 10,28" class="hand" />
    </g>
    <!-- Body -->
    <g id="body">
      <rect width="55" height="63" class="skin" />
    </g>
  </defs>
  <use x="0" y="0" href="#head" />
  <use x="25" y="84" href="#body" id="body" bind:this={body} />
  <path
    fill="transparent"
    stroke="black"
    stroke-width="4"
    bind:this={rightArm}
  />
  <path
    fill="transparent"
    stroke="black"
    stroke-width="4"
    bind:this={leftArm}
  />
  <use x="110" y="37" href="#hand" id="rightHand" bind:this={rightHand} />
</svg>

<style>
  @keyframes movep {
    100% {
      transform: translate(0, 0);
    }
    50% {
      transform: translate(var(--pupil-x), var(--pupil-y));
    }
    100% {
      transform: translate(0, 0);
    }
  }

  .pupil {
    transform: translate(var(--pupil-x), var(--pupil-y));
  }

  svg {
    position: fixed;
    background-color: transparent;
  }
  svg .face {
    fill: #9fbc4d;
  }
  svg .hand {
    fill: #9fbc4d;
  }
  svg .hair {
    transform-box: fill-box;
    transform-origin: center;
    transform: rotate(45deg);
  }
  svg .eye {
    width: 25px;
    height: 27px;
    fill: #ffffff;
  }
  svg .pupil {
    width: 9px;
    height: 16px;
    fill: #000000;
    transition: all linear .2s;
    -webkit-backface-visibility: hidden;
  }
  svg .skin {
    fill: #383c2c;
  }
</style>
