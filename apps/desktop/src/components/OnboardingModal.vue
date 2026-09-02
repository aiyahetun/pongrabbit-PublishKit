<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { WorkspaceSettings } from "@publishkit/shared";

const emit = defineEmits<{ done: [] }>();
const { t } = useI18n();
const projectName = ref("PublishKit");
const copyRoot = ref("");
const mediaRoot = ref("");
const saving = ref(false);
const error = ref("");

onMounted(async () => {
  try {
    const settings = await invoke<WorkspaceSettings>("get_workspace_settings");
    projectName.value = settings.projectName?.trim() || "PublishKit";
    copyRoot.value = settings.copyRoot ?? "";
    mediaRoot.value = settings.mediaRoot ?? "";
  } catch {
    /* use defaults */
  }
});

async function pickCopyRoot() {
  const picked = await open({ directory: true, multiple: false, title: t("onboarding.copyRootTitle") });
  if (picked && typeof picked === "string") copyRoot.value = picked;
}

async function pickMediaRoot() {
  const picked = await open({ directory: true, multiple: false, title: t("onboarding.mediaRootTitle") });
  if (picked && typeof picked === "string") mediaRoot.value = picked;
}

async function finish() {
  saving.value = true;
  error.value = "";
  try {
    await invoke("set_workspace_profile_cmd", {
      projectName: projectName.value.trim() || "PublishKit",
      brandDomestic: null,
      brandOverseas: null,
    });
    if (copyRoot.value.trim()) {
      await invoke("set_copy_root", { path: copyRoot.value.trim() });
    }
    if (mediaRoot.value.trim()) {
      await invoke("set_media_root", { path: mediaRoot.value.trim() });
    }
    await invoke("complete_onboarding_cmd");
    emit("done");
  } catch (e) {
    error.value = String(e);
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <div class="onboarding-screen">
    <div class="onboarding-inner">
      <header class="hero">
        <img class="brand-logo" src="../assets/logo-128.png" alt="" width="72" height="72" />
        <p class="brand-name">{{ t("app.name") }}</p>
        <h1 class="hero-title">{{ t("onboarding.title") }}</h1>
        <p class="hero-sub">{{ t("onboarding.subtitle") }}</p>
      </header>

      <section class="panel pk-card pk-card--elevated" aria-labelledby="onboarding-form-title">
        <h2 id="onboarding-form-title" class="panel-title">{{ t("onboarding.workspaceSection") }}</h2>

        <label class="field">
          <span class="field-label">{{ t("onboarding.projectName") }}</span>
          <input v-model="projectName" class="pk-input" type="text" autocomplete="organization" />
        </label>

        <label class="field">
          <span class="field-label">{{ t("onboarding.copyRoot") }}</span>
          <div class="path-row">
            <input
              v-model="copyRoot"
              class="pk-input pk-input--path"
              type="text"
              readonly
              :placeholder="t('onboarding.copyRootPlaceholder')"
            />
            <button type="button" class="pk-btn pk-btn--secondary" @click="pickCopyRoot">
              {{ t("onboarding.browse") }}
            </button>
          </div>
        </label>

        <label class="field">
          <span class="field-label">{{ t("onboarding.mediaRoot") }}</span>
          <div class="path-row">
            <input
              v-model="mediaRoot"
              class="pk-input pk-input--path"
              type="text"
              readonly
              :placeholder="t('onboarding.mediaRootPlaceholder')"
            />
            <button type="button" class="pk-btn pk-btn--secondary" @click="pickMediaRoot">
              {{ t("onboarding.browse") }}
            </button>
          </div>
        </label>

        <p v-if="error" class="error" role="alert">{{ error }}</p>

        <div class="actions">
          <button type="button" class="pk-btn pk-btn--primary" :disabled="saving" @click="finish">
            {{ saving ? t("onboarding.saving") : t("onboarding.start") }}
          </button>
        </div>
      </section>

      <p class="footer-hint">{{ t("onboarding.footerHint") }}</p>
    </div>
  </div>
</template>

<style scoped>
.onboarding-screen {
  min-height: 100vh;
  background: var(--pk-bg-app);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--pk-space-6) var(--pk-space-5);
}

.onboarding-inner {
  width: min(520px, 100%);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--pk-space-5);
}

.hero {
  text-align: center;
  max-width: 440px;
}

.brand-logo {
  display: block;
  width: 72px;
  height: 72px;
  margin: 0 auto var(--pk-space-4);
  border-radius: var(--pk-radius-lg);
  object-fit: contain;
}

.brand-name {
  margin: 0 0 var(--pk-space-2);
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  color: var(--pk-accent-text);
}

.hero-title {
  margin: 0 0 var(--pk-space-2);
  font-size: 20px;
  font-weight: 600;
  line-height: 1.3;
  color: var(--pk-ink);
}

.hero-sub {
  margin: 0;
  font-size: 14px;
  line-height: 1.5;
  color: var(--pk-ink-secondary);
}

.panel {
  width: 100%;
  padding: var(--pk-space-5);
  display: flex;
  flex-direction: column;
  gap: var(--pk-space-4);
}

.panel-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--pk-ink);
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.field-label {
  font-size: 12px;
  font-weight: 500;
  color: var(--pk-accent-text);
}

.path-row {
  display: flex;
  gap: var(--pk-space-2);
  align-items: stretch;
}

.path-row .pk-input {
  flex: 1;
  min-width: 0;
}

.path-row .pk-btn {
  flex-shrink: 0;
}

.actions {
  display: flex;
  justify-content: flex-end;
  padding-top: var(--pk-space-1);
}

.error {
  margin: 0;
  font-size: 13px;
  color: var(--pk-status-blocked);
}

.footer-hint {
  margin: 0;
  font-size: 12px;
  line-height: 1.45;
  color: var(--pk-ink-muted);
  text-align: center;
  max-width: 400px;
}
</style>
