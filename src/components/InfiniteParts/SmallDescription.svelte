<script>
  import anime from "animejs";
  import Compy from "../../elements/compy/Compy.svelte";

  import { useObservable } from "../../utils/ScrollController";
  const observable = useObservable("animDescription", {
    onIntersectionStart: () => {
      anime({
        targets: "#compy",
        right: "0.5rem",
      });
    },
  });
</script>

<div id="animDescription" use:observable>
  <div id="compy">
    <Compy />
  </div>
  <p class="typewriter">
    I'm an attention-grabbing animations fan and I always try to create some in
    an elegant way!
  </p>
</div>

<style type="text/scss">
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
</style>
