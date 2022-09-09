<script lang="ts">
</script>

<div>
    <h2>andrea.mangione.dev</h2>
    <span>
        <a href="https://github.com/MangioneAndrea">
            <img src={"assets/Logos/Github.png"}/>
        </a>
        <a href="https://www.linkedin.com/in/andrea-mangione-592902156/">
            <img src={"assets/Logos/Linkedin.png"}/>
        </a>
        <a href="mailto:andrea@mangione.dev">
            <img src={"assets/Logos/Email.png"}/>
        </a>
    </span>
</div>

<style>
    h2{
        font-family: var(--terminal-font);
        padding-left: 2rem;
    }

    div {
        padding-top: 1rem;
        position: sticky;
        justify-content: space-between;
        width: 100%;
        display: flex;
        align-items: center;
        max-width: var(--max-width);
        margin: auto;
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