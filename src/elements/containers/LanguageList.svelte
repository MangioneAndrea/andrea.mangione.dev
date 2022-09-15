<script>
    export let icons = [];
</script>

<div>
    {#each icons as icon}
        <img height="19" src={icon} alt={`language ${icon.split("/").at(-1)}`}/>
    {/each}
</div>

<style>
    img {
        padding: 0 .5rem;
    }
</style>