import { invoke } from "@tauri-apps/api/core";
import { ask } from "@tauri-apps/plugin-dialog";
import type { DuplicatePublishWarning, PublishTask } from "@publishkit/shared";
import type { ComposerTranslation } from "vue-i18n";

export async function confirmPublishIfDuplicate(
  task: PublishTask,
  t: ComposerTranslation
): Promise<boolean> {
  const warning = await invoke<DuplicatePublishWarning | null>("check_duplicate_publish_cmd", {
    taskId: task.id,
    contentItemId: task.content.id,
    channelId: task.channel.id,
  });
  if (!warning) return true;

  const when = warning.previousPublishedAt.slice(0, 10);
  const detail = warning.previousPublishUrl
    ? t("tasks.duplicatePublishWithUrl", { url: warning.previousPublishUrl })
    : t("tasks.duplicatePublishNoUrl");
  return ask(
    t("tasks.duplicatePublishConfirm", {
      channel: task.channel.name,
      title: task.content.title,
      days: warning.daysSince,
      when,
      detail,
    }),
    {
      title: t("tasks.duplicatePublishTitle"),
      kind: "warning",
    }
  );
}
