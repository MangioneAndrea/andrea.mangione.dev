<script>
  import { useObservable } from "../utils/ScrollController";
  import { TopBarShown } from "../utils/Stores";
  import { initPixi } from "../helpers/pixijs/PixiComponent";

  const observable = useObservable("page1", {
    onIntersectionStart: () => TopBarShown.set(true),
    onIntersectionEnd: () => TopBarShown.set(false),
  });

  const createPixi = () => {
    const leftRain = initPixi("#leftRain", { backgroundColor: 0xe5e5e5 });
    const rightRain = initPixi("#rightRain", { backgroundColor: 0xe5e5e5 });
    leftRain.rain();
    rightRain.rain();
  };
</script>

<style type="text/scss">
  div#page1 {
    height: calc(100% - 4rem);
    padding-top: 4rem;
    background-color: #e5e5e5;

    div.content {
      width: 100%;
      position: absolute;
      display: flex; /* establish flex container */
      flex-direction: row; /* default value; can be omitted */
      flex-wrap: nowrap; /* default value; can be omitted */
      justify-content: space-between;
      height: 100%;
      height: calc(100% - 5rem);

      div.sides {
        &.center {
          //margin: auto;
          width: 50%;
          h1 {
            width: 100%;
            text-align: center;
            font-size: 4.5rem;
            word-wrap: break-word;
          }
          h3 {
            width: 100%;
            text-align: center;
            font-size: 2.5rem;
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
</style>

<div id="page1" use:observable use:createPixi>
  <div class="content">
    <div class="sides" id="leftRain" />
    <div class="sides center">
      <h1>Hi there! I'm Andrea</h1>
      <h3>Part time student, Developer for life!</h3>
    </div>
    <div class="sides" id="rightRain" />
  </div>
</div>
