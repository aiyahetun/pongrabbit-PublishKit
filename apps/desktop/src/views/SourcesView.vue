<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { MarkdownScanItem, WorkspaceSettings } from "@publishkit/shared";
import SplitWizard from "../components/SplitWizard.vue";
import TableImportWizard from "../components/TableImportWizard.vue";

const { t } = useI18n();
const settings = ref<WorkspaceSettings | null>(null);
const scanning = ref(false);
const files = ref<MarkdownScanItem[]>([]);
const error = ref("");
const splitTarget = ref<MarkdownScanItem | null>(null);
const tableTarget = ref<MarkdownScanItem | null>(null);
const importNotice = ref("");

onMounted(async () => {
  settings.value = await invoke<WorkspaceSettings>("get_workspace_settings");
});

async function pickCopyRoot() {
  error.value = "";
  const picked = await open({ directory: true, multiple: false, title: t("sources.pickCopyRoot") });
  if (!picked || Array.isArray(picked)) return;
  settings.value = await invoke<WorkspaceSettings>("set_copy_root", { path: picked });
}

async function pickMediaRoot() {
  error.value = "";
  const picked = await open({ directory: true, multiple: false, title: t("sources.pickMediaRoot") });
  if (!picked || Array.isArray(picked)) return;
  settings.value = await invoke<WorkspaceSettings>("set_media_root", { path: picked });
}

async function scan() {
  if (!settings.value?.copyRoot) return;
  scanning.value = true;
  error.value = "";
  importNotice.value = "";
  try {
    files.value = await invoke<MarkdownScanItem[]>("scan_markdown", { root: settings.value.copyRoot });
  } catch (e) {
    error.value = String(e);
  } finally {
    scanning.value = false;
  }
}

function isTableFormat(format: string) {
  return format === "csv" || format === "xlsx";
}

function openImport(item: MarkdownScanItem) {
  if (isTableFormat(item.format)) {
    tableTarget.value = item;
    splitTarget.value = null;
  } else {
    splitTarget.value = item;
    tableTarget.value = null;
  }
}

function onImported(count: number) {
  splitTarget.value = null;
  tableTarget.value = null;
  importNotice.value = t("split.importDone", { count });
}

function formatLabel(format: string) {
  if (format === "docx") return t("sources.formatDocx");
  if (format === "txt") return t("sources.formatTxt");
  if (format === "pdf") return t("sources.formatPdf");
  if (format === "xlsx") return t("sources.formatXlsx");
  if (format === "csv") return t("sources.formatCsv");
  return t("sources.formatMd");
}

function importActionLabel(format: string) {
  return isTableFormat(format) ? t("tableImport.open") : t("split.open");
}
</script>

<template>
  <section class="page">
    <h1>{{ t("sources.title") }}</h1>

    <div class="card">
      <div class="field">
        <span class="label">{{ t("sources.pickCopyRoot") }}</span>
        <div class="row">
          <input class="pk-input row-input" readonly :value="settings?.copyRoot ?? ''" placeholder="—" />
          <button type="button" class="pk-btn pk-btn--secondary" @click="pickCopyRoot">{{ t("common.choose") }}</button>
        </div>
      </div>
      <div class="field">
        <span class="label">{{ t("sources.pickMediaRoot") }}</span>
        <div class="row">
          <input class="pk-input row-input" readonly :value="settings?.mediaRoot ?? ''" placeholder="—" />
          <button type="button" class="pk-btn pk-btn--secondary" @click="pickMediaRoot">{{ t("common.choose") }}</button>
        </div>
      </div>
      <button
        class="pk-btn pk-btn--primary"
        type="button"
        :disabled="!settings?.copyRoot || scanning"
        @click="scan"
      >
        {{ scanning ? t("sources.scanning") : t("sources.scan") }}
      </button>
      <p v-if="error" class="error">{{ error }}</p>
      <p v-if="importNotice" class="notice">{{ importNotice }}</p>
      <p class="hint">{{ t("sources.formatsHint") }}</p>
    </div>

    <div v-if="files.length" class="list card">
      <p class="meta">{{ t("sources.fileCount", { count: files.length }) }}</p>
      <ul>
        <li v-for="item in files" :key="item.path">
          <div class="file-row">
            <div>
              <strong>{{ item.title }}</strong>
              <span class="format-tag">{{ formatLabel(item.format) }}</span>
              <span class="path">{{ item.path }}</span>
            </div>
            <button type="button" class="pk-btn pk-btn--secondary" @click="openImport(item)">
              {{ importActionLabel(item.format) }}
            </button>
          </div>
        </li>
      </ul>
    </div>

    <SplitWizard
      v-if="splitTarget"
      :path="splitTarget.path"
      :title="splitTarget.title"
      @close="splitTarget = null"
      @imported="onImported"
    />

    <TableImportWizard
      v-if="tableTarget"
      :path="tableTarget.path"
      :title="tableTarget.title"
      @close="tableTarget = null"
      @imported="onImported"
    />
  </section>
</template>

<style scoped>
.page {
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.card {
  background: var(--pk-bg-panel);
  border: 1px solid var(--pk-border-strong);
  border-radius: 8px;
  padding: 16px;
}
.field {
  margin-bottom: 12px;
}
.label {
  display: block;
  font-size: 12px;
  color: var(--pk-ink-muted);
  margin-bottom: 6px;
}
.row {
  display: flex;
  gap: 8px;
}
.row-input {
  flex: 1;
}
.meta {
  font-size: 12px;
  color: var(--pk-ink-muted);
}
ul {
  list-style: none;
  padding: 0;
  margin: 8px 0 0;
  max-height: 360px;
  overflow: auto;
}
li {
  padding: 10px 0;
  border-top: 1px solid var(--pk-border);
}
.file-row {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  align-items: flex-start;
}
.path {
  display: block;
  font-size: 12px;
  color: var(--pk-ink-muted);
  word-break: break-all;
  margin-top: 4px;
}
.format-tag {
  display: inline-block;
  margin-left: 8px;
  padding: 1px 6px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 500;
  color: var(--pk-ink-secondary);
  background: #eceae4;
  vertical-align: middle;
}
.error {
  color: #b42318;
  font-size: 13px;
}
.notice {
  color: var(--pk-accent);
  font-size: 13px;
}
.hint {
  margin: 8px 0 0;
  font-size: 12px;
  color: var(--pk-ink-muted);
}
</style>
