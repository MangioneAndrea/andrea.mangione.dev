<script>
    export let icons = [];
</script>

<div>
    {#each icons as icon}
        <img src={icon} alt={`language ${icon.split("/").at(-1)}`}/>
    {/each}
</div>

<style>
    img {
        height: 1rem;
        padding: 0 .5rem;
    }
</style>