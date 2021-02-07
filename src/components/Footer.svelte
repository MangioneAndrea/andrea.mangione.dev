<script>
  import FaLinkedinIn from "svelte-icons/fa/FaLinkedinIn.svelte";
  import FaGithub from "svelte-icons/fa/FaGithub.svelte";
  import { github, linkedin } from "../utils/External";
  const year = new Date().getFullYear();
</script>

<div id="footer">
  <h2 class="name">Andrea Mangione</h2>
  <br />
  <div class="social">
    <a href={linkedin} target="_blank"><FaGithub /></a>
    <a href={github} target="_blank"><FaLinkedinIn /></a>
  </div>
  <p class="copyright">Andrea.mangione.dev © {year}</p>
</div>

<style type="text/scss">
  $bk-color: #111;

  div#footer {
    padding: 2rem;
    background-color: $bk-color;
    color: white;
    h2.name {
      font-family: "Dancing Script", cursive;
      align-self: center;
      font-size: 1.4em;
      text-align: center;
    }
    .social {
      text-align: center;
      padding-bottom: 25px;
      a {
        font-size: 24px;
        width: 40px;
        height: 40px;
        line-height: 40px;
        display: inline-block;
        text-align: center;
        margin: 0 8px;
        color: inherit;
        opacity: 0.75;
        &:hover {
          opacity: 0.9;
        }
      }
    }
    .copyright {
      text-align: center;
      font-size: 13px;
      color: #aaa;
      margin-bottom: 0;
    }
  }
</style>
