<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import type { ImportSplitsResult, SplitPreview, SplitStrategy } from "@publishkit/shared";

const props = defineProps<{
  path: string;
  title: string;
}>();

const emit = defineEmits<{
  close: [];
  imported: [count: number];
}>();

const { t } = useI18n();
const strategy = ref<SplitStrategy>("smart");
const previews = ref<SplitPreview[]>([]);
const selected = ref<Set<number>>(new Set());
const replaceExisting = ref(true);
const loading = ref(false);
const importing = ref(false);
const error = ref("");

const strategies: SplitStrategy[] = ["smart", "h1", "h2", "h3", "whole"];

const selectablePreviews = computed(() => previews.value.filter((p) => p.recommended));

const allSelected = computed(
  () =>
    selectablePreviews.value.length > 0 &&
    selectablePreviews.value.every((p) => selected.value.has(p.index))
);

function kindLabel(kind: SplitPreview["sectionKind"]) {
  return t(`split.kind_${kind}`);
}

async function loadPreview() {
  loading.value = true;
  error.value = "";
  try {
    previews.value = await invoke<SplitPreview[]>("preview_md_splits", {
      path: props.path,
      strategy: strategy.value,
    });
    selected.value = new Set(
      previews.value.filter((p) => p.recommended).map((p) => p.index)
    );
  } catch (e) {
    error.value = String(e);
    previews.value = [];
    selected.value = new Set();
  } finally {
    loading.value = false;
  }
}

function toggle(index: number) {
  const next = new Set(selected.value);
  if (next.has(index)) next.delete(index);
  else next.add(index);
  selected.value = next;
}

function toggleAll() {
  if (allSelected.value) selected.value = new Set();
  else selected.value = new Set(selectablePreviews.value.map((p) => p.index));
}

async function importSelected() {
  importing.value = true;
  error.value = "";
  try {
    const result = await invoke<ImportSplitsResult>("import_md_splits", {
      path: props.path,
      strategy: strategy.value,
      selectedIndexes: [...selected.value],
      replaceExisting: replaceExisting.value,
    });
    emit("imported", result.importedCount);
  } catch (e) {
    error.value = String(e);
  } finally {
    importing.value = false;
  }
}

watch(
  () => [props.path, strategy.value] as const,
  () => {
    if (props.path) loadPreview();
  },
  { immediate: true }
);
</script>

<template>
  <div class="overlay" @click.self="emit('close')">
    <section class="wizard card">
      <header class="header">
        <div>
          <h2>{{ t("split.title") }}</h2>
          <p class="source">{{ title }}</p>
          <p class="hint">{{ t("split.metaHint") }}</p>
        </div>
        <button type="button" class="pk-btn pk-btn--ghost" @click="emit('close')">
          {{ t("common.cancel") }}
        </button>
      </header>

      <div class="field">
        <span class="label">{{ t("split.strategy") }}</span>
        <div class="strategy-row">
          <label v-for="item in strategies" :key="item" class="chip">
            <input v-model="strategy" type="radio" :value="item" />
            {{ t(`split.strategy_${item}`) }}
          </label>
        </div>
      </div>

      <div v-if="loading" class="muted">{{ t("split.loading") }}</div>

      <div v-else-if="previews.length" class="preview-block">
        <div class="toolbar">
          <label class="check-all">
            <input type="checkbox" :checked="allSelected" @change="toggleAll" />
            {{ t("split.selectRecommended") }} ({{ selected.size }}/{{ previews.length }})
          </label>
          <label class="replace">
            <input v-model="replaceExisting" type="checkbox" />
            {{ t("split.replaceExisting") }}
          </label>
        </div>
        <ul>
          <li v-for="item in previews" :key="item.index" :class="{ dim: !item.recommended }">
            <label>
              <input
                type="checkbox"
                :checked="selected.has(item.index)"
                @change="toggle(item.index)"
              />
              <div class="item-body">
                <div class="item-head">
                  <strong>{{ item.title }}</strong>
                  <span class="badge kind" :data-kind="item.sectionKind">{{ kindLabel(item.sectionKind) }}</span>
                  <span class="badge">{{ item.language }}</span>
                  <span class="lines">L{{ item.startLine }}–{{ item.endLine }}</span>
                </div>
                <p class="preview">{{ item.bodyPreview }}</p>
              </div>
            </label>
          </li>
        </ul>
      </div>

      <p v-else-if="!loading" class="muted">{{ t("split.empty") }}</p>
      <p v-if="error" class="error">{{ error }}</p>

      <footer class="footer">
        <button type="button" class="pk-btn pk-btn--secondary" :disabled="loading" @click="loadPreview">
          {{ t("split.refresh") }}
        </button>
        <button
          type="button"
          class="pk-btn pk-btn--primary"
          :disabled="importing || selected.size === 0"
          @click="importSelected"
        >
          {{ importing ? t("split.importing") : t("split.import", { count: selected.size }) }}
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
.wizard {
  width: min(760px, 100%);
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
  margin-bottom: 16px;
}
.header h2 {
  margin: 0 0 4px;
  font-size: 18px;
}
.source,
.hint {
  margin: 0;
  font-size: 12px;
  color: var(--pk-ink-muted);
  word-break: break-all;
}
.hint {
  margin-top: 6px;
}
.field {
  margin-bottom: 12px;
}
.label {
  display: block;
  font-size: 12px;
  color: var(--pk-ink-muted);
  margin-bottom: 8px;
}
.strategy-row {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
.chip {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  border: 1px solid var(--pk-border-strong);
  border-radius: 999px;
  padding: 6px 12px;
  font-size: 13px;
}
.toolbar {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 8px;
  font-size: 13px;
}
ul {
  list-style: none;
  padding: 0;
  margin: 0;
  max-height: 360px;
  overflow: auto;
}
li {
  border-top: 1px solid var(--pk-border);
  padding: 10px 0;
}
li.dim {
  opacity: 0.72;
}
li label {
  display: flex;
  gap: 10px;
  align-items: flex-start;
  cursor: pointer;
}
.item-head {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
  margin-bottom: 4px;
}
.badge {
  font-size: 11px;
  padding: 2px 6px;
  border-radius: 999px;
  background: var(--pk-accent-soft);
}
.badge.kind[data-kind="meta"] {
  background: #f2f0eb;
  color: #8a8278;
}
.badge.kind[data-kind="platform"] {
  background: #e8f4ff;
  color: #175cd3;
}
.lines {
  font-size: 11px;
  color: var(--pk-ink-muted);
}
.preview {
  margin: 0;
  font-size: 12px;
  color: var(--pk-ink-secondary);
  white-space: pre-wrap;
}
.footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
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
