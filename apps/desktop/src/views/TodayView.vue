<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import type { PublishTask } from "@publishkit/shared";
import TaskCard from "../components/TaskCard.vue";
import TaskActionBar from "../components/TaskActionBar.vue";
import { copyMarkdownAsRichText } from "../utils/clipboard";

const { t } = useI18n();
const tasks = ref<PublishTask[]>([]);
const loading = ref(false);
const error = ref("");
const copiedId = ref("");
const publishUrls = ref<Record<string, string>>({});

async function loadTasks() {
  loading.value = true;
  error.value = "";
  try {
    const list = await invoke<PublishTask[]>("list_today_tasks_cmd");
    tasks.value = list;
    const next: Record<string, string> = {};
    for (const task of list) next[task.id] = task.publishUrl || "";
    publishUrls.value = next;
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

async function markPublished(task: PublishTask) {
  error.value = "";
  try {
    await invoke("update_publish_task_status_cmd", {
      taskId: task.id,
      status: "published",
      publishUrl: publishUrls.value[task.id]?.trim() || null,
    });
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
            @update:publish-url="publishUrls[task.id] = $event"
            @copy="copyTask(task)"
            @mark-published="markPublished(task)"
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
</style>
