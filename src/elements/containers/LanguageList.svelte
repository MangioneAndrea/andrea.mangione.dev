<script>
    export let icons = [];
</script>

<div>
    {#each icons as icon}
        <img src={icon}/>
    {/each}
</div>

<style>
    img {
        height: 1rem;
        padding: 0 .5rem;
    }
</style>