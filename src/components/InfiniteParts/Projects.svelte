<script>
  import KotlinFirmata from "./Projects/KotlinFirmata/KotlinFirmata.svelte";
  import AirHockey from "./Projects/AirHockey/AirHockey.svelte";
</script>

<div id="projects">
  <h1>Projects</h1>
  <KotlinFirmata />
  <AirHockey />
</div>

<style type="text/scss">
  div#projects {
    margin: auto;
    width: 100%;
    text-align: center;
    h1 {
      font-size: 4.5rem;
      margin: auto;
    }
  }
</style>
