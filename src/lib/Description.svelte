<script>
  import Compy from "../elements/compy/Compy.svelte";
</script>

<section>
  <div class="titleContainer">
    <h1>About me</h1>
    <div class="compySpace">
      <Compy />
    </div>
  </div>
  <p>
    I'm a Switzerland-based developer. Currently working at <b>Jacando AG</b>
    but always open for any kind of project.
    <br />
    I love wandering all over the world and I'm also trying to see as many places
    as possible. I speak 4 languages:
    <img src={"assets/Flags/it.svg"} alt="italian flag" />, <img src={"assets/Flags/gb.svg"}  alt="uk flag"/>
    and <img src={"assets/Flags/de.svg"}  alt="german flag"/> /
    <img src={"assets/Flags/ch.svg"}  alt="swiss flag"/>
    <br /><br />
    ... Did I already say I love creating animations and interactivity?
  </p>
</section>

<style>
  img {
    height: 1.5rem;
  }

  h1 {
    background: var(--greenish-background);
    -webkit-background-clip: text;
    background-clip: text;
  }
  .titleContainer {
    position: relative;
    display: flex;
  }

  .compySpace {
    flex: 1;
    position: relative;
  }

  @media only screen and (max-width: 1366px) {
    .compySpace {
      display: none;
    }
  }
</style>
