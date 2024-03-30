<script>
  import { onMount } from 'svelte'
  import { decimalPlaces } from '../../helpers/CommonFunctions'

  let x = 0
  let y = 0
  let top = 50
  let left = 50
  let t = 50
  let l = 50

  const onRotate = (gyro) => {
    x = decimalPlaces(gyro.beta, 2)
    y = decimalPlaces(gyro.gamma, 2)
    t = 50 - gyro.beta
    l = 50 - gyro.gamma
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
  /*

  */
</script>

<div
  id="bubble"
  style={`top: ${top}%; left: ${left}%`}
  class={'absolute h-8 w-8 -translate-y-4 -translate-x-4 bg-lime-500 rounded-full w-4'}
></div>
<div
  id="bubble2"
  style={`top: ${t}%; left: ${l}%`}
  class={'absolute h-8 w-8 -translate-y-4 -translate-x-4 bg-blue-500 rounded-full w-4'}
></div>
<div class="absolute flex p-5 w-full h-full">
  <div class="h-[90%] my-auto w-2 rounded bg-gray-400">
    <div class="w-full pb-4 -translate-x-[1.125rem] items-center h-full flex">
      <div class="text-center h-fit -rotate-90">
        {x}°
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
      {y}°
    </div>
  </div>
</div>
