<script>
  import { addObservable, useObservable } from "../utils/ScrollController";
  import { TopBarShown } from "../utils/Stores";
  const observable = useObservable("page1", {
    onIntersectionStart: () => TopBarShown.set(true),
    onIntersectionEnd: () => TopBarShown.set(false),
  });
</script>

<style>
  div {
    height: calc(100% - 4rem);
    background-color: #e5e5e5;
    padding-top: 4rem;
  }
</style>

<div id="page1" use:observable>hello</div>
