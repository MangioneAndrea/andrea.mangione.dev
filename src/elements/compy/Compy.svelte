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

  let compyRight = 0;
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
    let random = compyRight;

    while (Math.abs(random - compyRight) < 30) {
      random = Math.random() * 100;
    }

    compyRight = random;
  });

  onMount(() => {
    adaptArm();
    wink();
  });

  let pupilLeft = `--pupil-x: ${0}px; --pupil-y: ${0}px;`;
  let pupilRight = `--pupil-x: ${0}px; --pupil-y: ${0}px;`;

  const resetEyes = debounce(() => {
    console.log("resetting");
    pupilLeft = `--pupil-x: ${0}px; --pupil-y: ${0}px;`;
    pupilRight = `--pupil-x: ${0}px; --pupil-y: ${0}px;`;
  }, 1000);

  const moveEye = (_x, _y, eye, pup) => {
    const eyeX = eye.getBoundingClientRect().x;
    const eyeY = eye.getBoundingClientRect().y;

    const eyeW = eye.getBoundingClientRect().width;
    const eyeH = eye.getBoundingClientRect().height;

    const pWidth = pup.getBoundingClientRect().width;
    const pHeight = pup.getBoundingClientRect().height;

    const centerX = eyeW / 2 + eyeX;
    const centerY = eyeH / 2 + eyeY;

    const xBound = (eyeW - pWidth) / 2;
    const yBound = eyeH - pHeight;

    const tx = Math.round(clamp(_x - centerX, -xBound, xBound));
    const ty = Math.round(clamp(_y - centerY + pHeight / 2, 0, yBound));

    return `--pupil-x: ${tx}px; --pupil-y: ${ty}px;`;
  };

  document.addEventListener("mousemove", (evt) => {
    pupilLeft = moveEye(evt.clientX, evt.clientY, leftEye, leftPupil);
    pupilRight = moveEye(evt.clientX, evt.clientY, rightEye, rightPupil);
    resetEyes();
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
  style="--compy-right: {compyRight}%;"
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
  .pupil {
    transform: translate(var(--pupil-x), var(--pupil-y));
  }

  svg {
    position: absolute;
    right: var(--compy-right);
    transform: translateX(calc(var(--compy-right) + 1.5rem));
    top: 10%;
    background-color: transparent;
    transition: all ease-in-out 0.2s;
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
    transition: all linear 0.2s;
    -webkit-backface-visibility: hidden;
    backface-visibility: hidden;
  }
  svg .skin {
    fill: #383c2c;
  }
</style>
