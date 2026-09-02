<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { PublishTask } from "@publishkit/shared";
import TaskCard from "../components/TaskCard.vue";
import TaskActionBar from "../components/TaskActionBar.vue";
import { copyFirstLinkedImage, openLinkedImagesFolder } from "../utils/taskMediaActions";
import { confirmPublishIfDuplicate } from "../utils/confirmPublish";

const { t } = useI18n();
const tasks = ref<PublishTask[]>([]);
const loading = ref(false);
const error = ref("");
const copiedId = ref("");
const publishUrls = ref<Record<string, string>>({});
const scheduledDates = ref<Record<string, string>>({});
const taskNotes = ref<Record<string, string>>({});
const blockedReasonInputs = ref<Record<string, string>>({});
const notice = ref("");

async function loadTasks() {
  loading.value = true;
  error.value = "";
  try {
    const list = await invoke<PublishTask[]>("list_today_tasks_cmd");
    tasks.value = list;
    const next: Record<string, string> = {};
    const nextDates: Record<string, string> = {};
    const nextNotes: Record<string, string> = {};
    const nextBlocked: Record<string, string> = {};
    for (const task of list) {
      next[task.id] = task.publishUrl || "";
      nextDates[task.id] = task.scheduledAt?.slice(0, 10) ?? "";
      nextNotes[task.id] = task.note || "";
      nextBlocked[task.id] = task.blockedReason || "";
    }
    publishUrls.value = next;
    scheduledDates.value = nextDates;
    taskNotes.value = nextNotes;
    blockedReasonInputs.value = nextBlocked;
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

async function copyTask(task: PublishTask) {
  await invoke("copy_task_body_cmd", { taskId: task.id });
  copiedId.value = task.id;
  setTimeout(() => {
    if (copiedId.value === task.id) copiedId.value = "";
  }, 1500);
}

async function exportTaskPack(task: PublishTask) {
  error.value = "";
  notice.value = "";
  const picked = await open({
    directory: true,
    multiple: false,
    title: t("tasks.exportChannelPackTitle"),
  });
  if (!picked || typeof picked !== "string") return;
  try {
    const result = await invoke<{
      folderPath: string;
      mediaCount: number;
      fileCount: number;
    }>("export_task_pack_cmd", {
      taskId: task.id,
      destFolder: picked,
    });
    notice.value = t("tasks.exportChannelPackDone", {
      path: result.folderPath,
      files: result.fileCount,
      count: result.mediaCount,
    });
  } catch (e) {
    error.value = String(e);
  }
}

async function copyTaskSingleImage(task: PublishTask) {
  error.value = "";
  notice.value = "";
  try {
    const result = await copyFirstLinkedImage(task.content.id);
    if (result.mode === "none") {
      notice.value = t("media.suggestEmpty");
      return;
    }
    notice.value = t("media.copied");
  } catch (e) {
    error.value = String(e);
  }
}

async function openTaskImagesFolder(task: PublishTask) {
  error.value = "";
  notice.value = "";
  try {
    const result = await openLinkedImagesFolder(task.content.id);
    if (result.mode === "none") {
      notice.value = t("media.suggestEmpty");
      return;
    }
    notice.value = t("media.stagedMultiple", { count: result.count });
  } catch (e) {
    error.value = String(e);
  }
}

async function markPublished(task: PublishTask) {
  error.value = "";
  notice.value = "";
  const ok = await confirmPublishIfDuplicate(task, t);
  if (!ok) return;
  try {
    await invoke("update_publish_task_status_cmd", {
      taskId: task.id,
      status: "published",
      publishUrl: publishUrls.value[task.id]?.trim() || null,
      note: null,
      blockedReason: null,
    });
    await loadTasks();
  } catch (e) {
    error.value = String(e);
  }
}

async function saveTaskNote(task: PublishTask) {
  error.value = "";
  notice.value = "";
  try {
    await invoke("update_task_note_cmd", {
      taskId: task.id,
      note: taskNotes.value[task.id] ?? "",
    });
    notice.value = t("tasks.noteSaved");
    await loadTasks();
  } catch (e) {
    error.value = String(e);
  }
}

async function markBlocked(task: PublishTask) {
  const reason = blockedReasonInputs.value[task.id]?.trim();
  if (!reason) {
    error.value = t("tasks.blockedReasonRequired");
    return;
  }
  error.value = "";
  notice.value = "";
  try {
    await invoke("update_publish_task_status_cmd", {
      taskId: task.id,
      status: "blocked",
      publishUrl: null,
      note: null,
      blockedReason: reason,
    });
    await loadTasks();
  } catch (e) {
    error.value = String(e);
  }
}

async function saveScheduled(task: PublishTask) {
  error.value = "";
  notice.value = "";
  try {
    await invoke("update_publish_task_scheduled_cmd", {
      taskId: task.id,
      scheduledDate: scheduledDates.value[task.id] ?? "",
    });
    notice.value = t("tasks.scheduledSaved");
    await loadTasks();
  } catch (e) {
    error.value = String(e);
  }
}

onMounted(loadTasks);
</script>

<template>
  <section class="page">
    <header class="head">
      <div>
        <h1>{{ t("today.title") }}</h1>
        <p class="muted">{{ t("today.subtitle") }}</p>
      </div>
      <button type="button" class="pk-btn pk-btn--secondary" :disabled="loading" @click="loadTasks">
        {{ t("today.refresh") }}
      </button>
    </header>

    <p v-if="notice" class="notice">{{ notice }}</p>
    <p v-if="loading" class="muted">{{ t("today.loading") }}</p>
    <p v-else-if="error" class="error">{{ error }}</p>
    <p v-else-if="!tasks.length" class="muted">{{ t("today.empty") }}</p>

    <ul v-else class="list">
      <li v-for="task in tasks" :key="task.id">
        <TaskCard :task="task">
          <TaskActionBar
            :task="task"
            :copied="copiedId === task.id"
            :publish-url="publishUrls[task.id] ?? ''"
            :scheduled-date="scheduledDates[task.id] ?? ''"
            :task-note="taskNotes[task.id] ?? ''"
            :blocked-reason-input="blockedReasonInputs[task.id] ?? ''"
            @update:publish-url="publishUrls[task.id] = $event"
            @update:scheduled-date="scheduledDates[task.id] = $event"
            @update:task-note="taskNotes[task.id] = $event"
            @update:blocked-reason-input="blockedReasonInputs[task.id] = $event"
            @copy="copyTask(task)"
            @copy-single-image="copyTaskSingleImage(task)"
            @open-linked-images-folder="openTaskImagesFolder(task)"
            @export-pack="exportTaskPack(task)"
            @mark-published="markPublished(task)"
            @mark-blocked="markBlocked(task)"
            @save-note="saveTaskNote(task)"
            @save-scheduled="saveScheduled(task)"
          />
        </TaskCard>
      </li>
    </ul>
  </section>
</template>

<style scoped>
.page {
  padding: var(--pk-space-5);
  display: flex;
  flex-direction: column;
  gap: var(--pk-space-4);
}
.head {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: var(--pk-space-3);
}
.head h1 {
  margin: 0 0 4px;
  font-size: 20px;
  font-weight: 600;
}
.list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: var(--pk-space-3);
}
.muted {
  color: var(--pk-ink-muted);
  font-size: 14px;
}
.error {
  color: var(--pk-status-blocked);
  font-size: 14px;
}
.notice {
  color: var(--pk-accent);
  font-size: 13px;
}
</style>
