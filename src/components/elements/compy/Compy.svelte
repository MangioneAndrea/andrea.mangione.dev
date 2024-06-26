<script>
  import { onMount } from 'svelte'

  import {
    debounce,
    clamp,
    getIdealCurve,
    deferAsync,
    sleep,
    defer,
    oneRandomOutOf,
  } from '../../../helpers/CommonFunctions.js'
  import Ball from './Ball.svelte'

  let leftPupil, rightPupil
  let leftEye, rightEye
  let rightHand
  let rightArm, leftArm
  let body, svg
  let lastKnownMouseX, lastKnownMouseY

  let balls = new Set()

  let compyRight = 0

  const adaptArm = () => {
    const bodyX = Number(body.getAttribute('x'))
    const bodyY = Number(body.getAttribute('y'))
    const bodyWidth = body.getBoundingClientRect().width
    const bodyHeight = body.getBoundingClientRect().height

    const handX = Number(rightHand.getAttribute('x'))
    const handY = Number(rightHand.getAttribute('y'))
    const handWidth = rightHand.getBoundingClientRect().width
    const handHeight = rightHand.getBoundingClientRect().height

    rightArm.setAttribute(
      'd',
      getIdealCurve({
        p1x: bodyX + bodyWidth,
        p1y: bodyY + bodyHeight / 3,
        p2x: handX + handWidth / 2,
        p2y: handY + handHeight,
      })
    )

    leftArm.setAttribute(
      'd',
      getIdealCurve({
        p1x: bodyX,
        p1y: bodyY + bodyHeight / 3,
        p2x: bodyX,
        p2y: bodyY + bodyHeight / 1.3,
        minCurve: -50,
      })
    )
  }

  const teleport = deferAsync(async () => {
    let random = compyRight

    while (Math.abs(random - compyRight) < 30) {
      random =
        Math.random() *
        (svg.parentElement.parentElement.clientWidth - svg.clientWidth)
    }

    compyRight = random
  })

  const left = {
    targetX: 0,
    currentX: 0,
    targetY: 0,
    currentY: 0,
  }

  const right = {
    targetX: 0,
    currentX: 0,
    targetY: 0,
    currentY: 0,
  }

  let pupilLeft = `--pupil-x: ${left.currentX}px; --pupil-y: ${left.currentY}px;`
  let pupilRight = `--pupil-x: ${right.currentX}px; --pupil-y: ${right.currentY}px;`

  const resetEyes = debounce(() => {
    left.targetX = 0
    left.targetY = 0
    right.targetX = 0
    right.targetY = 0
  }, 1000)

  // No, css animations are not enough performant
  setInterval(() => {
    if (left.currentX !== left.targetX) {
      left.currentX += clamp(left.targetX - left.currentX, -1, 1)
    }
    if (left.currentY !== left.targetY) {
      left.currentY += clamp(left.targetY - left.currentY, -1, 1)
    }
    if (right.currentX !== right.targetX) {
      right.currentX += clamp(right.targetX - right.currentX, -1, 1)
    }
    if (right.currentY !== right.targetY) {
      right.currentY += clamp(right.targetY - right.currentY, -1, 1)
    }
    pupilLeft = `--pupil-x: ${left.currentX}px; --pupil-y: ${left.currentY}px;`
    pupilRight = `--pupil-x: ${right.currentX}px; --pupil-y: ${right.currentY}px;`
  }, 1000 / 60 /* 60 fps */)

  const moveEye = (_x, _y, eye, pup, target) => {
    const eyeX = eye.getBoundingClientRect().x
    const eyeY = eye.getBoundingClientRect().y

    const eyeW = eye.getBoundingClientRect().width
    const eyeH = eye.getBoundingClientRect().height
    const pWidth = pup.getBoundingClientRect().width
    const pHeight = pup.getBoundingClientRect().height

    const centerX = eyeW / 2 + eyeX
    const centerY = eyeH / 2 + eyeY

    const xBound = (eyeW - pWidth) / 2
    const yBound = eyeH - pHeight

    target.targetX = Math.round(clamp(_x - centerX, -xBound, xBound))
    target.targetY = Math.round(clamp(_y - centerY + pHeight / 2, 0, yBound))
  }

  onMount(() => {
    adaptArm()
    teleport()
  })

  let abortMouseStalker
  const onMouseMove = (evt) => {
    abortMouseStalker?.()
    if (!leftEye || !rightEye) return
    lastKnownMouseX = evt.clientX
    lastKnownMouseY = evt.clientY

    moveEye(evt.clientX, evt.clientY, leftEye, leftPupil, left)
    moveEye(evt.clientX, evt.clientY, rightEye, rightPupil, right)
    resetEyes()
    abortMouseStalker = mouseStalker()
  }

  const clearBalls = debounce(() => {
    balls.clear()
    balls = balls
  }, 2000)

  // Trickery with the heap, args might change, since it's async, so destructuring is a bad idea
  const mouseStalker = debounce(async (args) => {
    switch (oneRandomOutOf('cursor', 'left-right')) {
      case 'cursor': {
        moveEye(lastKnownMouseX, lastKnownMouseY, leftEye, leftPupil, left)
        moveEye(lastKnownMouseX, lastKnownMouseY, rightEye, rightPupil, right)
        break
      }
      case 'left-right': {
        moveEye(0, Infinity, leftEye, leftPupil, left)
        moveEye(0, Infinity, rightEye, rightPupil, right)
        await sleep(1000)
        if (args.shouldAbort) return
        moveEye(Infinity, Infinity, leftEye, leftPupil, left)
        moveEye(Infinity, Infinity, rightEye, rightPupil, right)
        await sleep(1000)
        if (args.shouldAbort) return
        moveEye(0, Infinity, leftEye, leftPupil, left)
        moveEye(0, Infinity, rightEye, rightPupil, right)
        break
      }
    }
    if (args.shouldAbort) return
    resetEyes()
    if (args.shouldAbort) return
    abortMouseStalker = mouseStalker()
  }, 4000)

  const shootBall = defer((evt) => {
    const ball = {
      x: rightHand.getBoundingClientRect().x,
      y: rightHand.getBoundingClientRect().y,
      targetX: evt.clientX,
      targetY: evt.clientY,
    }
    balls.add(ball)
    balls = balls

    clearBalls()
  }, 50)
</script>

<svelte:window on:mousemove={onMouseMove} on:mousedown={shootBall} />
{#each [...balls] as ball}
  <svelte:component this={Ball} {...ball} />
{/each}
<svg
  version="1.1"
  xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://www.w3.org/1999/xlink"
  width="150"
  height="150"
  bind:this={svg}
  on:mouseup={teleport}
  style="--compy-right: {compyRight}px;"
>
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
      <rect x="59" y="43" class="eye" rx="5" bind:this={rightEye} />
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
  <!-- Hand, x and y are not supported by G element, but needed for calculations -->
  <g
    id="hand"
    transform="translate(110, 37)"
    x="110"
    y="37"
    bind:this={rightHand}
  >
    <rect x="3" width="5" height="15" rx="5" />
    <rect x="3" width="5" height="15" rx="5" />
    <rect x="22" width="5" height="15" rx="5" />
    <polygon points="0,13 30,13 20,28 10,28" class="hand" />
  </g>
  <!-- Body -->
  <rect
    id="body"
    bind:this={body}
    x="25"
    y="84"
    width="55"
    height="63"
    class="skin"
  />
  <path
    fill="transparent"
    stroke="black"
    stroke-width="6"
    bind:this={rightArm}
  />
  <path
    fill="transparent"
    stroke="black"
    stroke-width="6"
    bind:this={leftArm}
  />
</svg>

<style>
  .pupil {
    will-change: transform;
    transform: translate(var(--pupil-x), var(--pupil-y));
  }

  svg {
    will-change: transform;
    right: var(--compy-right);
    transform: translateX(calc(var(--compy-right) + 1.5rem));
    top: 10%;
    background-color: transparent;
    transition: all ease-in-out 0.2s;
  }
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
</style>
