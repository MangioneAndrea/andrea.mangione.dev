<script>
</script>

<div class="root">
    <a href="https://github.com/MangioneAndrea/airhockey" target="_blank">
        <h2>Airhockey</h2>
    </a>
    <p>
        Small Project which creates a network replicated game example. Basically a
        multiplayer game :)<br/>
        Each 2 conseguential client opened, are coupled and put in the same server session.
        Many sessions are available
    </p>
    <div class="example">
        <img src="assets/Images/airhockey.gif" alt="airhockey client"/>
        <img
                id="airhockey-server"
                src="assets/Images/airhockey-server.png"
                alt="airhockey server"
        />
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

    a {
        font-size: 2em;
        color: white;
        text-decoration: underline;
    }

    div.example {
        display: flex;
        justify-content: space-around;
        align-items: center;
    }

    #airhockey-server{
        width: 70%;
    }

    @media only screen and (max-width: 768px) {
        div.example {
            flex-direction: column;
        }

        img {
            width: 40%;
        }

        @media only screen and (max-width: 768px) {
            img {
                width: 90%;
                margin-bottom: 1rem;

            }
        }
    }
</style>
