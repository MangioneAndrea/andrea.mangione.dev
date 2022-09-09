import './app.css'
import App from './App.svelte'

import "prismjs/themes/prism.css";
import "prismjs/themes/prism-okaidia.min.css";

const app = new App({
  target: document.getElementById('app')
})

export default app
