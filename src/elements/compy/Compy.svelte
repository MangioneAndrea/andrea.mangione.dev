<script>
  import anime from "animejs";
  import { onMount } from "svelte";
  import { asyncAnime } from "../../helpers/AnimeExtention";

  import {
    debounce,
    clamp,
    getIdealCurve,
    deferAsync,
  } from "../../helpers/CommonFunctions";

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
  });

  onMount(() => {
    adaptArm();
    wink();
  });

  const resetEyes = debounce(() => {
    anime({
      targets: leftPupil,
      x: 25,
      y: 43,
      easing: "linear",
      duration: 200,
    });
    anime({
      targets: rightPupil,
      x: 67,
      y: 43,
      easing: "linear",
      duration: 200,
    });
  }, 1000);

  const moveEye = (_x, _y, eye, pup) => {
    const x = eye.x.baseVal.value;
    const y = eye.y.baseVal.value;
    window.eye = eye;
    const differenceX = eye.getBoundingClientRect().x;
    const differenceY = eye.getBoundingClientRect().y;
    const width = eye.getBoundingClientRect().width;
    const height = eye.getBoundingClientRect().height;
    const pWidth = pup.getBoundingClientRect().width;
    const pHeight = pup.getBoundingClientRect().height;
    anime({
      targets: pup,
      x: clamp(_x - differenceX + pWidth * 2, x, x + width - pWidth),
      y: clamp(_y - differenceY + pHeight * 2, y, y + height - pHeight),
      easing: "linear",
      duration: 30,
    });
  };

  document.addEventListener("mousemove", (evt) => {
    moveEye(evt.clientX, evt.clientY, leftEye, leftPupil, 315);
    moveEye(evt.clientX, evt.clientY, rightEye, rightPupil, 357);
    resetEyes();
  });
  document.addEventListener("touchstart", (e) => {
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
    resetRight();
    resetLeft();
  });
</script>

<svg
  version="1.1"
  xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://www.w3.org/1999/xlink"
  width="225"
  bind:this={main}
  on:click={wink}
  height="200">
  <defs>
    <!-- Head-->
    <g id="head">
      <!-- Head contour -->
      <rect width="103" height="65" x="
      0" y="24" class="skin" />
      <!-- Face -->
      <rect width="88" height="50" x="8" y="32" class="face" />
      <!-- Left eye -->
      <g>
        <rect x="17" y="43" class="eye" bind:this={leftEye} />
        <!-- Left pupil -->
        <rect x="25" y="43" class="pupil" bind:this={leftPupil} />
      </g>
      <!-- Right eye -->
      <g>
        <rect x="59" y="43" class="eye" bind:this={rightEye} />
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
  <use x="50" href="#head" />
  <use x="75" y="84" href="#body" id="body" bind:this={body} />
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
  <use x="160" y="37" href="#hand" id="rightHand" bind:this={rightHand} />
</svg>

<style type="text/scss">
  svg {
    position: relative;
    background-color: transparent;
    .face {
      fill: #9fbc4d;
    }
    .hand {
      fill: #9fbc4d;
    }
    .hair {
      transform-box: fill-box;
      transform-origin: center;
      transform: rotate(45deg);
    }
    .eye {
      width: 25px;
      height: 27px;
      fill: #ffffff;
    }
    .pupil {
      width: 9px;
      height: 16px;
      fill: #000000;
    }
    .skin {
      fill: #383c2c;
    }
  }
</style>
