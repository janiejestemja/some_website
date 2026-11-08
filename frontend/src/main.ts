import { createApp } from "vue"
import { createPinia } from "pinia"

import { useUserSession } from "@/stores/session.ts"


import App from "./App.vue"
import router from "./router"
import "./assets/main.css"

async function bootstrap() {
  const app = createApp(App);
  const pinia = createPinia();

  app.use(pinia);
  app.use(router);

  // Hydrate
  const session = useUserSession(pinia)
  try {
    const request = await fetch("logged_in", { method: "POST" });
    const response = await request.json();
    if (response.success === true) {
      session.log_in();
    }
  } catch {
    session.log_out();
  }

  app.mount("#app");
}

bootstrap();
