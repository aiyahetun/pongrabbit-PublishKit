<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import type { Channel, ContentItem, MediaAsset, PublishTask } from "@publishkit/shared";
import { copyMarkdownAsRichText } from "../utils/clipboard";
import MediaThumb from "./MediaThumb.vue";

const props = defineProps<{
  item: ContentItem;
  channels: Channel[];
  media: MediaAsset[];
}>();

const emit = defineEmits<{ createTask: [channel: Channel] }>();

const { t } = useI18n();
const tab = ref<"body" | "media" | "tasks" | "note">("body");
const tasks = ref<PublishTask[]>([]);
const note = ref("");
const copied = ref(false);

const itemTasks = computed(() => tasks.value.filter((task) => task.content.id === props.item.id));

async function loadTasks() {
  tasks.value = await invoke<PublishTask[]>("list_publish_tasks_cmd", { status: null });
}

watch(
  () => props.item.id,
  async () => {
    note.value = "";
    tab.value = "body";
    await loadTasks();
  },
  { immediate: true }
);

async function copyBody() {
  await copyMarkdownAsRichText(props.item.body);
  copied.value = true;
  setTimeout(() => {
    copied.value = false;
  }, 1500);
}
</script>

<template>
  <aside class="inspector">
    <header class="head">
      <h2>{{ item.title }}</h2>
      <p class="meta">{{ item.language }} · {{ item.sourcePath.startsWith("manual://") ? t("content.sourceManual") : t("inspector.fromSource") }}</p>
    </header>

    <div class="tabs">
      <button
        v-for="key in (['body', 'media', 'tasks', 'note'] as const)"
        :key="key"
        type="button"
        class="tab"
        :class="{ active: tab === key }"
        @click="tab = key"
      >
        {{ t(`inspector.tab_${key}`) }}
      </button>
    </div>

    <div v-if="tab === 'body'" class="panel">
      <div class="btn-row">
        <button type="button" class="pk-btn pk-btn--primary" @click="copyBody">
          {{ copied ? t("content.copied") : t("content.copyRich") }}
        </button>
      </div>
      <pre class="body">{{ item.body }}</pre>
    </div>

    <div v-else-if="tab === 'media'" class="panel">
      <p v-if="!media.length" class="muted">{{ t("inspector.mediaEmpty") }}</p>
      <ul v-else class="media-grid">
        <li v-for="asset in media" :key="asset.id">
          <MediaThumb :asset="asset" />
          <span class="file">{{ asset.fileName }}</span>
        </li>
      </ul>
    </div>

    <div v-else-if="tab === 'tasks'" class="panel">
      <p v-if="!itemTasks.length" class="muted">{{ t("inspector.tasksEmpty") }}</p>
      <ul v-else class="task-list">
        <li v-for="task in itemTasks" :key="task.id">
          <span class="dot" :style="{ background: task.channel.color }" />
          <span>{{ task.channel.name }}</span>
          <span class="status">{{ t(`tasks.status_${task.status}`, task.status) }}</span>
        </li>
      </ul>
      <div class="channel-picks">
        <button
          v-for="channel in channels"
          :key="channel.id"
          type="button"
          class="pk-chip"
          @click="emit('createTask', channel)"
        >
          + {{ channel.name }}
        </button>
      </div>
    </div>

    <div v-else class="panel">
      <label class="note-field">
        <span>{{ t("inspector.noteHint") }}</span>
        <textarea v-model="note" class="pk-input" rows="5" :placeholder="t('tasks.notePlaceholder')" />
      </label>
      <p class="muted">{{ t("inspector.noteHelp") }}</p>
    </div>
  </aside>
</template>

<style scoped>
.inspector {
  border-left: 1px solid var(--pk-border);
  background: var(--pk-bg-panel);
  padding: 16px;
  min-width: 320px;
  max-width: 420px;
  overflow: auto;
}
.head h2 {
  margin: 0 0 4px;
  font-size: 16px;
}
.meta {
  margin: 0;
  font-size: 12px;
  color: var(--pk-ink-muted);
}
.tabs {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin: 12px 0;
}
.tab {
  border: 1px solid var(--pk-border);
  background: transparent;
  border-radius: var(--pk-radius-pill);
  padding: 4px 10px;
  font-size: 12px;
  cursor: pointer;
}
.tab.active {
  background: var(--pk-accent-soft);
  border-color: var(--pk-accent);
}
.panel {
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.body {
  white-space: pre-wrap;
  font-size: 13px;
  line-height: 1.5;
  margin: 0;
  max-height: 50vh;
  overflow: auto;
}
.media-grid {
  list-style: none;
  padding: 0;
  margin: 0;
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 8px;
}
.file {
  display: block;
  font-size: 10px;
  color: var(--pk-ink-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.task-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.task-list li {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
}
.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}
.status {
  margin-left: auto;
  font-size: 12px;
  color: var(--pk-ink-muted);
}
.channel-picks {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 8px;
}
.note-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-size: 12px;
  color: var(--pk-ink-muted);
}
.muted {
  font-size: 12px;
  color: var(--pk-ink-muted);
  margin: 0;
}
</style>
