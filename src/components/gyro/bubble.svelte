<script>
  import { onMount } from 'svelte'
  import { clamp } from '../../helpers/CommonFunctions.js'

  let top = 50
  let left = 50

  const onRotate = (gyro) => {
    let t = clamp(50 - gyro.beta, 10, 91)
    let l = clamp(50 - gyro.gamma, 10, 91)

    let distance = Math.sqrt(Math.pow(t - 50, 2) + Math.pow(l - 50, 2))
    let n = 40
    while (distance > 45 && n--) {
      if (t > 50) t--
      if (l > 50) l--
      if (t < 50) t++
      if (l < 50) l++
      distance = Math.sqrt(Math.pow(t - 50, 2) + Math.pow(l - 50, 2))
    }

    top = t
    left = l
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
