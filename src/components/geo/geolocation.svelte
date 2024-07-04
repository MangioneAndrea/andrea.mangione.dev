<script>
  import { onMount } from 'svelte'

  let lat = 0
  let lng = 0
  let time = 0

  let distance = 0

  let speedKmH = 0
  let speedMS = 0

  let deltaLat = 0
  let deltaLng = 0

  onMount(async () => {
    navigator.geolocation.getCurrentPosition(update)

    navigator.geolocation.watchPosition(update)
  })

  function haversine(lat2, lon2) {
    const R = 6371
    const dLat = toRadians(lat2 - lat)
    const dLon = toRadians(lon2 - lng)
    const a =
      Math.sin(dLat / 2) ** 2 +
      Math.cos(toRadians(lat)) *
        Math.cos(toRadians(lat2)) *
        Math.sin(dLon / 2) ** 2
    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a))
    return R * c
  }

  function toRadians(degrees) {
    return degrees * (Math.PI / 180)
  }

  /** @param {GeolocationPosition} pos */
  function update(pos) {
    deltaLat = ((lat - pos.coords.latitude) * 1000) / (pos.timestamp - time)
    deltaLng = ((lng - pos.coords.longitude) * 1000) / (pos.timestamp - time)

    distance = haversine(pos.coords.latitude, pos.coords.longitude)

    speedKmH = distance / ((pos.timestamp - time) / (60 * 60 * 1000))
    speedMS = speedKmH / 3.6

    lat = pos.coords.latitude
    lng = pos.coords.longitude
    time = pos.timestamp
  }

  const [c1, c2, c3] = [160, 147, 125].map((r) =>
    new Array(41)
      .fill(0)
      .map((_, i) => [
        r * Math.sin((((i + 10) / 60) * 360) / (180 / Math.PI)) + 175,
        r * Math.cos((((i + 10) / 60) * 360) / (180 / Math.PI)) + 175,
      ])
  )

  function pointInCircle(speed) {
    return [
      150 * Math.sin(((-speed / 240) * 360 - 60) / (180 / Math.PI)) + 175,
      150 * Math.cos(((-speed / 240) * 360 - 60) / (180 / Math.PI)) + 175,
    ]
  }
</script>

<div>
  <span>Latitude: {lat}</span>
  <br />
  <span>Longitude: {lng}</span>
  <br />
  <span>Speed: {Math.round(speedKmH)}Km/h or {Math.round(speedMS)}M/s</span>

  <svg
    width="350"
    height="270"
    viewBox="0 0 349 266"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    {#each new Array(41).fill(0).map((_, i) => i) as idx}
      <line
        x1={c1[idx][0]}
        y1={c1[idx][1]}
        x2={c2[idx][0]}
        y2={c2[idx][1]}
        stroke="#CFF80B"
        stroke-width="2"
      />
      {#if idx % 5 == 0}
        <line
          x1={c1[idx][0]}
          y1={c1[idx][1]}
          x2={c3[idx][0]}
          y2={c3[idx][1]}
          stroke="#CFF80B"
          stroke-width="2"
        />
      {/if}
    {/each}

    <circle cx="175" cy="175" r="170" stroke="#CFF80B" stroke-width="4" />
    <circle cx="175" cy="175" r="170" stroke="#80808050" stroke-width="4" />
    <circle cx="175" cy="175" r="60" stroke="#80808050" stroke-width="4" />
    <circle cx="175" cy="175" r="5" fill="red" stroke-width="4" />

    <text x="79" y="235">0</text>
    <text x="65" y="182">20</text>
    <text x="73" y="128">40</text>
    <text x="110" y="85">60</text>
    <text x="160" y="75">80</text>
    <text x="215" y="85">100</text>
    <text x="252" y="128">120</text>
    <text x="260" y="182">140</text>
    <text x="246" y="235">160</text>
    <text x="150" y="210" id="unit">Km/h</text>
    <text x="50%" y="160" id="speed" text-anchor="middle"
      >{Math.round(speedKmH)}</text
    >

    <line
      x1={175}
      y1={175}
      x2={20}
      y2={175}
      style={`transform: rotate(${(speedKmH / 240) * 360 - 30}deg)`}
      stroke="red"
      stroke-width="5"
      id="indicator"
    />
  </svg>
</div>

<style>
  text {
    fill: white;
    font-size: 0.9em;
  }

  #unit {
    fill: gray;
    font-size: 1em;
    font-weight: bold;
  }

  #speed {
    font-size: 2em;
    font-weight: bold;
  }

  #indicator {
    transform-origin: 175px 175px;
    transition: all ease-in 100ms;
  }
</style>
