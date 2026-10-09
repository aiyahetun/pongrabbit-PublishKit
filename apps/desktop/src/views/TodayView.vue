<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { PublishTask } from "@publishkit/shared";
import MediaLinkDialog from "../components/MediaLinkDialog.vue";
import TaskCard from "../components/TaskCard.vue";
import TaskActionBar from "../components/TaskActionBar.vue";
import { copyFirstLinkedImage, openLinkedImagesFolder } from "../utils/taskMediaActions";
import { buildTodayQueueSections, taskEventDate, type TaskSection } from "../utils/taskGroups";
import { confirmPublishIfDuplicate } from "../utils/confirmPublish";
import { confirmArchiveTask } from "../utils/confirmArchive";

const { t } = useI18n();
const tasks = ref<PublishTask[]>([]);
const loading = ref(false);
const error = ref("");
const copiedId = ref("");
const publishUrls = ref<Record<string, string>>({});
const scheduledDates = ref<Record<string, string>>({});
const publishedDates = ref<Record<string, string>>({});
const taskNotes = ref<Record<string, string>>({});
const blockedReasonInputs = ref<Record<string, string>>({});
const collapsed = ref<Set<string>>(new Set());
const notice = ref("");
const mediaTarget = ref<PublishTask | null>(null);

const sections = computed(() => buildTodayQueueSections(tasks.value));
const hasTasks = computed(() => sections.value.some((section) => section.tasks.length > 0));

function sectionLabel(section: TaskSection) {
  return t(section.labelKey, section.labelParams ?? {});
}

function isCollapsed(key: string) {
  return collapsed.value.has(key);
}

function toggleSection(key: string) {
  const next = new Set(collapsed.value);
  if (next.has(key)) next.delete(key);
  else next.add(key);
  collapsed.value = next;
}

async function loadTasks() {
  loading.value = true;
  error.value = "";
  try {
    const list = await invoke<PublishTask[]>("list_today_tasks_cmd");
    tasks.value = list;
    const next: Record<string, string> = {};
    const nextDates: Record<string, string> = {};
    const nextPublished: Record<string, string> = {};
    const nextNotes: Record<string, string> = {};
    const nextBlocked: Record<string, string> = {};
    for (const task of list) {
      next[task.id] = task.publishUrl || "";
      nextDates[task.id] = task.scheduledAt?.slice(0, 10) ?? taskEventDate(task);
      nextPublished[task.id] = taskEventDate(task);
      nextNotes[task.id] = task.note || "";
      nextBlocked[task.id] = task.blockedReason || "";
    }
    publishUrls.value = next;
    scheduledDates.value = nextDates;
    publishedDates.value = nextPublished;
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
      publishedDate: null,
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
      publishedDate: null,
    });
    await loadTasks();
  } catch (e) {
    error.value = String(e);
  }
}

async function backfillPublish(task: PublishTask) {
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
      publishedDate: publishedDates.value[task.id] || taskEventDate(task),
    });
    await loadTasks();
  } catch (e) {
    error.value = String(e);
  }
}

async function archiveTask(task: PublishTask) {
  error.value = "";
  notice.value = "";
  const ok = await confirmArchiveTask(t);
  if (!ok) return;
  try {
    await invoke("update_publish_task_status_cmd", {
      taskId: task.id,
      status: "archived",
      publishUrl: null,
      note: null,
      blockedReason: null,
      publishedDate: null,
    });
    notice.value = t("tasks.archiveDone");
    await loadTasks();
  } catch (e) {
    error.value = String(e);
  }
}

async function saveScheduled(task: PublishTask, isReschedule = false) {
  error.value = "";
  notice.value = "";
  try {
    await invoke("update_publish_task_scheduled_cmd", {
      taskId: task.id,
      scheduledDate: scheduledDates.value[task.id] ?? "",
    });
    notice.value = t(isReschedule ? "tasks.rescheduleDone" : "tasks.scheduledSaved");
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
    <p v-else-if="!hasTasks" class="muted">{{ t("today.empty") }}</p>

    <div v-else class="sections">
      <section v-for="group in sections" :key="group.key" class="task-section">
        <button type="button" class="section-head" @click="toggleSection(group.key)">
          <span class="chevron" :class="{ collapsed: isCollapsed(group.key) }">›</span>
          <span :class="{ overdue: group.key === 'ready_overdue' }">{{ sectionLabel(group) }}</span>
          <span class="count">{{ group.tasks.length }}</span>
        </button>

        <ul v-if="!isCollapsed(group.key)" class="list">
          <li v-for="task in group.tasks" :key="task.id">
            <TaskCard :task="task" :show-overdue="group.key === 'ready_overdue'">
              <TaskActionBar
                :task="task"
                :overdue="group.key === 'ready_overdue'"
                :copied="copiedId === task.id"
                :publish-url="publishUrls[task.id] ?? ''"
                :scheduled-date="scheduledDates[task.id] ?? ''"
                :published-date="publishedDates[task.id] ?? ''"
                :task-note="taskNotes[task.id] ?? ''"
                :blocked-reason-input="blockedReasonInputs[task.id] ?? ''"
                @update:publish-url="publishUrls[task.id] = $event"
                @update:scheduled-date="scheduledDates[task.id] = $event"
                @update:published-date="publishedDates[task.id] = $event"
                @update:task-note="taskNotes[task.id] = $event"
                @update:blocked-reason-input="blockedReasonInputs[task.id] = $event"
                @copy="copyTask(task)"
                @copy-single-image="copyTaskSingleImage(task)"
                @link-media="mediaTarget = task"
                @open-linked-images-folder="openTaskImagesFolder(task)"
                @export-pack="exportTaskPack(task)"
                @mark-published="markPublished(task)"
                @backfill-publish="backfillPublish(task)"
                @archive="archiveTask(task)"
                @mark-blocked="markBlocked(task)"
                @save-note="saveTaskNote(task)"
                @save-scheduled="saveScheduled(task, group.key === 'ready_overdue')"
              />
            </TaskCard>
          </li>
        </ul>
      </section>
    </div>

    <MediaLinkDialog
      v-if="mediaTarget"
      :content-id="mediaTarget.content.id"
      :content-title="mediaTarget.content.title"
      @close="mediaTarget = null"
    />
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
.sections {
  display: flex;
  flex-direction: column;
  gap: var(--pk-space-4);
}
.task-section {
  display: flex;
  flex-direction: column;
  gap: var(--pk-space-3);
}
.section-head {
  display: flex;
  align-items: center;
  gap: var(--pk-space-2);
  padding: 0;
  border: none;
  background: none;
  font: inherit;
  font-size: 14px;
  font-weight: 600;
  color: var(--pk-ink);
  cursor: pointer;
}
.section-head .overdue {
  color: var(--pk-status-blocked);
}
.chevron {
  display: inline-block;
  transition: transform 120ms ease;
}
.chevron.collapsed {
  transform: rotate(-90deg);
}
.count {
  margin-left: auto;
  color: var(--pk-ink-muted);
  font-weight: 500;
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
