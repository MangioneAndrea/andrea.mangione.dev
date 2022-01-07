<script>
  import LanguageList from "../../../../elements/containers/LanguageList.svelte";
  import Go from "devicon/icons/go/go-original-wordmark.svg";
</script>

<div class="root">
  <a href="https://github.com/MangioneAndrea/airhockey" target="_blank">
    AirHockey
  </a>
  <LanguageList icons={[Go]} />
  <p>
    Small Project which creates a network replicated game example. Basically a
    multiplayer game :)<br />
    Each 2 conseguential client opened, are coupled and put in the same server session.
    Many sessions are available
  </p>
  <div class="example">
    <img src="img/airhockey.gif" alt="airhockey client" />
    <img
      id="airhockey-server"
      src="img/airhockey-server.png"
      alt="airhockey server"
    />
  </div>
</div>

<style type="scss">
  div.root {
    margin: 1rem 10rem 1rem 10rem;
    @media only screen and (max-width: 768px) {
      margin: 1rem;
    }
    a {
      font-size: 2em;
      color: white;
      text-decoration: underline;
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
