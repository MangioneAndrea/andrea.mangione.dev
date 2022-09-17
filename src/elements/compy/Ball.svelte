<script>
  import { clamp, sleep } from "../../helpers/CommonFunctions";

  export let x;
  export let y;
  export let targetX;
  export let targetY;

  const xDistance = (targetX - x) / 2;
  const absXDistance = Math.abs(xDistance * 0.05);
  let step1X = x + xDistance;
  let step1Y = y - 100 - absXDistance * absXDistance;

  if (targetY < y) {
    step1Y = targetY-10;
  }

  let currentX = x;
  let currentY = y;
  new Promise(async (resolve) => {
    let t = 200;
    let goingDown = false;
    let xSpeed = (step1X - currentX) / 10;
    let ySpeed = (step1Y - currentY) / 10;

    let accellerationX = 0;
    let accellerationY = 0;

    const minSpeed = clamp(step1Y - currentY, -2, 2);

    const minMax = (a, b) => {
      if (a > 0) {
        return Math.min(a, b);
      }
      return a + b;
    };
    while (t--) {
      await sleep(10);
      if (
        Math.abs(currentX - step1X) > 3 &&
        Math.abs(currentY - step1Y) > 3 &&
        !goingDown
      ) {
        xSpeed = minMax(0, (xSpeed + (step1X - currentX) / 10) / 2);
        ySpeed = minMax(minSpeed, (ySpeed + (step1Y - currentY) / 10) / 2);
        currentX += xSpeed;
        currentY += ySpeed;
      } else {
        if (!goingDown) {
          goingDown = true;
          accellerationX = (targetX - currentX) / 1000;
          accellerationY = Math.abs(Math.sqrt(targetY - currentY) + 500) / 1000;
        }
        goingDown = true;
        xSpeed += accellerationX *= 0.9;
        ySpeed += accellerationY;

        currentX += xSpeed;
        currentY += ySpeed;
      }
    }
    resolve();
  });

  // Almost never red, but it's fine like this
  const randomColor = Math.trunc(Math.random() * 0xffffff)
    .toString(16)
    .padStart(6, "0");
</script>

<div
  style="left:{currentX}px; top:{currentY}px; background-color: #{randomColor};"
/>

<style>


  div {
    position: fixed;
    height: 25px;
    width: 25px;
    background-color: #bbb;
    border-radius: 50%;
    z-index: 10;
    animation: disappear 2s ease-out;
  }
</style>
