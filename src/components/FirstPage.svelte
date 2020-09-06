<script>
  import { addObservable, useObservable } from "../utils/ScrollController";
  import { TopBarShown } from "../utils/Stores";
  import anime from "animejs";
  import { initFireworks } from "../helpers/animejs/fireworks";
  import { createText } from "../helpers/pixijs/ParticleText";
  import { initPixi } from "../helpers/pixijs/PixiComponent";
  import * as PIXI from "pixi.js";

  const observable = useObservable("page1", {
    onIntersectionStart: () => TopBarShown.set(true),
    onIntersectionEnd: () => TopBarShown.set(false),
  });

  const createPixi = () => {
    const pixiApp = initPixi("#page1", { backgroundColor: 0xe5e5e5 });
    createText(pixiApp, "Hi there! I'm Andrea.", { size: 45 });
  };
</script>

<style>
  div {
    height: calc(100% - 4rem);
    background-color: #000000;
    padding-top: 4rem;
  }
</style>

<div id="page1" use:observable use:createPixi />
