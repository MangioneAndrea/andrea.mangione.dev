<script>
</script>

<div>
  <h2>andrea.mangione.dev</h2>
  <span>
    <a href="https://github.com/MangioneAndrea" target="_blank">
      <img src={"assets/Logos/Github.png"} />
    </a>
    <a
      href="https://www.linkedin.com/in/andrea-mangione-592902156/"
      target="_blank"
    >
      <img src={"assets/Logos/Linkedin.png"} />
    </a>
    <a href="mailto:andrea@mangione.dev" target="_blank">
      <img src={"assets/Logos/Email.png"} />
    </a>
  </span>
</div>

<style>
  div {
    z-index: 10;
    top: 0;
    padding-top: 1rem;
    position: fixed;
    justify-content: space-between;
    width: 100%;
    display: flex;
    align-items: center;
    max-width: var(--max-width);
    margin: auto;
    left: 50%;
    transform: translate(-50%);
    background: var(--background-color);
  }

  div > h2 {
    font-family: var(--terminal-font);
    padding-left: 2rem;
  }
  @media only screen and (max-width: 1366px) {
    div > h2 {
      display: none;
    }
    span {
      margin: auto;
    }
  }

  span {
    display: flex;
    justify-content: space-evenly;
    align-items: center;
  }

  span > a {
    padding: 0 20px 0 5px;
  }

  img {
    width: 2rem;
  }
</style>
