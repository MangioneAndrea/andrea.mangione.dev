<script context="module">
  import { TopBarShown } from "../utils/Stores";
  import { scrollTo } from "../utils/ScrollController";
</script>

<style type="text/scss">
  nav {
    background-color: #14213d;
    position: fixed;
    display: block;
    width: 100%;
    transition: 0.5s;
    z-index: 999;
    &.shown {
      transform: translateY(0%);
    }
    &.hidden {
      transform: translateY(-100%);
    }
  }
  div {
    margin: auto;
    width: 100%;
    max-width: 1400px;
    color: white;
    height: 4rem;
    display: flex;
    -webkit-box-pack: justify;
    justify-content: space-between;
    -webkit-box-align: center;
    align-items: center;
    .name {
      font-family: "Dancing Script", cursive;
      align-self: center;
      font-size: 1.4em;
    }
    .goDownButton {
      margin: 2rem;
      transition: 0.5s;
      height: 1rem;
      align-self: center;
    }
    .goDownButton:hover {
      padding-top: 0.5rem;
    }
  }
</style>

<nav class={$TopBarShown ? 'shown' : 'hidden'}>
  <div id="navbar">
    <span class="goDownButton" on:click={() => scrollTo('whoAmI')}>Who am I</span>
    <h2 class="name">Andrea Mangione</h2>
    <span class="goDownButton" on:click={() => scrollTo('myWorks')}>My Works</span>
  </div>
</nav>
