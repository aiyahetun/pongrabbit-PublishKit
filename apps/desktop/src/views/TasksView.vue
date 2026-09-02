<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import type { PublishTask, TaskStatus } from "@publishkit/shared";
import TaskCard from "../components/TaskCard.vue";
import TaskActionBar from "../components/TaskActionBar.vue";
import TasksKanbanBoard from "../components/TasksKanbanBoard.vue";
import { copyFirstLinkedImage, openLinkedImagesFolder } from "../utils/taskMediaActions";
import { buildTaskSections, type TaskSection } from "../utils/taskGroups";
import { confirmPublishIfDuplicate } from "../utils/confirmPublish";

const { t, locale } = useI18n();
const tasks = ref<PublishTask[]>([]);
const filter = ref<"all" | TaskStatus>("all");
const viewMode = ref<"list" | "kanban">("list");
const loading = ref(false);
const error = ref("");
const copiedId = ref("");
const publishUrls = ref<Record<string, string>>({});
const scheduledDates = ref<Record<string, string>>({});
const taskNotes = ref<Record<string, string>>({});
const blockedReasonInputs = ref<Record<string, string>>({});
const collapsed = ref<Set<string>>(new Set());
const notice = ref("");

const filters: Array<"all" | TaskStatus> = ["all", "draft", "ready", "blocked", "published"];

const sections = computed(() => buildTaskSections(tasks.value, filter.value));

const kanbanTasks = computed(() => {
  if (filter.value === "all") return tasks.value;
  return tasks.value.filter((task) => task.status === filter.value);
});

const hasTasks = computed(() =>
  viewMode.value === "kanban" ? kanbanTasks.value.length > 0 : sections.value.some((s) => s.tasks.length > 0)
);

function syncTaskFields(list: PublishTask[]) {
  const next: Record<string, string> = { ...publishUrls.value };
  const nextDates: Record<string, string> = { ...scheduledDates.value };
  const nextNotes: Record<string, string> = { ...taskNotes.value };
  const nextBlocked: Record<string, string> = { ...blockedReasonInputs.value };
  for (const task of list) {
    if (!(task.id in next)) next[task.id] = task.publishUrl || "";
    else if (task.publishUrl) next[task.id] = task.publishUrl;
    if (!(task.id in nextDates)) {
      nextDates[task.id] = task.scheduledAt?.slice(0, 10) ?? "";
    } else if (task.scheduledAt) {
      nextDates[task.id] = task.scheduledAt.slice(0, 10);
    }
    if (!(task.id in nextNotes)) nextNotes[task.id] = task.note || "";
    if (!(task.id in nextBlocked)) {
      nextBlocked[task.id] = task.blockedReason || "";
    } else if (task.status === "blocked" && task.blockedReason) {
      nextBlocked[task.id] = task.blockedReason;
    }
  }
  publishUrls.value = next;
  scheduledDates.value = nextDates;
  taskNotes.value = nextNotes;
  blockedReasonInputs.value = nextBlocked;
}

function sectionLabel(section: TaskSection) {
  if (section.labelKey === "tasks.sectionPublishedMonth" && section.labelParams?.month) {
    const month = String(section.labelParams.month);
    const [year, mon] = month.split("-");
    const monthText = locale.value.startsWith("zh")
      ? `${year}年${Number(mon)}月`
      : new Date(Number(year), Number(mon) - 1, 1).toLocaleDateString("en", {
          month: "long",
          year: "numeric",
        });
    return t(section.labelKey, { ...section.labelParams, month: monthText });
  }
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
    const list = await invoke<PublishTask[]>("list_publish_tasks_cmd", { status: null });
    tasks.value = list;
    syncTaskFields(list);
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

async function updateTask(
  task: PublishTask,
  status: TaskStatus,
  options?: { blockedReason?: string | null }
) {
  error.value = "";
  notice.value = "";
  if (status === "published") {
    const ok = await confirmPublishIfDuplicate(task, t);
    if (!ok) return;
  }
  try {
    await invoke("update_publish_task_status_cmd", {
      taskId: task.id,
      status,
      publishUrl: publishUrls.value[task.id]?.trim() || null,
      note: null,
      blockedReason: options?.blockedReason ?? null,
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
  await updateTask(task, "blocked", { blockedReason: reason });
}

async function unblockTask(task: PublishTask) {
  await updateTask(task, "draft");
}

async function saveTaskChecklist(task: PublishTask, items: string[]) {
  error.value = "";
  notice.value = "";
  try {
    await invoke("update_task_checklist_cmd", { taskId: task.id, checklist: items });
    notice.value = t("tasks.checklistSaved");
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

async function exportCsv() {
  error.value = "";
  notice.value = "";
  const picked = await save({
    defaultPath: `publishkit-tasks-${new Date().toISOString().slice(0, 10)}.csv`,
    filters: [{ name: "CSV", extensions: ["csv"] }],
    title: t("tasks.exportCsvTitle"),
  });
  if (!picked || typeof picked !== "string") return;
  try {
    const result = await invoke<{ rowCount: number }>("export_tasks_csv_cmd", { destPath: picked });
    notice.value = t("tasks.exportCsvDone", { count: result.rowCount });
  } catch (e) {
    error.value = String(e);
  }
}

async function exportMarkdown() {
  error.value = "";
  notice.value = "";
  const picked = await save({
    defaultPath: `publishkit-tasks-${new Date().toISOString().slice(0, 10)}.md`,
    filters: [{ name: "Markdown", extensions: ["md"] }],
    title: t("tasks.exportMarkdownTitle"),
  });
  if (!picked || typeof picked !== "string") return;
  try {
    const result = await invoke<{ rowCount: number }>("export_tasks_markdown_cmd", { destPath: picked });
    notice.value = t("tasks.exportMarkdownDone", { count: result.rowCount });
  } catch (e) {
    error.value = String(e);
  }
}

function statusLabel(status: string) {
  return t(`tasks.status_${status}`, status);
}

watch(filter, () => {
  collapsed.value = new Set(
    sections.value.filter((section) => section.defaultCollapsed).map((section) => section.key)
  );
}, { immediate: true });

onMounted(loadTasks);
</script>

<template>
  <section class="page">
    <header class="head">
      <div>
        <h1>{{ t("tasks.title") }}</h1>
        <p class="muted">{{ t("tasks.subtitleGrouped") }}</p>
      </div>
      <div class="head-actions">
        <button type="button" class="pk-btn pk-btn--ghost" :disabled="loading" @click="exportMarkdown">
          {{ t("tasks.exportMarkdown") }}
        </button>
        <button type="button" class="pk-btn pk-btn--ghost" :disabled="loading" @click="exportCsv">
          {{ t("tasks.exportCsv") }}
        </button>
        <button type="button" class="pk-btn pk-btn--secondary" :disabled="loading" @click="loadTasks">
          {{ t("tasks.refresh") }}
        </button>
      </div>
    </header>

    <p v-if="notice" class="notice">{{ notice }}</p>

    <div class="filters">
      <button
        type="button"
        class="pk-chip"
        :class="{ active: viewMode === 'list' }"
        @click="viewMode = 'list'"
      >
        {{ t("tasks.viewList") }}
      </button>
      <button
        type="button"
        class="pk-chip"
        :class="{ active: viewMode === 'kanban' }"
        @click="viewMode = 'kanban'"
      >
        {{ t("tasks.viewKanban") }}
      </button>
      <span class="filter-sep" />
      <button
        v-for="item in filters"
        :key="item"
        type="button"
        class="pk-chip"
        :class="{ active: filter === item }"
        @click="filter = item"
      >
        {{ item === "all" ? t("tasks.filterAll") : statusLabel(item) }}
      </button>
    </div>

    <p v-if="loading" class="muted">{{ t("tasks.loading") }}</p>
    <p v-else-if="error" class="error">{{ error }}</p>
    <p v-else-if="!hasTasks" class="muted">{{ t("tasks.empty") }}</p>

    <TasksKanbanBoard v-else-if="viewMode === 'kanban'" :tasks="kanbanTasks">
      <template #default="{ task }">
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
          @mark-ready="updateTask(task, 'ready')"
          @mark-published="updateTask(task, 'published')"
          @undo-publish="updateTask(task, 'draft')"
          @mark-blocked="markBlocked(task)"
          @unblock="unblockTask(task)"
          @save-note="saveTaskNote(task)"
          @save-checklist="saveTaskChecklist(task, $event)"
          @save-scheduled="saveScheduled(task)"
        />
      </template>
    </TasksKanbanBoard>

    <div v-else class="sections">
      <section v-for="group in sections" :key="group.key" class="task-section">
        <button type="button" class="section-head" @click="toggleSection(group.key)">
          <span class="chevron" :class="{ collapsed: isCollapsed(group.key) }">›</span>
          <span>{{ sectionLabel(group) }}</span>
          <span class="count">{{ group.tasks.length }}</span>
        </button>

        <ul v-if="!isCollapsed(group.key)" class="list">
          <li v-for="task in group.tasks" :key="task.id">
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
                @mark-ready="updateTask(task, 'ready')"
                @mark-published="updateTask(task, 'published')"
                @undo-publish="updateTask(task, 'draft')"
                @mark-blocked="markBlocked(task)"
                @unblock="unblockTask(task)"
                @save-note="saveTaskNote(task)"
                @save-checklist="saveTaskChecklist(task, $event)"
                @save-scheduled="saveScheduled(task)"
              />
            </TaskCard>
          </li>
        </ul>
      </section>
    </div>
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
.head-actions {
  display: flex;
  gap: var(--pk-space-2);
}
.filters {
  display: flex;
  flex-wrap: wrap;
  gap: var(--pk-space-2);
  align-items: center;
}
.filter-sep {
  width: 1px;
  height: 20px;
  background: var(--pk-border);
  margin: 0 4px;
}
.sections {
  display: flex;
  flex-direction: column;
  gap: var(--pk-space-4);
}
.section-head {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 0;
  border: none;
  background: transparent;
  font: inherit;
  font-weight: 600;
  color: var(--pk-ink);
  cursor: pointer;
  text-align: left;
}
.chevron {
  display: inline-block;
  transform: rotate(90deg);
  transition: transform 0.15s ease;
  color: var(--pk-ink-muted);
}
.chevron.collapsed {
  transform: rotate(0deg);
}
.count {
  margin-left: auto;
  font-size: 12px;
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
