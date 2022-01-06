<script>
  import Projects from "./InfiniteParts/Projects.svelte";
  import Stacks from "./InfiniteParts/Stacks.svelte";
</script>

<div>
  <Projects />
  <Stacks />
</div>

<style type="text/scss">
  div {
    min-height: calc(100% - 4rem);
    background-color: #772f1a;
    padding-top: 4rem;
    padding-bottom: 4rem;
  }
</style>
