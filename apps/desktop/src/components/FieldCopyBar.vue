<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps<{
  title: string;
  body: string;
  keywords?: string[];
  channelId?: string;
  showWarnings?: boolean;
}>();

const { t } = useI18n();
const copied = ref("");
const error = ref("");

const words = computed(() => (props.keywords ?? []).map((word) => word.trim()).filter(Boolean));
const hashOn = ref(false);
const hashMenu = ref(false);
const keywordWrap = ref<HTMLElement | null>(null);
const keywordBtn = ref<HTMLButtonElement | null>(null);
const menuEl = ref<HTMLElement | null>(null);
const menuStyle = ref({ top: "0px", left: "0px" });

const HASH_CHANNELS = ["xhs", "douyin", "kuaishou", "channels", "bilibili", "instagram", "tiktok", "threads", "weibo", "toutiao"];

async function loadHash() {
  hashOn.value = props.channelId ? HASH_CHANNELS.includes(props.channelId) : false;
  if (!props.channelId) return;
  try {
    hashOn.value = await invoke<boolean>("channel_keyword_hash_cmd", { channelId: props.channelId });
  } catch {
    // Keep the platform default if the setting cannot be read.
  }
}

function closeHashMenu() {
  hashMenu.value = false;
}

function onPointerDown(event: PointerEvent) {
  const target = event.target as Node | null;
  if (!target) return;
  if (keywordWrap.value?.contains(target) || menuEl.value?.contains(target)) return;
  closeHashMenu();
}

function onKey(event: KeyboardEvent) {
  if (event.key === "Escape") closeHashMenu();
}

function bindMenuListeners(open: boolean) {
  if (open) {
    document.addEventListener("pointerdown", onPointerDown, true);
    window.addEventListener("keydown", onKey);
    window.addEventListener("resize", closeHashMenu);
  } else {
    document.removeEventListener("pointerdown", onPointerDown, true);
    window.removeEventListener("keydown", onKey);
    window.removeEventListener("resize", closeHashMenu);
  }
}

function openHashMenu() {
  const button = keywordBtn.value;
  if (!button) return;
  const rect = button.getBoundingClientRect();
  const menuHeight = 88;
  const top =
    rect.bottom + 4 + menuHeight > window.innerHeight
      ? Math.max(8, rect.top - menuHeight - 4)
      : rect.bottom + 4;
  const left = Math.min(rect.left, window.innerWidth - 200);
  menuStyle.value = { top: `${top}px`, left: `${Math.max(8, left)}px` };
  hashMenu.value = true;
}

function onKeywordsClick() {
  if (!words.value.length) return;
  if (hashMenu.value) {
    closeHashMenu();
    return;
  }
  openHashMenu();
}

async function chooseHash(withHash: boolean) {
  closeHashMenu();
  let persistError = "";
  if (props.channelId) {
    try {
      await invoke("set_channel_keyword_hash_cmd", { channelId: props.channelId, enabled: withHash });
    } catch (e) {
      persistError = String(e);
    }
  }
  hashOn.value = withHash;
  await copyField("keywords");
  if (persistError && !error.value) error.value = persistError;
}

async function copyField(field: "title" | "body" | "keywords" | "full") {
  error.value = "";
  try {
    await invoke("copy_publish_field_cmd", {
      field,
      title: props.title,
      body: props.body,
      keywords: words.value,
      channelId: props.channelId || null,
      hash: hashOn.value,
    });
    copied.value = field;
    setTimeout(() => {
      if (copied.value === field) copied.value = "";
    }, 1200);
  } catch (e) {
    error.value = String(e);
  }
}

watch(() => props.channelId, () => {
  closeHashMenu();
  void loadHash();
});
watch(hashMenu, bindMenuListeners);
onMounted(() => {
  void loadHash();
});
onBeforeUnmount(() => bindMenuListeners(false));
</script>

<template>
  <div class="copy-bar">
    <button type="button" class="pk-btn pk-btn--ghost" @click="copyField('title')">
      {{ copied === "title" ? t("content.copied") : t("content.copyTitle") }}
    </button>
    <button type="button" class="pk-btn pk-btn--primary" @click="copyField('body')">
      {{ copied === "body" ? t("content.copied") : t("content.copyBody") }}
    </button>
    <span ref="keywordWrap" class="hash-anchor">
      <button
        ref="keywordBtn"
        type="button"
        class="pk-btn pk-btn--ghost"
        :disabled="!words.length"
        :aria-expanded="hashMenu"
        aria-haspopup="menu"
        @click="onKeywordsClick"
      >
        {{ copied === "keywords" ? t("content.copied") : t("content.copyKeywords") }}
      </button>
    </span>
    <button type="button" class="pk-btn pk-btn--ghost" @click="copyField('full')">
      {{ copied === "full" ? t("content.copied") : t("content.copyFull") }}
    </button>
    <Teleport to="body">
      <div v-if="hashMenu" ref="menuEl" class="hash-menu" role="menu" :style="menuStyle" :aria-label="t('content.hashChoose')">
        <button type="button" role="menuitem" :class="{ 'is-default': !hashOn }" @click="chooseHash(false)">
          <span>{{ t("content.hashPlain") }}</span>
          <small v-if="channelId && !hashOn">{{ t("content.hashDefault") }}</small>
        </button>
        <button type="button" role="menuitem" :class="{ 'is-default': hashOn }" @click="chooseHash(true)">
          <span>{{ t("content.hashMarked") }}</span>
          <small v-if="channelId && hashOn">{{ t("content.hashDefault") }}</small>
        </button>
      </div>
    </Teleport>
    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="showWarnings && title.length > 20" class="warn">{{ t("content.warnXhsTitle", { count: title.length }) }}</p>
    <p v-if="showWarnings && title.length > 100" class="warn">{{ t("content.warnPinTitle", { count: title.length }) }}</p>
    <p v-if="showWarnings && body.length > 500" class="warn">{{ t("content.warnPinBody", { count: body.length }) }}</p>
  </div>
</template>

<style scoped>
.copy-bar {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  align-items: center;
}
.warn,
.error {
  flex-basis: 100%;
  margin: 0;
  font-size: 12px;
}
.warn {
  color: var(--pk-warning, #9a6b2f);
}
.error {
  color: var(--pk-danger, #9b3a3a);
}
.hash-anchor {
  display: inline-flex;
}
.hash-menu {
  position: fixed;
  z-index: 80;
  min-width: 188px;
  padding: 4px;
  background: var(--pk-bg-panel);
  border: 1px solid var(--pk-border-strong);
  border-radius: 8px;
  box-shadow: var(--pk-shadow-lg);
}
.hash-menu button {
  display: flex;
  width: 100%;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  border: 0;
  background: transparent;
  color: var(--pk-ink);
  border-radius: 6px;
  padding: 6px 8px;
  font-size: 12px;
  cursor: pointer;
}
.hash-menu button.is-default {
  background: var(--pk-bg-alt);
}
.hash-menu small {
  color: var(--pk-ink-muted);
  font-size: 11px;
}
</style>
