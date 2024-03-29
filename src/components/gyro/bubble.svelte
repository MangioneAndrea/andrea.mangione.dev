<script>
  import { onMount } from 'svelte'

  let top = 50
  let left = 50

  const onRotate = (gyro) => {
    let distance = Math.sqrt(
      Math.pow(gyro.beta - 50, 2) + Math.pow(gyro.gamma - 50, 2)
    )

    if (distance > 50) {
      const ratio = 50 / distance
      top = (gyro.beta - 50) * ratio + 50
      left = (gyro.gamma - 50) * ratio + 50
    } else {
      top = gyro.beta
      left = gyro.gamma
    }
  }

  onMount(async () => {
    addEventListener('deviceorientation', onRotate)
  })
</script>

<div
  id="bubble"
  style={`top: ${top}%; left: ${left}%`}
  class={'absolute h-8 w-8 -translate-y-4 -translate-x-4 bg-lime-500 rounded-full w-4'}
></div>
