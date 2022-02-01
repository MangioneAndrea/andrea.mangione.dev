<script>
  import { onMount } from "svelte";
  import { clamp } from "../helpers/CommonFunctions";
  import Arrow from "./environment/Arrow.svelte";
  import Cloud from "./environment/Cloud.svelte";
  import Sign from "./environment/Sign.svelte";
  import Sun from "./environment/Sun.svelte";
  import ForestBack from "./environment/trees/ForestBack.svelte";
  import Trees from "./environment/trees/Trees.svelte";
  $: scr = 0;

  let world;

  onMount(() => {
    document.scrollLeft = 0;
    world.addEventListener("wheel", (evt) => {
      evt.preventDefault();
      if (evt.deltaY) {
        world.scrollLeft += evt.deltaY;
        scr = clamp(document.scrollLeft / world.scrollWidth, 0, 1) * 100;
      }
      console.log(scr);
    });
  });
</script>

<div id="world" bind:this={world}>
  <div id="sky" />

  <div id="terrain" />
  <header>
    <h1>Hi I'm Andrea</h1>
    <h2>
      Part-time Student<br />
      Lifelong Developer
    </h2>
  </header>

  <Cloud left="10rem" />
  <Sun />
  <Sign left="30rem" bottom="9%">
    <Arrow x="90px" y="80px" />
    <Arrow x="210px" y="80px" rotate="180" />
  </Sign>
  <ForestBack />
  <div class="square" style={`left:${0}%; bottom:15%`} />
</div>

<style type="scss">
  #world {
    width: inherit;
    height: inherit;
    overflow: scroll;
    scroll-behavior: smooth;
    position: absolute;
    header {
      position: absolute;
      top: 15%;
      width: 400px;
      font-family: "Cloud";
      text-align: center;
      h1 {
        font-size: xxx-large;
      }
      h2 {
        font-size: xx-large;
      }
    }

    div {
      &#sky {
        position: fixed;
        background-color: #023e8a; //night 001219
        height: 85%;
        width: 100%;
      }
      &#terrain {
        position: fixed;
        background: linear-gradient(to left, #e66465, #9198e5);
        height: 15%;
        width: 100%;
        bottom: 0;
      }
      &.square {
        width: 50px;
        height: 50px;
        background-color: red;
        position: fixed;
      }
    }
  }
</style>
