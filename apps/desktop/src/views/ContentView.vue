<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { ask, open } from "@tauri-apps/plugin-dialog";
import type { Channel, ContentItem, DeleteContentResult, MediaAsset } from "@publishkit/shared";
import { copyMarkdownAsRichText } from "../utils/clipboard";
import MediaLinkDialog from "../components/MediaLinkDialog.vue";
import MediaThumb from "../components/MediaThumb.vue";
import { copyLinkedImagesWorkflow } from "../utils/mediaActions";

const { t } = useI18n();
const items = ref<ContentItem[]>([]);
const channels = ref<Channel[]>([]);
const loading = ref(false);
const copiedId = ref("");
const error = ref("");
const notice = ref("");
const showCreate = ref(false);
const showTaskFor = ref<ContentItem | null>(null);
const showMediaFor = ref<ContentItem | null>(null);
const itemMedia = ref<Record<string, MediaAsset[]>>({});
const newTitle = ref("");
const newBody = ref("");
const creating = ref(false);
const taskCreating = ref(false);
const quickChannelName = ref("");
const selectedIds = ref<Set<string>>(new Set());
const deleting = ref(false);

const selectedCount = computed(() => selectedIds.value.size);
const allSelected = computed(
  () => items.value.length > 0 && selectedIds.value.size === items.value.length
);
const hasSelection = computed(() => selectedIds.value.size > 0);

const domesticChannels = computed(() =>
  channels.value.filter((c) => !c.isCustom && (c.market === "domestic" || c.market === "both"))
);
const overseasChannels = computed(() =>
  channels.value.filter((c) => !c.isCustom && (c.market === "overseas" || c.market === "both"))
);
const customChannels = computed(() => channels.value.filter((c) => c.isCustom));

async function loadItems() {
  loading.value = true;
  error.value = "";
  try {
    items.value = await invoke<ContentItem[]>("list_content_items_cmd");
    const valid = new Set(items.value.map((item) => item.id));
    selectedIds.value = new Set([...selectedIds.value].filter((id) => valid.has(id)));
    await refreshMediaMap();
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

async function refreshMediaMap() {
  const map: Record<string, MediaAsset[]> = {};
  await Promise.all(
    items.value.map(async (item) => {
      map[item.id] = await invoke<MediaAsset[]>("list_content_media_cmd", {
        contentItemId: item.id,
      });
    })
  );
  itemMedia.value = map;
}

async function loadChannels() {
  channels.value = await invoke<Channel[]>("list_channels_cmd");
}

function sourceLabel(path: string) {
  if (path.startsWith("manual://")) return t("content.sourceManual");
  return path;
}

async function copyBody(item: ContentItem) {
  try {
    await copyMarkdownAsRichText(item.body);
    copiedId.value = item.id;
    setTimeout(() => {
      if (copiedId.value === item.id) copiedId.value = "";
    }, 1500);
  } catch (e) {
    error.value = String(e);
  }
}

async function createManual() {
  creating.value = true;
  error.value = "";
  try {
    const created = await invoke<ContentItem>("create_manual_content", {
      title: newTitle.value,
      body: newBody.value,
    });
    items.value = [created, ...items.value];
    showCreate.value = false;
    newTitle.value = "";
    newBody.value = "";
  } catch (e) {
    error.value = String(e);
  } finally {
    creating.value = false;
  }
}

async function createTask(item: ContentItem, channel: Channel) {
  taskCreating.value = true;
  error.value = "";
  notice.value = "";
  try {
    await invoke("create_publish_task_cmd", {
      contentItemId: item.id,
      channelId: channel.id,
    });
    showTaskFor.value = null;
    notice.value = t("content.taskCreated", { channel: channel.name });
  } catch (e) {
    error.value = String(e);
  } finally {
    taskCreating.value = false;
  }
}

async function copyLinkedImagesForItem(item: ContentItem) {
  error.value = "";
  notice.value = "";
  const assets = itemMedia.value[item.id] ?? [];
  try {
    const result = await copyLinkedImagesWorkflow(item.id, assets);
    if (result.mode === "none") return;
    if (result.mode === "clipboard") {
      notice.value = t("media.copied");
      return;
    }
    notice.value = t("media.stagedMultiple", { count: result.count });
  } catch (e) {
    error.value = String(e);
  }
}

function linkedImageCount(itemId: string) {
  return (itemMedia.value[itemId] ?? []).filter((asset) => asset.kind === "image").length;
}

async function exportPack(item: ContentItem) {
  error.value = "";
  notice.value = "";
  const picked = await open({
    directory: true,
    multiple: false,
    title: t("content.exportPackTitle"),
  });
  if (!picked || typeof picked !== "string") return;
  try {
    const result = await invoke<{ folderPath: string; mediaCount: number }>("export_content_pack_cmd", {
      contentItemId: item.id,
      destFolder: picked,
    });
    notice.value = t("content.exportPackDone", {
      path: result.folderPath,
      count: result.mediaCount,
    });
  } catch (e) {
    error.value = String(e);
  }
}

function toggleSelected(id: string) {
  const next = new Set(selectedIds.value);
  if (next.has(id)) next.delete(id);
  else next.add(id);
  selectedIds.value = next;
}

function toggleSelectAll() {
  if (allSelected.value) {
    selectedIds.value = new Set();
    return;
  }
  selectedIds.value = new Set(items.value.map((item) => item.id));
}

async function deleteItems(ids: string[], confirmKey: string, confirmParams?: Record<string, unknown>) {
  if (!ids.length) return;
  const confirmed = await ask(t(confirmKey, confirmParams ?? {}), {
    title: t("content.deleteTitle"),
    kind: "warning",
  });
  if (!confirmed) return;

  deleting.value = true;
  error.value = "";
  notice.value = "";
  try {
    const result = await invoke<DeleteContentResult>("delete_content_items_cmd", {
      contentItemIds: ids,
    });
    notice.value = t("content.deleted", { count: result.deletedCount });
    selectedIds.value = new Set();
    await loadItems();
  } catch (e) {
    error.value = String(e);
  } finally {
    deleting.value = false;
  }
}

async function deleteOne(item: ContentItem) {
  await deleteItems([item.id], "content.deleteConfirmOne", { title: item.title });
}

async function deleteSelected() {
  await deleteItems([...selectedIds.value], "content.deleteConfirmMany", {
    count: selectedIds.value.size,
  });
}

async function addQuickChannel() {
  if (!quickChannelName.value.trim()) return;
  error.value = "";
  try {
    await invoke("create_channel_cmd", {
      name: quickChannelName.value,
      market: "both",
    });
    quickChannelName.value = "";
    await loadChannels();
  } catch (e) {
    error.value = String(e);
  }
}

onMounted(async () => {
  await Promise.all([loadItems(), loadChannels()]);
});
</script>

<template>
  <section class="page">
    <header class="head">
      <div>
        <h1>{{ t("content.title") }}</h1>
        <p class="muted">{{ t("content.subtitle") }}</p>
      </div>
      <div class="actions">
        <button type="button" class="pk-btn pk-btn--primary" @click="showCreate = true">{{ t("content.create") }}</button>
        <button
          v-if="hasSelection"
          type="button"
          class="pk-btn pk-btn--ghost danger"
          :disabled="deleting || loading"
          @click="deleteSelected"
        >
          {{ deleting ? t("content.deleting") : t("content.deleteSelected", { count: selectedCount }) }}
        </button>
        <button type="button" class="pk-btn pk-btn--secondary" :disabled="loading" @click="loadItems">{{ t("content.refresh") }}</button>
      </div>
    </header>

    <div v-if="items.length" class="bulk-bar card">
      <label class="select-all">
        <input type="checkbox" :checked="allSelected" @change="toggleSelectAll" />
        <span>{{ t("content.selectAll") }}</span>
      </label>
      <span v-if="hasSelection" class="selected-count">
        {{ t("content.selectedCount", { count: selectedCount }) }}
      </span>
    </div>

    <p v-if="notice" class="notice">{{ notice }}</p>
    <p v-if="loading" class="muted">{{ t("content.loading") }}</p>
    <p v-else-if="error" class="error">{{ error }}</p>
    <p v-else-if="!items.length" class="muted">{{ t("content.empty") }}</p>

    <ul v-else class="list card">
      <li v-for="item in items" :key="item.id" :class="{ selected: selectedIds.has(item.id) }">
        <div class="row">
          <label class="item-check">
            <input type="checkbox" :checked="selectedIds.has(item.id)" @change="toggleSelected(item.id)" />
          </label>
          <div class="meta">
            <strong>{{ item.title }}</strong>
            <span class="badge">{{ item.language }}</span>
          </div>
          <div class="row-actions">
            <button
              v-if="linkedImageCount(item.id) > 0"
              type="button"
              class="pk-btn pk-btn--ghost"
              @click="copyLinkedImagesForItem(item)"
            >
              {{
                linkedImageCount(item.id) > 1
                  ? t("media.copyImages", { count: linkedImageCount(item.id) })
                  : t("media.copyImage")
              }}
            </button>
            <button type="button" class="pk-btn pk-btn--ghost" @click="showMediaFor = item">
              {{ t("content.linkMedia") }}
              <span v-if="itemMedia[item.id]?.length" class="count">{{ itemMedia[item.id].length }}</span>
            </button>
            <button type="button" class="pk-btn pk-btn--ghost" @click="exportPack(item)">
              {{ t("content.exportPack") }}
            </button>
            <button type="button" class="pk-btn pk-btn--ghost" @click="showTaskFor = item">{{ t("content.addTask") }}</button>
            <button type="button" class="pk-btn pk-btn--secondary" @click="copyBody(item)">
              {{ copiedId === item.id ? t("content.copied") : t("content.copyRich") }}
            </button>
            <button
              type="button"
              class="pk-btn pk-btn--ghost danger"
              :disabled="deleting"
              @click="deleteOne(item)"
            >
              {{ t("content.deleteOne") }}
            </button>
          </div>
        </div>
        <p class="preview">{{ item.body.slice(0, 240) }}{{ item.body.length > 240 ? "…" : "" }}</p>
        <ul v-if="itemMedia[item.id]?.length" class="media-list">
          <li v-for="media in itemMedia[item.id]" :key="media.id">
            <MediaThumb :asset="media" size="sm" />
            <span>{{ media.fileName }}</span>
          </li>
        </ul>
        <span class="path">{{ sourceLabel(item.sourcePath) }}</span>
      </li>
    </ul>

    <div v-if="showCreate" class="overlay" @click.self="showCreate = false">
      <form class="modal card" @submit.prevent="createManual">
        <h2>{{ t("content.createTitle") }}</h2>
        <label>
          <span>{{ t("content.fieldTitle") }}</span>
          <input v-model="newTitle" required />
        </label>
        <label>
          <span>{{ t("content.fieldBody") }}</span>
          <textarea v-model="newBody" rows="10" required />
        </label>
        <p class="muted">{{ t("content.createHint") }}</p>
        <div class="footer">
          <button type="button" class="pk-btn pk-btn--secondary" @click="showCreate = false">
            {{ t("common.cancel") }}
          </button>
          <button type="submit" class="pk-btn pk-btn--primary" :disabled="creating">
            {{ creating ? t("content.creating") : t("common.save") }}
          </button>
        </div>
      </form>
    </div>

    <MediaLinkDialog
      v-if="showMediaFor"
      :content-id="showMediaFor.id"
      :content-title="showMediaFor.title"
      @close="showMediaFor = null"
      @updated="refreshMediaMap"
    />

    <div v-if="showTaskFor" class="overlay" @click.self="showTaskFor = null">
      <section class="modal card picker">
        <h2>{{ t("content.pickChannel") }}</h2>
        <p class="muted">{{ showTaskFor.title }}</p>

        <div class="group">
          <h3>{{ t("channels.domestic") }}</h3>
          <div class="channels">
            <button
              v-for="channel in domesticChannels"
              :key="channel.id"
              type="button"
              class="pk-btn pk-btn--secondary channel-btn"
              :disabled="taskCreating"
              @click="createTask(showTaskFor, channel)"
            >
              <span class="dot" :style="{ background: channel.color }" />
              {{ channel.name }}
            </button>
          </div>
        </div>

        <div class="group">
          <h3>{{ t("channels.overseas") }}</h3>
          <div class="channels">
            <button
              v-for="channel in overseasChannels"
              :key="channel.id"
              type="button"
              class="pk-btn pk-btn--secondary channel-btn"
              :disabled="taskCreating"
              @click="createTask(showTaskFor, channel)"
            >
              <span class="dot" :style="{ background: channel.color }" />
              {{ channel.name }}
            </button>
          </div>
        </div>

        <div v-if="customChannels.length" class="group">
          <h3>{{ t("channels.custom") }}</h3>
          <div class="channels">
            <button
              v-for="channel in customChannels"
              :key="channel.id"
              type="button"
              class="pk-btn pk-btn--secondary channel-btn"
              :disabled="taskCreating"
              @click="createTask(showTaskFor, channel)"
            >
              <span class="dot" :style="{ background: channel.color }" />
              {{ channel.name }}
            </button>
          </div>
        </div>

        <form class="quick-add" @submit.prevent="addQuickChannel">
          <input v-model="quickChannelName" class="pk-input" :placeholder="t('channels.namePlaceholder')" />
          <button type="submit" class="pk-btn pk-btn--ghost">{{ t("channels.add") }}</button>
        </form>

        <div class="footer">
          <button type="button" class="pk-btn pk-btn--secondary" @click="showTaskFor = null">
            {{ t("common.cancel") }}
          </button>
        </div>
      </section>
    </div>
  </section>
</template>

<style scoped>
.page {
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.head {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  align-items: flex-start;
}
.head h1 {
  margin: 0 0 4px;
}
.actions {
  display: flex;
  gap: 8px;
}
.row-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}
.card {
  background: var(--pk-bg-panel);
  border: 1px solid var(--pk-border-strong);
  border-radius: 8px;
  padding: 8px 16px;
}
.list {
  list-style: none;
  margin: 0;
  padding: 8px 16px;
}
li {
  padding: 14px 0;
  border-top: 1px solid var(--pk-border);
}
li:first-child {
  border-top: none;
}
.row {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  align-items: center;
}
.item-check {
  flex-shrink: 0;
  display: flex;
  align-items: center;
}
.bulk-bar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 16px;
}
.select-all {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--pk-ink-secondary);
  cursor: pointer;
}
.selected-count {
  font-size: 12px;
  color: var(--pk-ink-muted);
}
li.selected {
  background: var(--pk-bg-alt);
  margin: 0 -16px;
  padding-left: 16px;
  padding-right: 16px;
}
.danger {
  color: var(--pk-status-blocked);
}
.danger:hover {
  color: var(--pk-status-blocked);
  background: rgba(180, 35, 24, 0.08);
}
.meta {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
}
.badge {
  font-size: 11px;
  padding: 2px 6px;
  border-radius: 999px;
  background: var(--pk-accent-soft);
}
.preview {
  margin: 8px 0 4px;
  font-size: 13px;
  color: var(--pk-ink-secondary);
  white-space: pre-wrap;
}
.media-list {
  list-style: none;
  padding: 0;
  margin: 0 0 6px;
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}
.media-list li {
  border: none;
  padding: 0;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  color: var(--pk-ink-secondary);
  background: var(--pk-bg-alt);
  border-radius: var(--pk-radius-pill);
  padding: 2px 8px 2px 2px;
}
.count {
  margin-left: 4px;
  font-size: 11px;
  color: var(--pk-accent-text);
}
.path {
  display: block;
  font-size: 11px;
  color: var(--pk-ink-muted);
  word-break: break-all;
}
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.35);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 20;
  padding: 24px;
}
.modal {
  width: min(560px, 100%);
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.modal h2 {
  margin: 0;
  font-size: 18px;
}
label {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-size: 12px;
  color: var(--pk-ink-muted);
}
input,
textarea {
  border: 1px solid var(--pk-border-strong);
  border-radius: 8px;
  padding: 8px 10px;
  background: var(--pk-bg-app);
  font: inherit;
  color: inherit;
}
.picker {
  max-height: calc(100vh - 48px);
  overflow: auto;
}
.picker h3 {
  margin: 0 0 var(--pk-space-2);
  font-size: 12px;
  font-weight: 600;
  color: var(--pk-accent-text);
}
.group {
  margin-top: var(--pk-space-3);
}
.channels {
  display: flex;
  flex-wrap: wrap;
  gap: var(--pk-space-2);
}
.channel-btn {
  gap: 8px;
}
.quick-add {
  display: flex;
  gap: var(--pk-space-2);
  margin-top: var(--pk-space-4);
  padding-top: var(--pk-space-4);
  border-top: 1px solid var(--pk-border);
}
.quick-add .pk-input {
  flex: 1;
}
.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}
.footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
.muted {
  color: var(--pk-ink-muted);
}
.notice {
  color: var(--pk-accent);
  font-size: 13px;
}
.error {
  color: #b42318;
}
</style>
