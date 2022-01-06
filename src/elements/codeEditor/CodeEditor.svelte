<script>
  export let code = "";
  export let language = "javascript";
  import Prism from "prismjs";
  import "prismjs/components/prism-kotlin";

  // creates the prism classes
  $: prismClasses = `language-${language}`;

  $: formatted = Prism.highlight(code, Prism.languages[language], language);
</script>

<pre class={prismClasses}>
  <code>
    {@html formatted}
  </code>
</pre>

<style>
  :global(span.token.operator) {
    background: none;
  }
</style>
