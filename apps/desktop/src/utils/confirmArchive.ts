import { ask } from "@tauri-apps/plugin-dialog";
import type { ComposerTranslation } from "vue-i18n";

export async function confirmArchiveTask(t: ComposerTranslation): Promise<boolean> {
  return ask(t("tasks.archiveConfirm"), {
    title: t("tasks.archiveTitle"),
    kind: "warning",
  });
}
