<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import type { MediaAsset, ScanMediaResult, WorkspaceSettings } from "@publishkit/shared";
import MediaThumb from "../components/MediaThumb.vue";
import { copyMediaImage, revealMediaInFolder } from "../utils/mediaActions";

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
let thumbRunId = 0;

function formatSize(bytes: number) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function hasMissingThumbs(list: MediaAsset[]) {
  return list.some((item) => item.kind === "image" && !item.thumbPath);
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

onMounted(async () => {
  settings.value = await invoke<WorkspaceSettings>("get_workspace_settings");
  await refreshList();
});

onBeforeUnmount(() => {
  thumbRunId += 1;
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

    <div v-else-if="assets.length" class="grid">
      <article v-for="item in assets" :key="item.id" class="card">
        <button type="button" class="preview-btn" @click="reveal(item)">
          <MediaThumb :asset="item" size="lg" />
        </button>
        <div class="meta">
          <strong>{{ item.fileName }}</strong>
          <span class="size">{{ formatSize(item.sizeBytes) }}</span>
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
  padding: 0;
  border: none;
  background: transparent;
  cursor: pointer;
  align-self: flex-start;
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
