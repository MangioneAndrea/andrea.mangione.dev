<script>
  import Tree1 from "./Tree1.svelte";
  import Tree2 from "./Tree2.svelte";
  import Tree3 from "./Tree3.svelte";
  import Tree4 from "./Tree4.svelte";
  import Tree5 from "./Tree5.svelte";
  import Tree6 from "./Tree6.svelte";
  import Tree7 from "./Tree7.svelte";
  import Tree8 from "./Tree8.svelte";
  const trees = [Tree1, Tree2, Tree3, Tree4, Tree5, Tree6, Tree7, Tree8];

  export let tree = Math.floor(Math.random() * 8);
  export let left = 0;
  export let bottom = "100px";
  export let width = 150;
  export let height = 800;

  const woodShadow = "#813b1a";
  const wood = "#b35825";
  const woodLight = "#e27839";
  const leaves = "#67b730";
  const leavesShadow = "#4c9534";
  const leavesLight = "#8ad0FF34";
  console.log(tree);
</script>

<svg
  style={`
        position: absolute;
        left: ${left};
        bottom: ${bottom};
        `}
  xml:space="preserve"
  {height}
  viewBox="1000 500 1029.3333 1800"
  ><defs id="defs6" /><sodipodi:namedview
    pagecolor="#ffffff"
    bordercolor="#666666"
    borderopacity="1"
    objecttolerance="10"
    gridtolerance="10"
    guidetolerance="10"
    id="namedview4"
  /><g id="g10" transform="matrix(1.3333333,0,0,-1.3333333,0,2496)"
    ><g id="g12" transform="scale(0.1)">
      <svelte:component this={trees[tree]} />
    </g></g
  ></svg
>

<style type="scss">
  :global(path) {
    fill-opacity: 1;
    fill-rule: nonzero;
    stroke: none;
  }
</style>
