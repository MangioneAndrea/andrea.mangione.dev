<script>
  import SmallDescription from "./InfiniteParts/SmallDescription.svelte";
  import Stacks from "./InfiniteParts/Stacks.svelte";
</script>

<div id="whoAmI"><SmallDescription /><Stacks /></div>

<style type="text/scss">
  div#whoAmI {
    min-height: calc(100% - 4rem);
    height: 300%;
    background-color: #772f1a;
    padding-top: 4rem;
  }
</style>
