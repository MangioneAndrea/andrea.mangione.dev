<script>
  import anime from "animejs";

  import { debounce } from "../../helpers/CommonFunctions";

  let leftPupil, rightPupil;
  let leftEye, rightEye;

  const resetLeft = debounce(() => {
    anime({
      targets: leftPupil,
      x: 315,
      y: 283,
      easing: "linear",
      duration: 200,
    });
  }, 1000);
  const resetRight = debounce((prop) => {
    anime({
      targets: rightPupil,
      x: 357,
      y: 283,
      easing: "linear",
      duration: 200,
    });
  }, 1000);

  const moveEye = (evt, eye, pup, defaultX) => {
    const x = eye.x.baseVal.value;
    const y = eye.y.baseVal.value;
    const differenceX = eye.getBoundingClientRect().x;
    const differenceY = eye.getBoundingClientRect().y;
    const width = eye.getBoundingClientRect().width;
    const height = eye.getBoundingClientRect().height;
    anime({
      targets: pup,
      x: Math.clamp(evt.screenX - differenceX, x, x + width),
      y: Math.clamp(evt.screenY - differenceY, y, y + height),
      easing: "linear",
      duration: 30,
    });
  };
  document.addEventListener("mousemove", (evt) => {
    moveEye(evt, leftEye, leftPupil, 315);
    moveEye(evt, rightEye, rightPupil, 357);
    resetRight();
    resetLeft();
  });
</script>

<svg
  version="1.1"
  xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://www.w3.org/1999/xlink"
  viewBox="290 240 105 185"
  width="500"
  height="500">
  <!-- Head contour -->
  <rect width="103" height="65" x="290" y="264" id="a2JYesZlv3" class="skin" />
  <!-- Face -->
  <rect width="88" height="50" x="298" y="272" class="face" />
  <!-- Body -->
  <rect width="55" height="63" x="315" y="324" id="f7bstTLj33" class="skin" />
  <!-- Left eye -->
  <g>
    <rect x="307" y="283" class="eye" bind:this={leftEye} />
    <!-- Left pupil -->
    <rect x="315" y="283" class="pupil" bind:this={leftPupil} /></g
  >
  <!-- Right eye -->
  <g>
    <rect x="349" y="283" class="eye" bind:this={rightEye} />
    <!-- Right pupil -->
    <rect x="357" y="283" class="pupil" bind:this={rightPupil} />
  </g>
  <!-- Left leg -->
  <path
    d="M328.72 380.26C331.48 380.26 333.72 382.5 333.72 385.26C333.72 393.21 333.72 412.07 333.72 420.02C333.72 422.78 331.48 425.02 328.72 425.02C326.77 425.02 325.91 425.02 323.96 425.02C321.2 425.02 318.96 422.78 318.96 420.02C318.96 412.07 318.96 393.21 318.96 385.26C318.96 382.5 321.2 380.26 323.96 380.26C325.91 380.26 326.77 380.26 328.72 380.26Z"
    class="skin"
  />
  <!-- Right leg -->
  <path
    d="M360.87 380.26C363.63 380.26 365.87 382.5 365.87 385.26C365.87 393.21 365.87 412.07 365.87 420.02C365.87 422.78 363.63 425.02 360.87 425.02C358.91 425.02 358.06 425.02 356.1 425.02C353.34 425.02 351.1 422.78 351.1 420.02C351.1 412.07 351.1 393.21 351.1 385.26C351.1 382.5 353.34 380.26 356.1 380.26C358.06 380.26 358.91 380.26 360.87 380.26Z"
    class="skin"
  />
  <!-- Left arm -->
  <path
    d="M313.77 339.79C315.44 336.99 319.07 336.07 321.88 337.74C322.89 338.35 320.86 337.13 321.88 337.74C324.68 339.41 325.6 343.04 323.93 345.85C319.31 353.59 308.49 371.74 303.88 379.48C302.2 382.29 298.58 383.21 295.77 381.53C294.75 380.93 296.79 382.14 295.77 381.53C292.97 379.86 292.05 376.23 293.72 373.43C298.33 365.69 309.15 347.53 313.77 339.79Z"
    class="skin"
  />

  <!-- 1 Hair -->
  <path
    d="M321.09 243.45L324.76 245.15L313.44 269.56L309.76 267.85L321.09 243.45Z"
    class="skin"
  />
  <!-- 2 Hair -->
  <path
    d="M341.71 243.45L345.39 245.15L334.07 269.56L330.39 267.85L341.71 243.45Z"
    class="skin"
  />
  <!-- 3 Hair -->
  <path
    d="M358.62 243.45L362.29 245.15L350.97 269.56L347.3 267.85L358.62 243.45Z"
    class="skin"
  />
  <!-- 4 Hair -->
  <path
    d="M378.74 243.45L382.41 245.15L371.09 269.56L367.41 267.85L378.74 243.45Z"
    class="skin"
  />
  <!-- Right arm -->
  <path
    d="M369.97 339.79C368.29 336.99 364.67 336.07 361.86 337.74C360.84 338.35 362.88 337.13 361.86 337.74C359.06 339.41 358.14 343.04 359.81 345.85C364.43 353.59 375.24 371.74 379.86 379.48C381.53 382.29 385.16 383.21 387.97 381.53C388.98 380.93 386.95 382.14 387.97 381.53C390.77 379.86 391.69 376.23 390.02 373.43C385.4 365.69 374.58 347.53 369.97 339.79Z"
    class="skin"
  />
</svg>

<style type="text/scss">
  svg {
    position: relative;
    width: 100px;
    height: 100px;
    background-color: transparent;
    .face {
      fill: #9fbc4d;
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
