<script>
  export let right = 50;
  export let top = 30;
  export let width = 150;
  const white = "#f2f9fe";
  const azure = "#d6f0fd";
</script>

<svg
  viewBox="0 0 1000 600"
  style={`
    position: fixed;
    right: ${right}px;
    top: ${top}px;
    width: ${width}px;
    `}
  ><defs>
    <linearGradient id="CloudGrad1" x1="0" x2="0" y1="0" y2="1">
      <stop stop-color={white} offset="5%" />
      <stop stop-color={azure} offset="100%" />
    </linearGradient>
  </defs>
  <circle r="150" cx="300" cy="250" class="small" fill={white} />
  <circle r="250" cx="600" cy="250" class="big" fill={white} />
  <rect
    x="0"
    y="250"
    width="1000"
    height="350"
    rx="175"
    fill="url(#CloudGrad1)"
  />
</svg>
