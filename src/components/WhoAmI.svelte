<script>
  import anime from "animejs";

  import Compy from "../elements/Compy/Compy.svelte";
  import { useObservable } from "../utils/ScrollController";

  const observable = useObservable("animDescription", {
    onIntersectionStart: () => {
      anime({
        targets: "#compy",
        right: "0.5rem",
      });
    },
  });
</script>

<div id="whoAmI" use:observable>
  <div id="animDescription">
    <div id="compy">
      <Compy />
    </div>
    <p>
      I'm an attention-grabbing animations fan and I always try to create some
      in an elegant way!
    </p>
  </div>
</div>

<style type="text/scss">
  div#whoAmI {
    height: calc(100% - 4rem);
    background-color: #772f1a;
    padding-top: 4rem;
    div#animDescription {
      p {
        width: calc(100% - 30rem);
        margin: auto;
      }
      #compy {
        display: inline-block;
        position: absolute;
        right: -250px;
      }
    }
  }
</style>
