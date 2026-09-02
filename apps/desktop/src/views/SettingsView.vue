<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { ask, open, save } from "@tauri-apps/plugin-dialog";
import type { UiLocale, Channel, ExportBackupResult, ImportBackupResult, ApiStatus, WorkspaceSettings, LicenseStatus } from "@publishkit/shared";

const { t, locale } = useI18n();
const saving = ref(false);
const backingUp = ref(false);
const restoring = ref(false);
const includeThumbs = ref(false);
const restoreMode = ref<"replace" | "merge">("replace");
const backupNotice = ref("");
const apiStatus = ref<ApiStatus | null>(null);
const apiLoading = ref(false);
const channels = ref<Channel[]>([]);
const newChannelName = ref("");
const newChannelMarket = ref<"domestic" | "overseas" | "both">("both");
const channelError = ref("");
const projectName = ref("");
const brandDomestic = ref("");
const brandOverseas = ref("");
const scanIgnoreText = ref("");
const videoRoot = ref("");
const licenseKey = ref("");
const licenseStatus = ref<LicenseStatus | null>(null);

const domesticChannels = computed(() =>
  channels.value.filter((c) => c.market === "domestic" || c.market === "both")
);
const overseasChannels = computed(() =>
  channels.value.filter((c) => c.market === "overseas" || c.market === "both")
);
const customChannels = computed(() => channels.value.filter((c) => c.isCustom));

async function setLocale(next: UiLocale) {
  saving.value = true;
  try {
    await invoke("set_ui_locale", { locale: next });
    locale.value = next;
  } finally {
    saving.value = false;
  }
}

async function loadChannels() {
  channels.value = await invoke<Channel[]>("list_channels_cmd");
}

async function addChannel() {
  channelError.value = "";
  try {
    await invoke("create_channel_cmd", {
      name: newChannelName.value,
      market: newChannelMarket.value,
    });
    newChannelName.value = "";
    await loadChannels();
  } catch (e) {
    channelError.value = String(e);
  }
}

async function removeChannel(channel: Channel) {
  if (!channel.isCustom) return;
  channelError.value = "";
  try {
    await invoke("delete_channel_cmd", { channelId: channel.id });
    await loadChannels();
  } catch (e) {
    channelError.value = String(e);
  }
}

function marketLabel(market: string) {
  return t(`channels.market_${market}`, market);
}

async function exportBackup() {
  backupNotice.value = "";
  channelError.value = "";
  const stamp = new Date().toISOString().slice(0, 10).replace(/-/g, "");
  const picked = await save({
    defaultPath: `publishkit-backup-${stamp}.zip`,
    filters: [{ name: "ZIP", extensions: ["zip"] }],
    title: t("settings.backupTitle"),
  });
  if (!picked || typeof picked !== "string") return;

  backingUp.value = true;
  try {
    const result = await invoke<ExportBackupResult>("export_backup_cmd", {
      destPath: picked,
      includeThumbs: includeThumbs.value,
    });
    backupNotice.value = t("settings.backupDone", {
      count: result.fileCount,
      thumbs: result.includesThumbs ? t("settings.backupWithThumbs") : t("settings.backupWithoutThumbs"),
    });
  } catch (e) {
    channelError.value = String(e);
  } finally {
    backingUp.value = false;
  }
}

async function loadApiStatus() {
  apiLoading.value = true;
  try {
    apiStatus.value = await invoke<ApiStatus>("get_api_status_cmd");
  } catch (e) {
    channelError.value = String(e);
  } finally {
    apiLoading.value = false;
  }
}

async function regeneratePairingToken() {
  channelError.value = "";
  backupNotice.value = "";
  apiLoading.value = true;
  try {
    apiStatus.value = await invoke<ApiStatus>("regenerate_pairing_token_cmd");
    backupNotice.value = t("settings.extensionTokenRotated");
  } catch (e) {
    channelError.value = String(e);
  } finally {
    apiLoading.value = false;
  }
}

async function importBackup() {
  backupNotice.value = "";
  channelError.value = "";
  const picked = await open({
    filters: [{ name: "ZIP", extensions: ["zip"] }],
    title: t("settings.restoreTitle"),
    multiple: false,
  });
  if (!picked || typeof picked !== "string") return;

  const confirmKey =
    restoreMode.value === "merge" ? "settings.restoreMergeConfirm" : "settings.restoreConfirm";
  const confirmed = await ask(t(confirmKey), {
    title: t("settings.restoreTitle"),
    kind: "warning",
  });
  if (!confirmed) return;

  restoring.value = true;
  try {
    const result = await invoke<ImportBackupResult>("import_backup_cmd", {
      zipPath: picked,
      mode: restoreMode.value,
    });
    if (restoreMode.value === "merge" && result.merged) {
      const m = result.merged;
      backupNotice.value = t("settings.restoreMergeDone", {
        content: m.contentItemsAdded,
        tasks: m.publishTasksAdded,
        media: m.mediaAssetsAdded,
      });
    }
  } catch (e) {
    channelError.value = String(e);
  } finally {
    restoring.value = false;
  }
}

async function loadWorkspace() {
  const settings = await invoke<WorkspaceSettings>("get_workspace_settings");
  projectName.value = settings.projectName ?? "";
  brandDomestic.value = settings.brandDomestic ?? "";
  brandOverseas.value = settings.brandOverseas ?? "";
  videoRoot.value = settings.videoRoot ?? "";
  scanIgnoreText.value = (settings.scanIgnoreDirs ?? []).join("\n");
}

async function loadLicense() {
  licenseStatus.value = await invoke<LicenseStatus>("get_license_status_cmd");
}

async function saveProject() {
  channelError.value = "";
  backupNotice.value = "";
  try {
    await invoke("set_workspace_profile_cmd", {
      projectName: projectName.value,
      brandDomestic: brandDomestic.value || null,
      brandOverseas: brandOverseas.value || null,
    });
    backupNotice.value = t("settings.projectSaved");
  } catch (e) {
    channelError.value = String(e);
  }
}

async function saveScanIgnore() {
  channelError.value = "";
  const dirs = scanIgnoreText.value
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);
  try {
    await invoke("set_scan_ignore_dirs_cmd", { dirs });
    backupNotice.value = t("settings.scanIgnoreSaved");
  } catch (e) {
    channelError.value = String(e);
  }
}

async function pickVideoRoot() {
  const picked = await open({ directory: true, multiple: false, title: t("settings.chooseVideoRoot") });
  if (picked && typeof picked === "string") {
    await invoke("set_video_root", { path: picked });
    videoRoot.value = picked;
  }
}

async function activateLicense() {
  channelError.value = "";
  try {
    licenseStatus.value = await invoke<LicenseStatus>("activate_license_cmd", {
      licenseKey: licenseKey.value,
    });
    backupNotice.value = t("settings.licenseActivated");
  } catch (e) {
    channelError.value = String(e);
  }
}

onMounted(async () => {
  await Promise.all([loadChannels(), loadApiStatus(), loadWorkspace(), loadLicense()]);
});
</script>

<template>
  <section class="page">
    <h1>{{ t("settings.title") }}</h1>

    <div class="card">
      <label>{{ t("settings.uiLocale") }}</label>
      <div class="row">
        <button
          type="button"
          class="pk-chip"
          :class="{ active: locale === 'zh-CN' }"
          :disabled="saving"
          @click="setLocale('zh-CN')"
        >
          {{ t("settings.localeZh") }}
        </button>
        <button
          type="button"
          class="pk-chip"
          :class="{ active: locale === 'en' }"
          :disabled="saving"
          @click="setLocale('en')"
        >
          {{ t("settings.localeEn") }}
        </button>
      </div>
    </div>

    <div class="card">
      <h2>{{ t("settings.backupSection") }}</h2>
      <p class="muted">{{ t("settings.backupHint") }}</p>
      <label class="check-row">
        <input v-model="includeThumbs" type="checkbox" />
        <span>{{ t("settings.backupIncludeThumbs") }}</span>
      </label>
      <div class="restore-mode">
        <span class="field-label">{{ t("settings.restoreMode") }}</span>
        <label class="mode-option">
          <input v-model="restoreMode" type="radio" value="replace" />
          {{ t("settings.restoreModeReplace") }}
        </label>
        <label class="mode-option">
          <input v-model="restoreMode" type="radio" value="merge" />
          {{ t("settings.restoreModeMerge") }}
        </label>
      </div>
      <button type="button" class="pk-btn pk-btn--secondary" :disabled="backingUp || restoring" @click="exportBackup">
        {{ backingUp ? t("settings.backingUp") : t("settings.exportBackup") }}
      </button>
      <button type="button" class="pk-btn pk-btn--ghost" :disabled="backingUp || restoring" @click="importBackup">
        {{ restoring ? t("settings.restoring") : t("settings.importBackup") }}
      </button>
      <p v-if="backupNotice" class="notice">{{ backupNotice }}</p>
    </div>

    <div class="card">
      <h2>{{ t("settings.extensionSection") }}</h2>
      <p class="muted">{{ t("settings.extensionHint") }}</p>
      <div v-if="apiLoading && !apiStatus" class="muted">{{ t("settings.extensionLoading") }}</div>
      <template v-else-if="apiStatus">
        <label>
          <span>{{ t("settings.extensionPort") }}</span>
          <input class="pk-input" :value="apiStatus.baseUrl" readonly />
        </label>
        <label>
          <span>{{ t("settings.extensionToken") }}</span>
          <input class="pk-input token" :value="apiStatus.pairingToken" readonly />
        </label>
        <div class="actions-inline">
          <button type="button" class="pk-btn pk-btn--ghost" :disabled="apiLoading" @click="regeneratePairingToken">
            {{ t("settings.extensionRotate") }}
          </button>
        </div>
      </template>
    </div>

    <div class="card">
      <h2>{{ t("settings.projectSection") }}</h2>
      <label>
        <span>{{ t("settings.projectName") }}</span>
        <input v-model="projectName" class="pk-input" type="text" />
      </label>
      <label>
        <span>{{ t("settings.brandDomestic") }}</span>
        <input v-model="brandDomestic" class="pk-input" type="text" />
      </label>
      <label>
        <span>{{ t("settings.brandOverseas") }}</span>
        <input v-model="brandOverseas" class="pk-input" type="text" />
      </label>
      <button type="button" class="pk-btn pk-btn--secondary" @click="saveProject">{{ t("settings.saveProject") }}</button>
    </div>

    <div class="card">
      <h2>{{ t("settings.scanIgnoreSection") }}</h2>
      <p class="muted">{{ t("settings.scanIgnoreHint") }}</p>
      <textarea v-model="scanIgnoreText" class="pk-input" rows="4" :placeholder="t('settings.scanIgnorePlaceholder')" />
      <label>
        <span>{{ t("settings.videoRoot") }}</span>
        <div class="row">
          <input v-model="videoRoot" class="pk-input" type="text" readonly />
          <button type="button" class="pk-btn pk-btn--ghost" @click="pickVideoRoot">{{ t("common.choose") }}</button>
        </div>
        <span class="muted">{{ t("settings.videoRootHint") }}</span>
      </label>
      <button type="button" class="pk-btn pk-btn--secondary" @click="saveScanIgnore">{{ t("settings.saveScanIgnore") }}</button>
    </div>

    <div class="card">
      <h2>{{ t("settings.licenseSection") }}</h2>
      <p v-if="licenseStatus" class="muted">
        {{ t("settings.licenseTier", { tier: licenseStatus.tier }) }} ·
        {{ t("settings.licenseUsage", { count: licenseStatus.contentCount, limit: licenseStatus.contentLimit }) }}
      </p>
      <label>
        <span>{{ t("settings.licenseKey") }}</span>
        <input v-model="licenseKey" class="pk-input" type="text" :placeholder="t('settings.licensePlaceholder')" />
      </label>
      <button type="button" class="pk-btn pk-btn--secondary" @click="activateLicense">{{ t("settings.activateLicense") }}</button>
    </div>

    <div class="card muted-card">
      <h2>{{ t("settings.syncSection") }}</h2>
      <p class="muted">{{ t("settings.syncHint") }}</p>
    </div>

    <div class="card wide">
      <h2>{{ t("channels.title") }}</h2>
      <p class="muted">{{ t("channels.subtitle") }}</p>

      <div class="group">
        <h3>{{ t("channels.domestic") }}</h3>
        <div class="channel-grid">
          <span v-for="ch in domesticChannels.filter((c) => !c.isCustom)" :key="ch.id" class="channel-tag">
            <span class="dot" :style="{ background: ch.color }" />
            {{ ch.name }}
          </span>
        </div>
      </div>

      <div class="group">
        <h3>{{ t("channels.overseas") }}</h3>
        <div class="channel-grid">
          <span v-for="ch in overseasChannels.filter((c) => !c.isCustom)" :key="ch.id" class="channel-tag">
            <span class="dot" :style="{ background: ch.color }" />
            {{ ch.name }}
          </span>
        </div>
      </div>

      <div v-if="customChannels.length" class="group">
        <h3>{{ t("channels.custom") }}</h3>
        <div class="channel-grid">
          <span v-for="ch in customChannels" :key="ch.id" class="channel-tag custom">
            <span class="dot" :style="{ background: ch.color }" />
            {{ ch.name }}
            <span class="market">{{ marketLabel(ch.market) }}</span>
            <button type="button" class="remove" :title="t('channels.remove')" @click="removeChannel(ch)">×</button>
          </span>
        </div>
      </div>

      <form class="add-form" @submit.prevent="addChannel">
        <input v-model="newChannelName" class="pk-input" :placeholder="t('channels.namePlaceholder')" required />
        <select v-model="newChannelMarket" class="pk-input select">
          <option value="domestic">{{ t("channels.market_domestic") }}</option>
          <option value="overseas">{{ t("channels.market_overseas") }}</option>
          <option value="both">{{ t("channels.market_both") }}</option>
        </select>
        <button type="submit" class="pk-btn pk-btn--primary">{{ t("channels.add") }}</button>
      </form>
      <p v-if="channelError" class="error">{{ channelError }}</p>
    </div>
  </section>
</template>

<style scoped>
.page {
  padding: var(--pk-space-5);
  display: flex;
  flex-direction: column;
  gap: var(--pk-space-4);
}
.page > h1 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
}
.card {
  background: var(--pk-bg-panel);
  border: 1px solid var(--pk-border-strong);
  border-radius: var(--pk-radius-lg);
  padding: var(--pk-space-4);
  max-width: 480px;
  box-shadow: var(--pk-shadow-sm);
  display: flex;
  flex-direction: column;
  gap: var(--pk-space-2);
}
.card.wide {
  max-width: 720px;
}
.card h2 {
  margin: 0 0 4px;
  font-size: 16px;
  font-weight: 600;
}
.check-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: var(--pk-space-3) 0;
  font-size: 13px;
  color: var(--pk-ink-secondary);
}
.restore-mode {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: var(--pk-space-3);
}
.mode-option {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--pk-ink-secondary);
  cursor: pointer;
}
.actions-inline {
  display: flex;
  gap: var(--pk-space-2);
}
.token {
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  font-size: 12px;
}
.card .field-label {
  display: block;
  font-size: 12px;
  color: var(--pk-ink-muted);
  margin-bottom: 6px;
}
.card .pk-input {
  width: 100%;
}
.notice {
  margin: var(--pk-space-2) 0 0;
  color: var(--pk-accent);
  font-size: 13px;
}
label {
  display: block;
  font-size: 12px;
  color: var(--pk-ink-muted);
  margin-bottom: var(--pk-space-3);
}
.row {
  display: flex;
  gap: var(--pk-space-2);
}
.group {
  margin-top: var(--pk-space-4);
}
.group h3 {
  margin: 0 0 var(--pk-space-2);
  font-size: 13px;
  font-weight: 600;
  color: var(--pk-accent-text);
}
.channel-grid {
  display: flex;
  flex-wrap: wrap;
  gap: var(--pk-space-2);
}
.channel-tag {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  height: 32px;
  padding: 0 12px;
  border-radius: var(--pk-radius-pill);
  border: 1px solid var(--pk-border-strong);
  background: var(--pk-bg-app);
  font-size: 13px;
}
.channel-tag.custom {
  padding-right: 8px;
}
.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}
.market {
  font-size: 11px;
  color: var(--pk-ink-muted);
}
.remove {
  border: none;
  background: transparent;
  color: var(--pk-ink-muted);
  font-size: 16px;
  line-height: 1;
  padding: 0 4px;
}
.remove:hover {
  color: var(--pk-status-blocked);
}
.add-form {
  display: flex;
  flex-wrap: wrap;
  gap: var(--pk-space-2);
  margin-top: var(--pk-space-4);
  padding-top: var(--pk-space-4);
  border-top: 1px solid var(--pk-border);
}
.add-form .pk-input {
  flex: 1;
  min-width: 140px;
}
.select {
  max-width: 160px;
}
.muted {
  color: var(--pk-ink-muted);
  font-size: 13px;
  margin: 0;
}
.error {
  color: var(--pk-status-blocked);
  font-size: 13px;
  margin-top: var(--pk-space-2);
}
</style>
