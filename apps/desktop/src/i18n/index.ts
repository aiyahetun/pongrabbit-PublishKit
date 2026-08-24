import { createI18n } from "vue-i18n";
import type { UiLocale } from "@publishkit/shared";
import zhCN from "@publishkit/i18n/locales/zh-CN.json";
import en from "@publishkit/i18n/locales/en.json";

export function createAppI18n(locale: UiLocale) {
  return createI18n({
    legacy: false,
    locale,
    fallbackLocale: "en",
    messages: {
      "zh-CN": zhCN,
      en,
    },
  });
}
