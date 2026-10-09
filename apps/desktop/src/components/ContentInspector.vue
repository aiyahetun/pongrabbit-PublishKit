<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import type { Channel, ContentItem, MediaAsset, PublishTask } from "@publishkit/shared";
import FieldCopyBar from "./FieldCopyBar.vue";
import MediaThumb from "./MediaThumb.vue";

const props = defineProps<{
  item: ContentItem;
  channels: Channel[];
  media: MediaAsset[];
}>();

const emit = defineEmits<{ createTask: [channel: Channel]; changed: [] }>();

const { t } = useI18n();
const tab = ref<"body" | "media" | "tasks" | "note">("body");
const tasks = ref<PublishTask[]>([]);
const note = ref("");
const draftTitle = ref("");
const draftBody = ref("");
const draftKeywords = ref("");
const saving = ref(false);
const saveError = ref("");

const itemTasks = computed(() => tasks.value.filter((task) => task.content.id === props.item.id));

async function loadTasks() {
  tasks.value = await invoke<PublishTask[]>("list_publish_tasks_cmd", { status: null });
}

watch(
  () => props.item.id,
  async () => {
    note.value = "";
    tab.value = "body";
    draftTitle.value = props.item.title;
    draftBody.value = props.item.body;
    draftKeywords.value = (props.item.keywords ?? []).join("、");
    await loadTasks();
  },
  { immediate: true }
);

function keywordList() {
  return draftKeywords.value
    .split(/[、,，;；\n]/)
    .map((word) => word.trim().replace(/^#/, ""))
    .filter(Boolean);
}

async function saveFields() {
  saving.value = true;
  saveError.value = "";
  try {
    await invoke("update_content_fields_cmd", {
      contentItemId: props.item.id,
      title: draftTitle.value,
      body: draftBody.value,
      keywords: keywordList(),
      language: props.item.language,
    });
    emit("changed");
  } catch (e) {
    saveError.value = String(e);
  } finally {
    saving.value = false;
  }
}

async function recognize() {
  saveError.value = "";
  try {
    await invoke("recognize_content_keywords_cmd", { contentItemId: props.item.id });
    emit("changed");
  } catch (e) {
    saveError.value = String(e);
  }
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
      <FieldCopyBar :title="draftTitle" :body="draftBody" :keywords="keywordList()" show-warnings />
      <label class="note-field">
        <span>{{ t("content.fieldTitle") }}</span>
        <input v-model="draftTitle" class="pk-input" type="text" />
      </label>
      <label class="note-field">
        <span>{{ t("content.fieldBody") }}</span>
        <textarea v-model="draftBody" class="pk-input" rows="8" />
      </label>
      <label class="note-field">
        <span>{{ t("content.fieldKeywords") }}</span>
        <input v-model="draftKeywords" class="pk-input" type="text" :placeholder="t('content.keywordsHint')" />
      </label>
      <div class="btn-row">
        <button type="button" class="pk-btn pk-btn--secondary" :disabled="saving" @click="saveFields">
          {{ t("common.save") }}
        </button>
        <button type="button" class="pk-btn pk-btn--ghost" @click="recognize">
          {{ t("content.recognize") }}
        </button>
      </div>
      <p v-if="saveError" class="muted">{{ saveError }}</p>
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
