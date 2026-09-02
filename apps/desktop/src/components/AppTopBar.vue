<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import type { WorkspaceSettings } from "@publishkit/shared";

const { t } = useI18n();
const projectName = ref("PublishKit");

onMounted(async () => {
  try {
    const settings = await invoke<WorkspaceSettings>("get_workspace_settings");
    projectName.value = settings.projectName?.trim() || "PublishKit";
  } catch {
    projectName.value = "PublishKit";
  }
});
</script>

<template>
  <header class="topbar">
    <span class="project">{{ projectName }}</span>
    <span class="hint">{{ t("app.topbarHint") }}</span>
  </header>
</template>

<style scoped>
.topbar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 20px;
  border-bottom: 1px solid var(--pk-border);
  background: var(--pk-bg-panel);
  min-height: 40px;
}
.project {
  font-weight: 600;
  font-size: 14px;
}
.hint {
  font-size: 12px;
  color: var(--pk-ink-muted);
}
</style>
