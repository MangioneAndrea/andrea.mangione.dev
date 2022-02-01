<script>
  import { onMount } from "svelte";
  import { clamp } from "../helpers/CommonFunctions";
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

  <Cloud />
  <Sun />
  <Sign left="30rem" bottom="9%">
    <g transform="scale(0.5,0.5) translate(0,-140) ">
      <path
        d="m415.49 237.76h-78.996c-10.914 0-19.75 8.832-19.75 19.75v78.996c0 10.895 8.832 19.75 19.75 19.75h78.996c10.918 0 19.75-8.8516 19.75-19.75v-78.996c0-10.914-8.832-19.75-19.75-19.75zm-19.941 73.02-9.6836-9.6797v26.613h-19.75v-26.418l-9.0273 9.1367-14.039-13.887 32.902-33.289 33.559 33.559zm19.941 84.973h-78.996c-10.914 0-19.75 8.832-19.75 19.746v78.996c0 10.898 8.832 19.75 19.75 19.75h78.996c10.918 0 19.75-8.8516 19.75-19.75v-78.996c0-10.914-8.832-19.746-19.75-19.746zm-39.457 92.996-33.559-33.559 13.965-13.965 9.6797 9.6797v-26.652h19.75v26.457l9.0273-9.1367 14.039 13.887zm-98.766-92.996h-78.996c-10.895 0-19.75 8.832-19.75 19.746v78.996c0 10.918 8.8516 19.75 19.75 19.75h78.996c10.918 0 19.75-8.832 19.75-19.75v-78.996c0-10.914-8.832-19.746-19.75-19.746zm-8.7734 69.121h-26.438l9.1367 9.0273-13.887 14.039-33.289-32.902 33.559-33.559 13.965 13.965-9.6797 9.6797h26.633zm285.24-69.121h-78.996c-10.918 0-19.75 8.8711-19.75 19.746v78.996c0 10.918 8.832 19.75 19.75 19.75h78.996c10.879 0 19.75-8.832 19.75-19.75v-78.996c0-10.875-8.8711-19.746-19.75-19.746zm-39.305 92.766-13.965-13.965 9.6836-9.6797h-26.652v-19.75h26.457l-9.1367-9.0273 13.887-14.039 33.289 32.902z"
      />
    </g>
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
