<script>
  import { useObservable } from "../utils/ScrollController";
  import { TopBarShown } from "../utils/Stores";
  import Compy from "../elements/compy/Compy.svelte";
  import FaLinkedin from "svelte-icons/fa/FaLinkedin.svelte";
  import FaGithub from "svelte-icons/fa/FaGithub.svelte";
  import RainHandler from "../elements/rain/RainHandler.svelte";
  import { openGithub, openlinkedin } from "../utils/External";

  let right, left;
  const observable = useObservable("page1", {
    onIntersectionStart: () => TopBarShown.set(true),
    onIntersectionEnd: () => TopBarShown.set(false),
  });
</script>

<div id="page1" use:observable>
  <div class="content">
    <div bind:this={right} class="sides" id="leftRain">
      <RainHandler container={right} />
    </div>
    <div class="sides center">
      <h1>Hi there! I'm Andrea</h1>
      <h3>Part time student, Developer for life!</h3>
      <div class="linkscontainer">
        <div class="sprite github" on:click={openGithub}>
          <FaGithub />
        </div>
        <div class="sprite linkedin" on:click={openlinkedin}>
          <FaLinkedin />
        </div>
      </div>
    </div>
    <div bind:this={left} class="sides" id="rightRain">
      <RainHandler container={left} />
    </div>
    <div id="compy">
      <Compy />
    </div>
  </div>
</div>

<style type="text/scss">
  div#page1 {
    height: calc(100% - 4rem);
    padding-top: 4rem;
    background-color: #252627;

    div.content {
      width: 100%;
      position: absolute;
      display: flex;
      flex-direction: row;
      flex-wrap: nowrap;
      justify-content: space-between;
      height: calc(100% - 5rem);

      div.sides {
        height: 100%;
        position: relative;
        &.center {
          //margin: auto;
          width: 50%;
          h1 {
            width: 100%;
            text-align: center;
            font-size: 4.5rem;
            word-wrap: break-word;
            @media only screen and (max-width: 768px) {
              font-size: 3rem;
            }
          }
          h3 {
            width: 100%;
            text-align: center;
            font-size: 2.5rem;
            @media only screen and (max-width: 768px) {
              font-size: 1.5rem;
            }
          }
          div.linkscontainer {
            text-align: center;
            position: absolute;
            bottom: 0;
            width: 100%;
            .sprite {
              height: 50px;
              width: 50px;
              display: inline-block;
              &.github {
                padding: 5px;
              }
              &.linkedin {
                padding: 5px;
              }
            }
          }
        }
        &#leftRain {
          width: 25%;
          opacity: 0.6;
        }
        &#rightRain {
          width: 25%;
          opacity: 0.6;
        }
      }
    }
  }
  div#compy {
    display: inline-block;
    position: absolute;
    bottom: -50px;
    right: 0;
  }
</style>
