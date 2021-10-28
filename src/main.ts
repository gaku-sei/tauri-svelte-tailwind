import "./styles/global.css";

import App from "./App.svelte";

const app = new App({
  target: document.body,
  props: { title: "Tauri Svelte Tailwind" },
});

export default app;
