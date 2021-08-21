<script>
  import { useObservable } from "../utils/ScrollController";
  import { TopBarShown } from "../utils/Stores";
  import Compy from "../elements/compy/Compy.svelte";
  import FaLinkedin from "svelte-icons/fa/FaLinkedin.svelte";
  import FaGithub from "svelte-icons/fa/FaGithub.svelte";
  import { github, linkedin, email } from "../utils/External";
  import Flag from "../utils/Flags.svelte";
  import FaEnvelope from "svelte-icons/fa/FaEnvelope.svelte";

  const observable = useObservable("page1", {
    onIntersectionStart: () => TopBarShown.set(true),
    onIntersectionEnd: () => TopBarShown.set(false),
  });
</script>

<div id="page1" use:observable>
  <div class="content">
    <div class="sides" />
    <div class="sides center">
      <h1>Hi there! I'm Andrea</h1>
      <h3>Part time student, Developer for life!</h3>
      <p>
        I'm a Switzerland-based developer. Currently working at <a
          href="https://www.jacando.com/">Jacando AG</a
        >
        but always open for any kind of project.
        <br /><br /> I love wandering all over the world and I'm also trying to
        see as many places as possible. I speak 4 languages:
        <Flag name="IT" />, <Flag name="UK" /> and <Flag name="CH" />/<Flag
          name="DE"
        /> . Lately, I've been trying to learn
        <Flag name="ES" />.
        <br /><br />
        Did I say I love creating animations and interactive elements? ... maybe
        you've already noticed 😜

        <br /><br />
        If any of these intrigued you, do not hesitate to contact me, I'd love to
        share our experiences and see other's points of view
      </p>
      <div class="linkscontainer">
        <a class="sprite github" href={github}>
          <FaGithub />
        </a>
        <a class="sprite linkedin" href={linkedin}>
          <FaLinkedin />
        </a>
        <a class="sprite email" href={email}>
          <FaEnvelope />
        </a>
      </div>
    </div>
    <div class="sides" />
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
        width: 25%;
        opacity: 0.6;
        height: 100%;
        position: relative;
        @media only screen and (max-height: 1024px) {
          width: 15%;
        }
        &.center {
          display: flex;
          flex-direction: column;
          width: 50%;
          @media only screen and (max-height: 1024px) {
            width: 60%;
          }
          h1 {
            width: 100%;
            text-align: center;
            font-size: 4.5rem;
            word-wrap: break-word;
            @media only screen and (max-width: 768px) {
              font-size: 3rem;
            }
            margin-bottom: 0;
          }
          h3 {
            width: 100%;
            text-align: center;
            font-size: 2.5rem;
            @media only screen and (max-width: 768px) {
              margin: 0;
              font-size: 1.2rem;
            }
          }
          p {
            margin-top: 10%;
            @media only screen and (max-height: 1024px) {
              margin-top: 4%;
            }
            font-size: 1.5rem;
            @media only screen and (max-width: 768px) {
              font-size: 0.7rem;
            }
            :global(svg) {
              height: 1rem;
              @media only screen and (max-width: 768px) {
                height: 0.5rem;
              }
            }
          }

          div.linkscontainer {
            text-align: center;
            position: absolute;
            bottom: 0;
            width: 100%;
            .sprite {
              color: white;
              opacity: 1;
              height: 50px;
              width: 50px;
              display: inline-block;
              &.github {
                padding: 5px;
              }
              &.linkedin {
                padding: 5px;
              }
              &.email {
                padding: 5px;
              }
            }
          }
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
