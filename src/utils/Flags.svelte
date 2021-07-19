<script>
    export let name;

    import IT from "../images/Flags/it.svg";
    import DE from "../images/Flags/de.svg";
    import UK from "../images/Flags/gb.svg";
    import ES from "../images/Flags/es.svg";
    import CH from "../images/Flags/ch.svg";

    const flags = { IT, DE, UK, ES, CH };
</script>

<svelte:component this={flags[name]} />
