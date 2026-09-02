import { createApp } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { UiLocale } from "@publishkit/shared";
import App from "./App.vue";
import { createAppI18n } from "./i18n";
import { router } from "./router";
import "./styles/tokens.css";

async function bootstrap() {
  const splash = document.getElementById("splash");
  let locale: UiLocale = "en";

  try {
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
  } finally {
    try {
      const win = getCurrentWindow();
      await win.show();
      await win.setFocus();
    } catch {
      // vite dev in browser
    }
  }
}

bootstrap();
