<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import type { PublishTask, TaskStatus } from "@publishkit/shared";
import TaskCard from "../components/TaskCard.vue";
import TaskActionBar from "../components/TaskActionBar.vue";
import { copyMarkdownAsRichText } from "../utils/clipboard";

const { t } = useI18n();
const tasks = ref<PublishTask[]>([]);
const filter = ref<"all" | TaskStatus>("all");
const loading = ref(false);
const error = ref("");
const copiedId = ref("");
const publishUrls = ref<Record<string, string>>({});

const filters: Array<"all" | TaskStatus> = ["all", "draft", "ready", "published"];

const visibleTasks = computed(() => {
  if (filter.value === "all") return tasks.value;
  return tasks.value.filter((task) => task.status === filter.value);
});

function syncPublishUrls(list: PublishTask[]) {
  const next: Record<string, string> = { ...publishUrls.value };
  for (const task of list) {
    if (!(task.id in next)) next[task.id] = task.publishUrl || "";
    else if (task.publishUrl) next[task.id] = task.publishUrl;
  }
  publishUrls.value = next;
}

async function loadTasks() {
  loading.value = true;
  error.value = "";
  try {
    const list = await invoke<PublishTask[]>("list_publish_tasks_cmd", { status: null });
    tasks.value = list;
    syncPublishUrls(list);
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

async function copyTask(task: PublishTask) {
  await copyMarkdownAsRichText(task.content.body);
  copiedId.value = task.id;
  setTimeout(() => {
    if (copiedId.value === task.id) copiedId.value = "";
  }, 1500);
}

async function updateTask(task: PublishTask, status: TaskStatus) {
  error.value = "";
  try {
    await invoke("update_publish_task_status_cmd", {
      taskId: task.id,
      status,
      publishUrl: publishUrls.value[task.id]?.trim() || null,
    });
    await loadTasks();
  } catch (e) {
    error.value = String(e);
  }
}

function statusLabel(status: string) {
  return t(`tasks.status_${status}`, status);
}

onMounted(loadTasks);
</script>

<template>
  <section class="page">
    <header class="head">
      <div>
        <h1>{{ t("tasks.title") }}</h1>
        <p class="muted">{{ t("tasks.subtitle") }}</p>
      </div>
      <button type="button" class="pk-btn pk-btn--secondary" :disabled="loading" @click="loadTasks">
        {{ t("tasks.refresh") }}
      </button>
    </header>

    <div class="filters">
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
    <p v-else-if="!visibleTasks.length" class="muted">{{ t("tasks.empty") }}</p>

    <ul v-else class="list">
      <li v-for="task in visibleTasks" :key="task.id">
        <TaskCard :task="task">
          <TaskActionBar
            :task="task"
            :copied="copiedId === task.id"
            :publish-url="publishUrls[task.id] ?? ''"
            @update:publish-url="publishUrls[task.id] = $event"
            @copy="copyTask(task)"
            @mark-ready="updateTask(task, 'ready')"
            @mark-published="updateTask(task, 'published')"
            @undo-publish="updateTask(task, 'draft')"
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
.filters {
  display: flex;
  flex-wrap: wrap;
  gap: var(--pk-space-2);
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
</style>
