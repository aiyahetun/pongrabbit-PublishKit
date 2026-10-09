<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import type { Project } from "@publishkit/shared";

const props = defineProps<{
  startDate: string;
  endDate: string;
}>();

const emit = defineEmits<{
  close: [];
  exported: [count: number];
}>();

const STATUS_KEYS = ["ready", "scheduled", "published", "blocked", "draft", "archived"] as const;
const DEFAULT_STATUSES = ["ready", "scheduled", "published"];

const { t } = useI18n();
const start = ref(props.startDate);
const end = ref(props.endDate);
const projects = ref<Project[]>([]);
const allProjects = ref(true);
const selected = ref<Set<string>>(new Set());
const statuses = ref<Set<string>>(new Set(DEFAULT_STATUSES));
const loading = ref(false);
const saving = ref(false);
const error = ref("");

const rangeInvalid = computed(() => !start.value || !end.value || start.value > end.value);
const canExport = computed(
  () => !rangeInvalid.value && statuses.value.size > 0 && (allProjects.value || selected.value.size > 0) && !saving.value,
);

function toggleStatus(status: string) {
  const next = new Set(statuses.value);
  if (next.has(status)) next.delete(status);
  else next.add(status);
  statuses.value = next;
}

function onAllChange(event: Event) {
  const checked = (event.target as HTMLInputElement).checked;
  allProjects.value = checked;
  if (checked) selected.value = new Set();
}

function toggleProject(id: string) {
  const next = new Set(selected.value);
  if (next.has(id)) next.delete(id);
  else next.add(id);
  selected.value = next;
  allProjects.value = next.size === 0;
}

function fileName() {
  if (allProjects.value || selected.value.size !== 1) {
    return selected.value.size > 1
      ? t("calendar.exportFileSome", { start: start.value, end: end.value })
      : t("calendar.exportFileAll", { start: start.value, end: end.value });
  }
  const id = [...selected.value][0];
  const project = projects.value.find((item) => item.id === id);
  const name = (project?.name || "project").replace(/[\\/:*?"<>|]/g, " ").trim().slice(0, 40) || "project";
  return t("calendar.exportFileOne", { name, start: start.value, end: end.value });
}

function labels() {
  return {
    sheet: t("calendar.exportSheet"),
    date: t("calendar.colDate"),
    project: t("calendar.colProject"),
    channel: t("calendar.colChannel"),
    status: t("calendar.colStatus"),
    language: t("calendar.colLanguage"),
    title: t("calendar.colTitle"),
    body: t("calendar.colBody"),
    url: t("calendar.colUrl"),
    note: t("calendar.colNote"),
    statusDraft: t("tasks.status_draft"),
    statusReady: t("tasks.status_ready"),
    statusScheduled: t("tasks.status_scheduled"),
    statusPublished: t("tasks.status_published"),
    statusArchived: t("tasks.status_archived"),
    statusBlocked: t("tasks.status_blocked"),
    langZh: t("calendar.langZh"),
    langEn: t("calendar.langEn"),
    langBilingual: t("calendar.langBilingual"),
  };
}

async function confirmExport() {
  if (!canExport.value) return;
  error.value = "";
  const picked = await save({
    defaultPath: fileName(),
    filters: [{ name: "Excel", extensions: ["xlsx"] }],
    title: t("calendar.exportTitle"),
  });
  if (!picked || typeof picked !== "string") return;
  saving.value = true;
  try {
    const count = await invoke<number>("export_calendar_ledger_cmd", {
      destPath: picked,
      startDate: start.value,
      endDate: end.value,
      projectIds: allProjects.value ? [] : [...selected.value],
      statuses: [...statuses.value],
      labels: labels(),
    });
    emit("exported", count);
  } catch (e) {
    error.value = String(e);
  } finally {
    saving.value = false;
  }
}

onMounted(async () => {
  loading.value = true;
  try {
    projects.value = await invoke<Project[]>("list_ledger_projects_cmd");
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <div class="overlay" @click.self="emit('close')">
    <section class="dialog" role="dialog" :aria-label="t('calendar.exportTitle')">
      <header class="head">
        <div>
          <h2>{{ t("calendar.exportTitle") }}</h2>
          <p>{{ t("calendar.exportHint") }}</p>
        </div>
        <button type="button" class="pk-btn pk-btn--ghost" @click="emit('close')">{{ t("common.close") }}</button>
      </header>

      <div class="dates">
        <label>
          <span>{{ t("calendar.exportFrom") }}</span>
          <input v-model="start" class="pk-input" type="date" />
        </label>
        <label>
          <span>{{ t("calendar.exportTo") }}</span>
          <input v-model="end" class="pk-input" type="date" />
        </label>
      </div>
      <p class="note">{{ t("calendar.exportDateHint") }}</p>
      <p v-if="rangeInvalid" class="warn">{{ t("calendar.exportRangeInvalid") }}</p>

      <div class="block">
        <span class="label">{{ t("calendar.exportProjects") }}</span>
        <label class="check">
          <input type="checkbox" :checked="allProjects" @change="onAllChange" />
          {{ t("calendar.exportAllProjects") }}
        </label>
        <p v-if="loading" class="note">{{ t("calendar.loading") }}</p>
        <div v-else class="project-list">
          <label v-for="project in projects" :key="project.id" class="check">
            <input
              type="checkbox"
              :checked="!allProjects && selected.has(project.id)"
              @change="toggleProject(project.id)"
            />
            <i class="dot" :style="{ background: project.color }" />
            {{ project.name }}
            <small v-if="project.archivedAt">{{ t("calendar.exportArchived") }}</small>
          </label>
        </div>
      </div>

      <div class="block">
        <span class="label">{{ t("calendar.exportStatuses") }}</span>
        <div class="status-row">
          <label v-for="status in STATUS_KEYS" :key="status" class="check">
            <input type="checkbox" :checked="statuses.has(status)" @change="toggleStatus(status)" />
            {{ t(`tasks.status_${status}`) }}
          </label>
        </div>
      </div>

      <p v-if="error" class="error">{{ error }}</p>
      <footer>
        <button type="button" class="pk-btn pk-btn--primary" :disabled="!canExport" @click="confirmExport">
          {{ saving ? t("calendar.exporting") : t("calendar.export") }}
        </button>
      </footer>
    </section>
  </div>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  z-index: 40;
  background: rgba(27, 28, 25, 0.28);
  display: flex;
  justify-content: center;
  padding: 32px 16px;
}
.dialog {
  width: min(520px, 100%);
  max-height: 100%;
  overflow: auto;
  background: var(--pk-bg-panel);
  border-radius: 12px;
  padding: 16px 18px 18px;
  box-shadow: var(--pk-shadow-lg);
}
.head,
.dates,
footer,
.check {
  display: flex;
  gap: 8px;
  align-items: center;
}
.head {
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 12px;
}
.head h2,
.head p,
.note,
.warn,
.error {
  margin: 0;
}
.head p,
.note {
  color: var(--pk-ink-muted);
  font-size: 12px;
}
.dates {
  gap: 16px;
  margin-bottom: 6px;
}
.dates label,
.block {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.dates span,
.label {
  font-size: 12px;
  color: var(--pk-ink-muted);
}
.note {
  margin-bottom: 12px;
}
.warn,
.error {
  font-size: 12px;
  margin-bottom: 8px;
}
.warn {
  color: var(--pk-warning, #9a6b2f);
}
.error {
  color: var(--pk-danger, #9b3a3a);
}
.block {
  margin-bottom: 14px;
}
.project-list {
  max-height: 168px;
  overflow: auto;
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.status-row {
  display: flex;
  flex-wrap: wrap;
  gap: 8px 14px;
}
.check {
  font-size: 13px;
}
.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}
.check small {
  color: var(--pk-ink-muted);
  font-size: 11px;
}
footer {
  justify-content: flex-end;
}
</style>
