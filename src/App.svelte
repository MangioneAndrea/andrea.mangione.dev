<script>
  import "./helpers/CommonFunctions"
  import TopBar from "./components/TopBar.svelte";
  import FirstPage from "./components/FirstPage.svelte";
  import WhoAmI from "./components/WhoAmI.svelte";
  import MyWorks from "./components/MyWorks.svelte";
import { } from "os";
</script>

<style type="scss">
  :global(body) {
    padding: 0;
    font-family: "Rubik", sans-serif;
  }
  :global(html) {
    overflow-x: hidden;
    box-sizing: border-box;
    text-rendering: optimizeLegibility;
  }
</style>

<svelte:head>
  <title>Andrea's Portfolio</title>
  
</svelte:head>

<TopBar />
<FirstPage />

<WhoAmI />
<MyWorks />
<main />
<footer />

<link
  href="https://fonts.googleapis.com/css2?family=Rubik:wght@300&display=swap"
  rel="stylesheet" />
<link
  href="https://fonts.googleapis.com/css2?family=Dancing+Script&display=swap"
  rel="stylesheet" />
<link
  href="https://fonts.googleapis.com/css2?family=Alata&display=swap"
  rel="stylesheet" />
