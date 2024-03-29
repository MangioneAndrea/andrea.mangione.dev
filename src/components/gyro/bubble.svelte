<script>
  import { onMount } from 'svelte'

  let top = 50
  let left = 50

  const onRotate = (gyro) => {
    let t = 50 - gyro.beta
    let l = 50 - gyro.gamma 
    let distance = Math.sqrt(Math.pow(t - 50, 2) + Math.pow(l - 50, 2))

    if (distance > 40) {
      const ratio = 40 / distance
      top = (t - 50) * ratio + 50
      left = (l - 50) * ratio + 50
    } else {
      top = t
      left = l
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
