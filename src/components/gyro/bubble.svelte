<script>
  import { onMount } from 'svelte'
  import { decimalPlaces } from '../../helpers/CommonFunctions'

  let x = 0
  let y = 0
  let top = 50
  let left = 50

  const onRotate = ({ gamma, beta }) => {
    x = decimalPlaces(gamma, 2)
    y = decimalPlaces(beta, 2)
    gamma = (gamma / 180) * Math.PI
    beta = (beta / 180) * Math.PI

    left = 50 - Math.sin(gamma) * Math.cos(beta) * 40
    top = 50 - Math.sin(beta) * 40
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
<div class="absolute flex p-5 w-full h-full">
  <div class="h-[90%] my-auto w-2 rounded bg-gray-400 flex items-center">
    <div>
      <div class="-translate-x-2 origin-left -rotate-90">
        {-y}°
      </div>
    </div>
    <div
      style={`top: ${top}%;`}
      class={`absolute -translate-y-2 h-4 w-2 rounded bg-gray-600`}
    ></div>
  </div>
  <div class="h-2 mx-auto w-[90%] rounded bg-gray-400">
    <div
      style={`left: ${left}%`}
      class={`absolute -translate-x-2 w-4 h-2 rounded bg-gray-600`}
    ></div>
    <div class="w-full text-center -translate-y-6">
      {-x}°
    </div>
  </div>
</div>
