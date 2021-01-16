export function getWidth(element) {
  return element
    ? Math.max(element.offsetWidth)
    : Math.max(
        document.body.scrollWidth,
        document.documentElement.scrollWidth,
        document.body.offsetWidth,
        document.documentElement.offsetWidth,
        document.documentElement.clientWidth
      );
}

/**
 * Assert a value to be within 2 other
 * @param {number} n Value to clamp
 * @param {number} min Minimum limit for the value
 * @param {number} max Maximum limit for the value
 */
Math.clamp = function (n = 0, min = -Infinity, max = Infinity) {
  return Math.min(Math.max(n, min), max);
};

export function debounce(callback, delay) {
  var timeout;
  return (...prop) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => {
      timeout = null;
      callback(...prop);
    }, delay);
  };
}
