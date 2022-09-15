<script>
  import LanguageList from "../../elements/containers/LanguageList.svelte";
</script>

<div class="root">
  <a
    href="https://github.com/MangioneAndrea/airhockey"
    target="_blank"
    class="projectTitle"
  >
    <h2>Airhockey</h2>
  </a>
  <LanguageList icons={["assets/Logos/Go.svg", "assets/Logos/Grpc.svg"]} />
  <p>
    Small Project which creates a network replicated game example. Basically a
    multiplayer game :)<br />
    Each 2 conseguential client opened, are coupled and put in the same server session.
    Many sessions are available
  </p>
  <div class="example">
    <video autoplay loop muted>
      <source src="assets/Images/airhockey.webm" type="video/webm">
        Your browser does not support the video tag.
    </video>
  </div>
</div>

<style>
  div.root {
    margin: 1rem 3rem 1rem 3rem;
  }

  @media only screen and (max-width: 768px) {
    div.root {
      margin: 1rem;
    }
  }


  div.example {
    display: flex;
    justify-content: space-evenly;
    align-items: center;
  }

  video {
    width: 50%;
  }

  @media only screen and (max-width: 1366px) {
    div.example {
      flex-direction: column;
    }

    @media only screen and (max-width: 768px) {
      video {
        width: 90%;
        margin-bottom: 1rem;
      }
    }
  }
</style>
