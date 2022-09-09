<script>
    import KotlinFirmata from "./kotlin-firmata/KotlinFirmata.svelte";
    import AirHockey from "./airhockey/AirHockey.svelte";
</script>

<section>
    <h1>Projects</h1>
    <KotlinFirmata/>
    <AirHockey/>
</section>

<style>
    section {
        margin-top: 0;
    }

    img {
        height: 1.5rem;
    }

    h1 {
        background: var(--reddish-background);
        -webkit-background-clip: text;
    }
</style>