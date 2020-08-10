<script context="module">
  import { TopBarShown } from "../utils/Stores";
</script>

<style>
  nav {
    background-color: #14213d;
    position: fixed;
    display: block;
    width: 100%;
    transition: 0.5s;
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
  }
  .name {
    align-self: center;
  }
  .goDownButton {
    margin: 1.5rem;
    transition: 0.5s;
    height: 100%;
    align-self: center;
    padding-top: 2.5rem;
  }
  .goDownButton:hover {
    padding-top: 3.5rem;
  }
</style>

<nav style="transform: translateY({$TopBarShown? "0%" : "-100%"})">
  <div id="navbar">
    <span class="goDownButton">Who am I</span>
    <h2 class="name">Andrea Mangione</h2>
    <span class="goDownButton">My Works</span>
  </div>
</nav>
