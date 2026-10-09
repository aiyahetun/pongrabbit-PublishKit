<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import type { MediaAsset, ScanMediaResult, WorkspaceSettings } from "@publishkit/shared";
import MediaThumb from "../components/MediaThumb.vue";
import { copyMediaImage, revealMediaInFolder } from "../utils/mediaActions";
import { mediaPreviewSrc } from "../utils/mediaPreview";

type ThumbBatchResult = { generated: number; remaining: number };

const { t } = useI18n();
const assets = ref<MediaAsset[]>([]);
const usagesByMedia = ref<Record<string, Array<{ id: string; title: string }>>>({});
const settings = ref<WorkspaceSettings | null>(null);
const loading = ref(false);
const scanning = ref(false);
const thumbing = ref(false);
const error = ref("");
const notice = ref("");
const copiedId = ref("");
const failedOnly = ref(false);
const previewAsset = ref<MediaAsset | null>(null);
let thumbRunId = 0;

function formatSize(bytes: number) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function hasMissingThumbs(list: MediaAsset[]) {
  return list.some(
    (item) => item.kind === "image" && item.thumbStatus !== "failed" && item.thumbStatus !== "ready" && !item.thumbPath
  );
}

const visibleAssets = computed(() =>
  failedOnly.value ? assets.value.filter((item) => item.thumbStatus === "failed") : assets.value
);

function pixelLabel(item: MediaAsset) {
  if (!item.width || !item.height) return "";
  return t("media.pixels", { width: item.width, height: item.height });
}

function isLong(item: MediaAsset) {
  return !!item.width && !!item.height && item.height / item.width >= 2.2;
}

async function openAsset(item: MediaAsset) {
  if (item.thumbPath) {
    previewAsset.value = item;
    return;
  }
  await invoke("open_media_file_cmd", { path: item.path });
}

async function retryFailed() {
  await invoke("retry_failed_thumbnails_cmd");
  await refreshList();
}

async function loadUsages() {
  try {
    const rows = await invoke<
      Array<{ mediaAssetId: string; contentId: string; contentTitle: string }>
    >("list_media_content_usages_cmd");
    const next: Record<string, Array<{ id: string; title: string }>> = {};
    for (const row of rows) {
      (next[row.mediaAssetId] ??= []).push({
        id: row.contentId,
        title: row.contentTitle,
      });
    }
    usagesByMedia.value = next;
  } catch (e) {
    error.value = String(e);
  }
}

async function loadAssets() {
  loading.value = true;
  error.value = "";
  try {
    assets.value = await invoke<MediaAsset[]>("list_media_assets_cmd");
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

async function generateThumbnailsInBackground() {
  const runId = ++thumbRunId;
  thumbing.value = true;
  try {
    while (runId === thumbRunId) {
      const result = await invoke<ThumbBatchResult>("generate_media_thumbnails_cmd", {
        batchSize: 24,
      });
      if (result.generated > 0) {
        assets.value = await invoke<MediaAsset[]>("list_media_assets_cmd");
      }
      if (result.remaining === 0) break;
      await new Promise((resolve) => setTimeout(resolve, 30));
    }
  } catch (e) {
    if (runId === thumbRunId) error.value = String(e);
  } finally {
    if (runId === thumbRunId) thumbing.value = false;
  }
}

async function refreshList() {
  await Promise.all([loadAssets(), loadUsages()]);
  if (hasMissingThumbs(assets.value)) {
    void generateThumbnailsInBackground();
  }
}

async function scan() {
  scanning.value = true;
  error.value = "";
  notice.value = "";
  try {
    const result = await invoke<ScanMediaResult>("scan_media_cmd");
    notice.value = t("media.scanDone", {
      indexed: result.indexedCount,
      total: result.totalCount,
    });
    await refreshList();
  } catch (e) {
    error.value = String(e);
  } finally {
    scanning.value = false;
  }
}

async function copyImage(item: MediaAsset) {
  error.value = "";
  try {
    await copyMediaImage(item);
    copiedId.value = item.id;
    notice.value = t("media.copied");
    setTimeout(() => {
      if (copiedId.value === item.id) copiedId.value = "";
    }, 1500);
  } catch (e) {
    error.value = String(e);
  }
}

async function reveal(item: MediaAsset) {
  error.value = "";
  try {
    await revealMediaInFolder(item);
  } catch (e) {
    error.value = String(e);
  }
}

async function onProjectChanged() {
  try {
    settings.value = await invoke<WorkspaceSettings>("get_workspace_settings");
    await refreshList();
  } catch (e) {
    error.value = String(e);
  }
}

onMounted(async () => {
  window.addEventListener("publishkit-project-changed", onProjectChanged);
  settings.value = await invoke<WorkspaceSettings>("get_workspace_settings");
  await refreshList();
});

onBeforeUnmount(() => {
  thumbRunId += 1;
  window.removeEventListener("publishkit-project-changed", onProjectChanged);
});
</script>

<template>
  <section class="page">
    <header class="header">
      <div>
        <h1>{{ t("media.title") }}</h1>
        <p class="subtitle">{{ t("media.subtitle") }}</p>
      </div>
      <div class="header-actions">
        <button type="button" class="pk-btn pk-btn--ghost" @click="failedOnly = !failedOnly">
          {{ t("media.filterFailed") }}
        </button>
        <button type="button" class="pk-btn pk-btn--ghost" @click="retryFailed">
          {{ t("media.retryFailed") }}
        </button>
        <button type="button" class="pk-btn pk-btn--secondary" :disabled="loading" @click="refreshList">
          {{ t("media.refresh") }}
        </button>
        <button
          type="button"
          class="pk-btn pk-btn--primary"
          :disabled="!settings?.mediaRoot || scanning"
          @click="scan"
        >
          {{ scanning ? t("media.scanning") : t("media.scan") }}
        </button>
      </div>
    </header>

    <p v-if="!settings?.mediaRoot" class="hint">{{ t("media.noRoot") }}</p>
    <p v-if="thumbing" class="notice">{{ t("media.thumbGenerating") }}</p>
    <p v-else-if="notice" class="notice">{{ notice }}</p>
    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="loading" class="muted">{{ t("media.loading") }}</p>

    <div v-else-if="visibleAssets.length" class="grid">
      <article v-for="item in visibleAssets" :key="item.id" class="card">
        <button type="button" class="preview-btn" @click="openAsset(item)">
          <MediaThumb :asset="item" size="lg" />
          <span v-if="isLong(item)" class="badge">{{ t("media.longImage") }}</span>
        </button>
        <div class="meta">
          <strong>{{ item.fileName }}</strong>
          <span class="size">{{ pixelLabel(item) }} {{ formatSize(item.sizeBytes) }}</span>
          <p v-if="item.thumbStatus === 'failed'" class="usage-empty">
            {{ t("media.thumbFailed") }}
            <button type="button" class="pk-btn pk-btn--ghost" @click="invoke('open_media_file_cmd', { path: item.path })">
              {{ t("media.openOriginal") }}
            </button>
          </p>
          <span class="path">{{ item.path }}</span>
          <div v-if="usagesByMedia[item.id]?.length" class="usage">
            <span class="usage-label">{{ t("media.usageTitle") }}</span>
            <ul class="usage-list">
              <li v-for="usage in usagesByMedia[item.id]" :key="usage.id">{{ usage.title }}</li>
            </ul>
          </div>
          <p v-else class="usage-empty">{{ t("media.usageEmpty") }}</p>
          <div class="actions">
            <button
              v-if="item.kind === 'image'"
              type="button"
              class="pk-btn pk-btn--ghost"
              @click="copyImage(item)"
            >
              {{ copiedId === item.id ? t("media.copied") : t("media.copyImage") }}
            </button>
            <button type="button" class="pk-btn pk-btn--ghost" @click="reveal(item)">
              {{ t("media.revealInFolder") }}
            </button>
          </div>
        </div>
      </article>
    </div>

    <p v-else class="muted">{{ t("media.empty") }}</p>

    <div v-if="previewAsset" class="overlay" @click.self="previewAsset = null">
      <div class="preview-dialog">
        <header>
          <strong>{{ previewAsset.fileName }}</strong>
          <button type="button" class="pk-btn pk-btn--ghost" @click="previewAsset = null">{{ t("common.cancel") }}</button>
        </header>
        <img :src="mediaPreviewSrc(previewAsset) ?? undefined" :alt="previewAsset.fileName" />
        <button type="button" class="pk-btn pk-btn--secondary" @click="invoke('open_media_file_cmd', { path: previewAsset.path })">
          {{ t("media.openOriginal") }}
        </button>
      </div>
    </div>
  </section>
</template>

<style scoped>
.page {
  padding: 24px;
}
.header {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  align-items: flex-start;
  margin-bottom: 16px;
}
.header h1 {
  margin: 0 0 6px;
}
.subtitle {
  margin: 0;
  color: var(--pk-ink-muted);
  font-size: 13px;
}
.header-actions {
  display: flex;
  gap: 8px;
}
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 12px;
}
.card {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 12px;
  background: var(--pk-bg-panel);
  border: 1px solid var(--pk-border-strong);
  border-radius: var(--pk-radius-md);
}
.preview-btn {
  position: relative;
  padding: 0;
  border: none;
  background: transparent;
  cursor: pointer;
  align-self: flex-start;
}
.badge {
  position: absolute;
  left: 6px;
  top: 6px;
  background: rgba(0, 0, 0, 0.62);
  color: white;
  font-size: 11px;
  padding: 2px 6px;
  border-radius: 99px;
}
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 30;
}
.preview-dialog {
  background: var(--pk-bg-panel);
  padding: 16px;
  border-radius: 12px;
  max-width: 760px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.preview-dialog img {
  max-width: 720px;
  max-height: 70vh;
  object-fit: contain;
}
.preview-dialog header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}
.meta {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.meta strong {
  font-size: 13px;
  word-break: break-all;
}
.path {
  font-size: 11px;
  color: var(--pk-ink-muted);
  word-break: break-all;
}
.size {
  font-size: 11px;
  color: var(--pk-ink-secondary);
}
.usage {
  margin-top: 4px;
}
.usage-label {
  display: block;
  font-size: 11px;
  color: var(--pk-ink-muted);
  margin-bottom: 2px;
}
.usage-list {
  margin: 0;
  padding-left: 16px;
  font-size: 11px;
  color: var(--pk-ink-secondary);
}
.usage-empty {
  margin: 4px 0 0;
  font-size: 11px;
  color: var(--pk-ink-muted);
}
.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  margin-top: 4px;
}
.hint,
.notice,
.error,
.muted {
  font-size: 13px;
}
.hint,
.muted {
  color: var(--pk-ink-muted);
}
.notice {
  color: var(--pk-accent-text);
}
.error {
  color: #b42318;
}
</style>
