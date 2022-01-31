<script>
  import "./helpers/CommonFunctions";
  import World from "./world/World.svelte";
</script>

<svelte:head>
  <title>Andrea's Portfolio</title>
</svelte:head>

<World />
<main />
<footer />

<link
  href="https://fonts.googleapis.com/css2?family=Rubik:wght@300&display=swap"
  rel="stylesheet"
/>
<link
  href="https://fonts.googleapis.com/css2?family=Dancing+Script&display=swap"
  rel="stylesheet"
/>
<link
  href="https://fonts.googleapis.com/css2?family=Alata&display=swap"
  rel="stylesheet"
/>

<style type="scss">
  :global(body) {
    padding: 0;
    font-family: "Rubik", sans-serif;
  }
  :global(html) {
    overflow-x: hidden;
    box-sizing: border-box;
    text-rendering: optimizeLegibility;
  }
</style>
