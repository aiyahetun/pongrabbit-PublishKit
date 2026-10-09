<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import type { Project, WorkspaceSettings } from "@publishkit/shared";

const { t } = useI18n();
const settings = ref<WorkspaceSettings | null>(null);
const projects = ref<Project[]>([]);
const error = ref("");

const current = computed(
  () => projects.value.find((item) => item.id === settings.value?.currentProjectId) ?? null
);

let reloadToken = 0;

async function reload() {
  const token = ++reloadToken;
  const nextSettings = await invoke<WorkspaceSettings>("get_workspace_settings");
  const nextProjects = await invoke<Project[]>("list_projects_cmd");
  if (token !== reloadToken) return;
  settings.value = nextSettings;
  projects.value = nextProjects;
}

async function choose(projectId: string) {
  if (projectId === settings.value?.currentProjectId) return;
  error.value = "";
  try {
    settings.value = await invoke<WorkspaceSettings>("set_current_project_cmd", { projectId });
    window.dispatchEvent(new CustomEvent("publishkit-project-changed"));
  } catch (e) {
    error.value = String(e);
  }
}

async function createProject() {
  const name = window.prompt(t("project.namePrompt"));
  if (!name?.trim()) return;
  error.value = "";
  try {
    settings.value = await invoke<WorkspaceSettings>("create_project_cmd", { name: name.trim() });
    await reload();
    window.dispatchEvent(new CustomEvent("publishkit-project-changed"));
  } catch (e) {
    error.value = String(e);
  }
}

async function renameCurrent() {
  const project = current.value;
  if (!project) return;
  const name = window.prompt(t("project.rename"), project.name);
  if (!name?.trim() || name.trim() === project.name) return;
  error.value = "";
  try {
    settings.value = await invoke<WorkspaceSettings>("update_project_cmd", {
      projectId: project.id,
      name: name.trim(),
    });
    await reload();
    window.dispatchEvent(new CustomEvent("publishkit-project-changed"));
  } catch (e) {
    error.value = String(e);
  }
}

async function archiveCurrent() {
  const project = current.value;
  if (!project) return;
  error.value = "";
  try {
    settings.value = await invoke<WorkspaceSettings>("archive_project_cmd", { projectId: project.id });
    await reload();
    window.dispatchEvent(new CustomEvent("publishkit-project-changed"));
  } catch (e) {
    error.value = String(e);
  }
}

function onProjectChanged() {
  void reload().catch((e) => {
    error.value = String(e);
  });
}

onMounted(() => {
  window.addEventListener("publishkit-project-changed", onProjectChanged);
  void reload().catch((e) => {
    error.value = String(e);
  });
});

onBeforeUnmount(() => {
  window.removeEventListener("publishkit-project-changed", onProjectChanged);
});
</script>

<template>
  <header class="topbar">
    <div class="row">
      <div class="tabs" role="tablist" :aria-label="t('project.switch')">
        <button
          v-for="project in projects"
          :key="project.id"
          type="button"
          class="tab"
          role="tab"
          :aria-selected="project.id === current?.id"
          :title="t('project.counts', { content: project.contentCount, media: project.mediaCount })"
          :style="{ '--tab-color': project.color }"
          @click="choose(project.id)"
        >
          <span class="dot" :style="{ background: project.color }" />
          <span class="name">{{ project.name }}</span>
          <span class="counts">{{ project.contentCount }}</span>
        </button>
        <button type="button" class="tab add" @click="createProject">{{ t("project.new") }}</button>
      </div>
      <div v-if="current" class="tab-actions">
        <button type="button" class="pk-btn pk-btn--ghost" @click="renameCurrent">{{ t("project.rename") }}</button>
        <button type="button" class="pk-btn pk-btn--ghost" @click="archiveCurrent">{{ t("project.archive") }}</button>
      </div>
    </div>
    <p class="hint">{{ t("project.tabHint") }}</p>
    <p v-if="error" class="error">{{ error }}</p>
  </header>
</template>

<style scoped>
.topbar {
  padding: 8px 16px 0;
  border-bottom: 1px solid var(--pk-border);
  background: var(--pk-bg-panel);
}
.row {
  display: flex;
  align-items: flex-end;
  gap: 12px;
  min-width: 0;
}
.tabs {
  display: flex;
  align-items: flex-end;
  gap: 4px;
  flex: 1;
  min-width: 0;
  overflow-x: auto;
}
.tab {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  max-width: 240px;
  padding: 8px 14px 10px;
  border: 1px solid transparent;
  border-bottom: none;
  border-radius: 8px 8px 0 0;
  background: transparent;
  color: var(--pk-ink-muted, #6f675d);
  font-size: 14px;
  white-space: nowrap;
  cursor: pointer;
}
.tab[aria-selected="true"] {
  background: var(--pk-bg-app);
  color: var(--pk-ink, #2c2824);
  font-weight: 600;
  border-color: var(--pk-border);
  box-shadow: inset 0 -3px 0 var(--tab-color, var(--pk-accent));
}
.name {
  overflow: hidden;
  text-overflow: ellipsis;
}
.counts {
  color: var(--pk-ink-muted, #6f675d);
  font-size: 12px;
  font-weight: 500;
}
.dot {
  width: 8px;
  height: 8px;
  border-radius: 99px;
  flex: none;
}
.add {
  color: var(--pk-accent-text);
  border: 1px dashed var(--pk-border-strong, var(--pk-border));
  border-bottom: none;
  margin-bottom: 0;
}
.tab-actions {
  display: flex;
  gap: 6px;
  padding-bottom: 6px;
  flex: none;
}
.hint,
.error {
  margin: 0;
  padding: 4px 2px 8px;
  font-size: 12px;
}
.hint {
  color: var(--pk-ink-muted, #6f675d);
}
.error {
  color: #9b3a3a;
}
</style>
