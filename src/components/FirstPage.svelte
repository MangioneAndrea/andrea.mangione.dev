<script>
  import { addObservable, useObservable } from "../utils/ScrollController";
  import { TopBarShown } from "../utils/Stores";
  import anime from "animejs";
  import { initFireworks } from "../helpers/animejs/fireworks";

  const observable = useObservable("page1", {
    onIntersectionStart: () => TopBarShown.set(true),
    onIntersectionEnd: () => TopBarShown.set(false),
  });

  const animate = () => {
    const firstText = document.querySelectorAll(".small");
    const secondText = document.querySelectorAll(".medium");
    const bottomText = document.querySelectorAll(".bottom");
    anime({
      targets: firstText,
      baseFrequency: 0,
      scale: 1.1,
      loop: false,
      easing: "easeInOutExpo",
    });
    anime({
      targets: secondText,
      baseFrequency: 0,
      scale: 1.1,
      x: 100,
      y: 200,
      loop: 1,
      easing: "easeInOutExpo",
      delay: 400,
      complete: () => {
        anime({
          targets: secondText,
          baseFrequency: 0,
          scale: 1,
          x: 100,
          y: 200,
          loop: 1,
          easing: "easeInOutExpo",
          delay: 400,
        });
      },
    });
    anime({
      delay: 975,
      targets: bottomText,
      y: 280,
      loop: false,
    });
    anime({
      delay: 1000,
      targets: bottomText,
      rotate: 40,
      duration: 700,
      direction: "alternate",
      loop: 2,
    });
  };
</script>

<style>
  div {
    height: calc(100% - 4rem);
    background-color: #e5e5e5;
    padding-top: 4rem;
  }
  h3 {
    text-align: center;
    height: 100%;
  }
  svg {
    max-width: 40em;
    margin: auto;
    margin-top: 10vh;
  }
  .small {
    font-family: "Dancing Script", cursive;
    align-self: center;
    font-size: 4em;
  }
  .medium {
    font-family: "Dancing Script", cursive;
    align-self: center;
    font-size: 6em;
  }
  .bottom {
    font-family: "Alata", sans-serif;
    align-self: center;
    font-size: 2.5em;
  }
  .fireworks {
    left: 0;
    top: 0;
    position: absolute;
  }
</style>

<div id="page1" use:observable>

  <h3 use:animate>
    <svg
      version="1.1"
      class="subtitle"
      xmlns="http://www.w3.org/2000/svg"
      xmlns:xlink="http://www.w3.org/1999/xlink"
      viewBox="0 0 730.2 500.6"
      xml:space="preserve">
      <text x="100" y="80" class="small">Hi there !</text>
      <text x="80" y="190" class="medium">I'm Andrea</text>
      <text x="20" y="250" class="bottom">
        Full stack developer and student!
      </text>
    </svg>
    <canvas class="fireworks" use:initFireworks />
  </h3>

</div>
