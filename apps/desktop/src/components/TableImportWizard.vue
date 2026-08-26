<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import type { ImportSplitsResult, TableImportPreview } from "@publishkit/shared";

const props = defineProps<{
  path: string;
  title: string;
}>();

const emit = defineEmits<{
  close: [];
  imported: [count: number];
}>();

const { t } = useI18n();
const preview = ref<TableImportPreview | null>(null);
const selected = ref<Set<number>>(new Set());
const replaceExisting = ref(true);
const loading = ref(false);
const importing = ref(false);
const error = ref("");

const rows = computed(() => preview.value?.rows ?? []);

const allSelected = computed(
  () => rows.value.length > 0 && rows.value.every((row) => selected.value.has(row.index))
);

async function loadPreview() {
  loading.value = true;
  error.value = "";
  try {
    preview.value = await invoke<TableImportPreview>("preview_table_import_cmd", {
      path: props.path,
    });
    selected.value = new Set(
      preview.value.rows.filter((row) => row.recommended).map((row) => row.index)
    );
  } catch (e) {
    error.value = String(e);
    preview.value = null;
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
  else selected.value = new Set(rows.value.map((row) => row.index));
}

async function importSelected() {
  importing.value = true;
  error.value = "";
  try {
    const result = await invoke<ImportSplitsResult>("import_table_rows_cmd", {
      path: props.path,
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
  () => props.path,
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
          <h2>{{ t("tableImport.title") }}</h2>
          <p class="source">{{ title }}</p>
          <p v-if="preview" class="mapping">
            {{ t("tableImport.mapping", {
              title: preview.titleColumn,
              body: preview.bodyColumn,
            }) }}
          </p>
        </div>
        <button type="button" class="pk-btn pk-btn--ghost" @click="emit('close')">
          {{ t("common.cancel") }}
        </button>
      </header>

      <div v-if="loading" class="muted">{{ t("tableImport.loading") }}</div>

      <div v-else-if="rows.length" class="preview-block">
        <div class="toolbar">
          <label class="check-all">
            <input type="checkbox" :checked="allSelected" @change="toggleAll" />
            {{ t("tableImport.selectAll") }} ({{ selected.size }}/{{ rows.length }})
          </label>
          <label class="replace">
            <input v-model="replaceExisting" type="checkbox" />
            {{ t("split.replaceExisting") }}
          </label>
        </div>
        <ul>
          <li v-for="row in rows" :key="row.index">
            <label>
              <input
                type="checkbox"
                :checked="selected.has(row.index)"
                @change="toggle(row.index)"
              />
              <div class="item-body">
                <div class="item-head">
                  <strong>{{ row.title }}</strong>
                  <span class="badge">{{ row.language }}</span>
                  <span v-if="row.channelName" class="badge channel">{{ row.channelName }}</span>
                  <span class="lines">#{{ row.index + 1 }}</span>
                </div>
                <p class="preview">{{ row.bodyPreview }}</p>
              </div>
            </label>
          </li>
        </ul>
      </div>

      <p v-else-if="!loading" class="muted">{{ t("tableImport.empty") }}</p>
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
.mapping {
  margin: 0;
  font-size: 12px;
  color: var(--pk-ink-muted);
  word-break: break-all;
}
.mapping {
  margin-top: 6px;
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
.badge.channel {
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
