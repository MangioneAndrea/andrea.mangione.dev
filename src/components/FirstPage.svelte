<script>
  import { useObservable } from "../utils/ScrollController";
  import { TopBarShown } from "../utils/Stores";
  import { initPixi } from "../helpers/pixijs/PixiComponent";

  const observable = useObservable("page1", {
    onIntersectionStart: () => TopBarShown.set(true),
    onIntersectionEnd: () => TopBarShown.set(false),
  });

  const createPixi = () => {
    const pixiApp = initPixi("#page1", { backgroundColor: 0xe5e5e5 });
    pixiApp.rain();
    //createText(pixiApp, "Hi there! I'm Andrea.", { size: 45 });
  };
</script>

<style type="text/scss">
  div {
    height: calc(100% - 4rem);
    padding-top: 4rem;
  }
  div.content {
    width: 100%;
    position: absolute;
  }
</style>

<div id="page1" use:observable use:createPixi>
  <div class="content">hi</div>
</div>
