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
    speedMS = distance / (pos.timestamp - time)

    lat = pos.coords.latitude
    lng = pos.coords.longitude
    time = pos.timestamp
  }
</script>

<div>
  <span>Latitude: {lat} | {deltaLat}</span>
  <br />
  <span>Longitude: {lng} | {deltaLng}</span>
  <br />
  <span>Speed: {Math.round(speedKmH)}Km/h or {Math.round(speedMS)}M/s</span>
</div>
