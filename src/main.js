import App from "./App.svelte";
import "prismjs/plugins/line-numbers/prism-line-numbers.css";
import "prismjs/plugins/command-line/prism-command-line.css";
import "prismjs/plugins/line-highlight/prism-line-highlight.css";

import "prismjs/themes/prism.css";
import "prismjs/themes/prism-twilight.css";
var app = new App({
  target: document.body,
});

export default app;
