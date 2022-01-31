<script>
  import { onMount } from "svelte";
  import { clamp } from "../helpers/CommonFunctions";
  import Cloud from "./environment/Cloud.svelte";
  import Sign from "./environment/Sign.svelte";
  import Sun from "./environment/Sun.svelte";
  import Trees from "./environment/trees/Trees.svelte";
  $: scr = 0;

  onMount((e) => {
    document.scrollLeft = 0;
    document.getElementById("world").addEventListener("wheel", (evt) => {
      evt.preventDefault();
      if (evt.deltaY) {
        document.scrollLeft += evt.deltaY;
        scr =
          clamp(document.scrollLeft / document.body.scrollWidth, 0, 1) * 100;
      }
      console.log(scr);
    });
  });
</script>

<div id="world">
  <div id="sky" />

  <div id="terrain" />
  <Cloud />
  <Sun />
  <Sign />
  <Trees />

  <div class="square" style={`left:${0}%; bottom:15%`} />
</div>

<style type="scss">
  #world {
    width: inherit;
    height: inherit;
    div {
      width: 200%;
      &#sky {
        background-color: #023e8a; //night 001219
        height: 85%;
      }
      &#terrain {
        background: linear-gradient(to left, #e66465, #9198e5);
        height: 15%;
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
