export function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

export function decimalPlaces(num, places) {
  return Math.trunc(num * 10 ** places) / 10 ** places
}

/**
 * Assert a value to be within 2 other
 * @param {number} n Value to clamp
 * @param {number} min Minimum limit for the value
 * @param {number} max Maximum limit for the value
 *
 * @returns {number} The clamped value
 */
export function clamp(n = 0, min = -Infinity, max = Infinity) {
  return Math.min(Math.max(n, min), max)
}
/**
 * Enclose a function with a controller which avoids repetitions of the function.
 * Only the latest call within the delay of idle will fire the function
 * @param {function} callback function wrapped by the debouncer
 * @param {number} delay delay before the the callback is fired
 */
export function debounce(callback, delay) {
  var timeout
  return (...props) => {
    const args = { props, shouldAbort: false }
    clearTimeout(timeout)
    timeout = setTimeout(() => {
      timeout = null
      callback(args)
    }, delay)
    return () => (args.shouldAbort = true)
  }
}

/**
 * Avoid one function to be called again, untill it has ended
 * @param {function} callback function wrapped by the defer
 */
export function deferAsync(callback) {
  let done = true
  return async (...prop) => {
    if (done) {
      done = false
      await callback(...prop)
      done = true
    }
  }
}
/**
 * Avoid one function to be called again, untill it has ended
 * @param {function} callback function wrapped by the defer
 */
export function defer(callback) {
  let done = true
  return (...prop) => {
    if (done) {
      done = false
      callback(...prop)
      done = true
    }
  }
}
/**
 * @return {string} a svg curve value between two points
 */
export function getIdealCurve({ p1x, p1y, p2x, p2y, minCurve = 0 }) {
  // mid-point of line:
  var mpx = (p2x + p1x) * 0.5
  var mpy = (p2y + p1y) * 0.5
  // angle of perpendicular to line:
  var theta = Math.atan2(p2y - p1y, p2x - p1x) - Math.PI / 2
  // distance of control point from mid-point of line:
  var offsetX = Math.min((p2x - p1x) / -2, minCurve)
  // location of control point:
  var c1x = mpx + offsetX * Math.cos(theta)
  var c1y = mpy + offsetX * Math.sin(theta)
  // construct the command to draw a quadratic curve
  return `M${p1x} ${p1y} Q${c1x} ${c1y} ${p2x} ${p2y}`
}

export function oneRandomOutOf(...arr) {
  return arr[Math.floor(Math.random() * arr.length)]
}

export class RingedBuffer {
  index = 0
  size = 0
  constructor(maxSize) {
    this.buffer = new Array(maxSize);
    this.maxSize = maxSize
  }

  push(num) {
    if (Number.isNaN(num)) return
    this.buffer[this.index] = num
    this.index++
    if (this.index >= this.maxSize) this.index = 0
    if (this.size < this.maxSize) this.size++
  }

  get average() {
    let sum = 0

    for (let i = 0; i < this.size; i++) {
      sum += this.buffer[i]
    }
    return sum / (this.size || 1)
  }
}
