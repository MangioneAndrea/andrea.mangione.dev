<script>
  export let icons = [];
</script>

<div>
  {#each icons as icon}
    <img
      class="p-2 h-5"
      height="19"
      src={icon}
      alt={`language ${icon.split("/").at(-1)}`}
    />
  {/each}
</div>
