<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import type { MediaAsset, MediaSuggestion } from "@publishkit/shared";
import MediaThumb from "./MediaThumb.vue";

const props = defineProps<{
  contentId: string;
  contentTitle: string;
}>();

const emit = defineEmits<{
  close: [];
  updated: [];
}>();

const { t } = useI18n();
const allAssets = ref<MediaAsset[]>([]);
const suggestions = ref<MediaSuggestion[]>([]);
const linked = ref<MediaAsset[]>([]);
const loading = ref(true);
const saving = ref(false);
const error = ref("");
const selected = ref<Set<string>>(new Set());

const linkedIds = computed(() => new Set(linked.value.map((item) => item.id)));

function reasonLabel(reason: string) {
  return t(`media.suggestReason_${reason}`, reason);
}

async function load() {
  loading.value = true;
  error.value = "";
  try {
    const [assets, linkedItems, suggested] = await Promise.all([
      invoke<MediaAsset[]>("list_media_assets_cmd"),
      invoke<MediaAsset[]>("list_content_media_cmd", { contentItemId: props.contentId }),
      invoke<MediaSuggestion[]>("suggest_media_for_content_cmd", {
        contentItemId: props.contentId,
        limit: 12,
      }),
    ]);
    allAssets.value = assets;
    linked.value = linkedItems;
    suggestions.value = suggested;
    selected.value = new Set(linkedItems.map((item) => item.id));
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

function toggle(id: string) {
  const next = new Set(selected.value);
  if (next.has(id)) next.delete(id);
  else next.add(id);
  selected.value = next;
}

function applySuggestions() {
  const next = new Set(selected.value);
  for (const item of suggestions.value) {
    next.add(item.id);
  }
  selected.value = next;
}

async function save() {
  saving.value = true;
  error.value = "";
  try {
    const toLink = [...selected.value].filter((id) => !linkedIds.value.has(id));
    const toUnlink = [...linkedIds.value].filter((id) => !selected.value.has(id));

    for (const mediaId of toLink) {
      await invoke("link_content_media_cmd", {
        contentItemId: props.contentId,
        mediaAssetId: mediaId,
      });
    }
    for (const mediaId of toUnlink) {
      await invoke("unlink_content_media_cmd", {
        contentItemId: props.contentId,
        mediaAssetId: mediaId,
      });
    }
    emit("updated");
    emit("close");
  } catch (e) {
    error.value = String(e);
  } finally {
    saving.value = false;
  }
}

watch(
  () => props.contentId,
  () => {
    if (props.contentId) load();
  },
  { immediate: true }
);
</script>

<template>
  <div class="overlay" @click.self="emit('close')">
    <section class="panel">
      <header class="header">
        <div>
          <h2>{{ t("media.linkTitle") }}</h2>
          <p class="source">{{ contentTitle }}</p>
        </div>
        <button type="button" class="pk-btn pk-btn--ghost" @click="emit('close')">
          {{ t("common.cancel") }}
        </button>
      </header>

      <p v-if="loading" class="muted">{{ t("media.loading") }}</p>
      <p v-else-if="!allAssets.length" class="muted">{{ t("media.linkEmpty") }}</p>

      <template v-else>
        <section class="suggest-block">
          <div class="suggest-head">
            <h3>{{ t("media.suggestTitle") }}</h3>
            <button
              v-if="suggestions.length"
              type="button"
              class="pk-btn pk-btn--ghost"
              @click="applySuggestions"
            >
              {{ t("media.suggestApply") }}
            </button>
          </div>
          <ul v-if="suggestions.length" class="suggest-list">
            <li v-for="item in suggestions" :key="item.id">
              <label>
                <input type="checkbox" :checked="selected.has(item.id)" @change="toggle(item.id)" />
                <MediaThumb :asset="item" size="md" />
                <div class="text">
                  <strong>{{ item.fileName }}</strong>
                  <span class="reason">{{ reasonLabel(item.reason) }}</span>
                </div>
              </label>
            </li>
          </ul>
          <p v-else class="suggest-empty">{{ t("media.suggestEmpty") }}</p>
        </section>

        <h3 class="all-title">{{ t("media.allAssets") }}</h3>
        <ul class="list">
          <li v-for="item in allAssets" :key="item.id">
            <label>
              <input type="checkbox" :checked="selected.has(item.id)" @change="toggle(item.id)" />
              <MediaThumb :asset="item" size="md" />
              <div class="text">
                <strong>{{ item.fileName }}</strong>
                <span class="path">{{ item.path }}</span>
              </div>
            </label>
          </li>
        </ul>
      </template>

      <p v-if="error" class="error">{{ error }}</p>

      <footer class="footer">
        <button type="button" class="pk-btn pk-btn--primary" :disabled="saving || loading" @click="save">
          {{ saving ? t("media.saving") : t("common.save") }}
        </button>
      </footer>
    </section>
  </div>
</template>

<style scoped>
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
.panel {
  width: min(720px, 100%);
  max-height: calc(100vh - 48px);
  overflow: auto;
  background: var(--pk-bg-panel);
  border: 1px solid var(--pk-border-strong);
  border-radius: 12px;
  padding: 20px;
}
.header {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 12px;
}
.header h2 {
  margin: 0 0 4px;
  font-size: 18px;
}
.source {
  margin: 0;
  font-size: 12px;
  color: var(--pk-ink-muted);
}
.suggest-block {
  margin-bottom: 16px;
  padding: 12px;
  border: 1px solid var(--pk-accent-soft);
  border-radius: 8px;
  background: var(--pk-bg-alt);
}
.suggest-head {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}
.suggest-head h3 {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
}
.suggest-list,
.list {
  list-style: none;
  padding: 0;
  margin: 0;
  max-height: 240px;
  overflow: auto;
}
.all-title {
  margin: 0 0 8px;
  font-size: 13px;
  font-weight: 600;
  color: var(--pk-ink-muted);
}
li {
  border-top: 1px solid var(--pk-border);
  padding: 10px 0;
}
label {
  display: flex;
  gap: 12px;
  align-items: center;
  cursor: pointer;
}
.text {
  min-width: 0;
}
.reason {
  display: block;
  font-size: 11px;
  color: var(--pk-accent-text);
  margin-top: 2px;
}
.suggest-empty {
  margin: 0;
  font-size: 12px;
  color: var(--pk-ink-muted);
  line-height: 1.5;
}
.path {
  display: block;
  font-size: 11px;
  color: var(--pk-ink-muted);
  word-break: break-all;
  margin-top: 2px;
}
.footer {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
}
.muted {
  color: var(--pk-ink-muted);
  font-size: 13px;
}
.error {
  color: #b42318;
  font-size: 13px;
}
</style>
