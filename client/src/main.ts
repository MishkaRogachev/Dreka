import "./styles.css";

import { EventsService } from "$services/events";

import App from "./App.svelte";

EventsService.init();

const app = new App({
  target: document.body
})

export default app;
