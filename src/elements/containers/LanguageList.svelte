<script>
  export let icons = [];
  export let height = 30;
</script>

<div>
  {#each icons as icon}
    <svelte:component this={icon} {height} />
  {/each}
</div>

