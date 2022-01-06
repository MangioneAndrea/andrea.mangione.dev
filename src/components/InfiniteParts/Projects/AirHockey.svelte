<script>
  import CodeEditor from "../../../elements/codeEditor/CodeEditor.svelte";
  import LanguageList from "../../../elements/containers/LanguageList.svelte";
  import Go from "devicon/icons/go/go-original-wordmark.svg";
  import FaArrowRight from "svelte-icons/fa/FaArrowRight.svelte";
</script>

<div class="root">
  <h2>AirHockey</h2>
  <LanguageList icons={[Go]} />
  <p>
    Small Project which creates a network replicated game example. Basically a
    multiplayer game :)<br />
    Each 2 conseguential client opened, are coupled and put in the same server session.
    Many sessions are available
  </p>
  <div class="example">
    <img src="img/airhockey.gif" alt="background image" />
    <img
      id="airhockey-server"
      src="img/airhockey-server.png"
      alt="background image"
    />
  </div>
</div>

<style type="scss">
  div.root {
    margin: 1rem 10rem 1rem 10rem;
    @media only screen and (max-width: 768px) {
      margin: 1rem;
    }
    h2 {
      margin-bottom: 0;
    }
    div.example {
      display: flex;
      justify-content: space-around;
      align-items: center;
      @media only screen and (max-width: 768px) {
        flex-direction: column;
      }
      img {
        width: 40%;
        @media only screen and (max-width: 768px) {
          width: 90%;
          margin-bottom: 1rem;
        }
      }
    }
  }
</style>
