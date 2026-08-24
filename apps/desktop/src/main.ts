import { createApp } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { UiLocale } from "@publishkit/shared";
import App from "./App.vue";
import { createAppI18n } from "./i18n";
import { router } from "./router";
import "./styles/tokens.css";

async function bootstrap() {
  const splash = document.getElementById("splash");
  let locale: UiLocale = "en";

  try {
    locale = await invoke<UiLocale>("get_ui_locale");
  } catch {
    locale = navigator.language.toLowerCase().startsWith("zh") ? "zh-CN" : "en";
  }

  const app = createApp(App);
  app.use(createAppI18n(locale));
  app.use(router);
  app.mount("#app");
  splash?.remove();
}

bootstrap();
