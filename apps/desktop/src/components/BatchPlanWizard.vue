<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import type { Channel, MediaAsset } from "@publishkit/shared";
import MediaThumb from "./MediaThumb.vue";

type PreviewImage = {
  id: string;
  fileName: string;
  path: string;
  included: boolean;
  thumbPath?: string;
};

type PreviewItem = {
  id: string;
  sourcePath: string;
  sourceName: string;
  title: string;
  body: string;
  bodyPreview: string;
  keywords: string[];
  language: string;
  pairKey?: string;
  fileDate?: string;
  channelName?: string;
  images: PreviewImage[];
  missingImages: string[];
  imagesAmbiguous: boolean;
};

type Row = PreviewItem & { included: boolean };

const emit = defineEmits<{ close: []; done: [message: string] }>();
const { t } = useI18n();

const rows = ref<Row[]>([]);
const errors = ref<string[]>([]);
const channels = ref<Channel[]>([]);
const selected = ref<Set<string>>(new Set());
const startDate = ref(tomorrow());
const intervalDays = ref(1);
const loading = ref(false);
const saving = ref(false);
const error = ref("");
const newChannelName = ref("");
const newChannelMarket = ref<"domestic" | "overseas" | "both">("both");
const addingChannel = ref(false);

const planned = computed(() => {
  const interval = Number.isFinite(intervalDays.value) ? Math.max(0, intervalDays.value) : 1;
  const undatedIndex = new Map<string, number>();
  let cursor = 0;
  for (const row of rows.value) {
    if (row.included && !row.fileDate) {
      undatedIndex.set(row.id, cursor);
      cursor += 1;
    }
  }
  return rows.value.map((row) => {
    const index = undatedIndex.get(row.id);
    const generated = index === undefined ? "" : addDays(startDate.value, index * interval);
    return {
      row,
      date: row.included ? row.fileDate || generated : "",
      channels: row.included ? channelsFor(row) : [],
    };
  });
});

const taskCount = computed(() => planned.value.reduce((sum, item) => sum + item.channels.length, 0));

function tomorrow() {
  const date = new Date();
  date.setDate(date.getDate() + 1);
  return formatDate(date);
}

function formatDate(date: Date) {
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${date.getFullYear()}-${month}-${day}`;
}

function addDays(iso: string, days: number) {
  const [year, month, day] = iso.split("-").map(Number);
  if (!year || !month || !day) return "";
  const date = new Date(year, month - 1, day);
  date.setDate(date.getDate() + days);
  return formatDate(date);
}

function channelsFor(row: Row) {
  if (row.channelName) {
    const parts = row.channelName
      .split(/[、,，;；/|]/)
      .map((part) => part.trim().toLowerCase())
      .filter((part) => part.length >= 2);
    const hits = channels.value.filter((channel) =>
      parts.some((part) => {
        const name = channel.name.toLowerCase();
        const id = channel.id.toLowerCase();
        if (name === part || id === part) return true;
        return (name.length >= 2 && (name.includes(part) || part.includes(name)))
          || (id.length >= 2 && (id.includes(part) || part.includes(id)));
      })
    );
    if (hits.length) return hits;
  }
  return channels.value.filter((channel) => selected.value.has(channel.id));
}

function toggleChannel(id: string) {
  const next = new Set(selected.value);
  if (next.has(id)) next.delete(id);
  else next.add(id);
  selected.value = next;
}

function selectChannel(id: string) {
  const next = new Set(selected.value);
  next.add(id);
  selected.value = next;
}

async function addChannel() {
  const name = newChannelName.value.trim();
  if (!name || addingChannel.value) return;
  error.value = "";
  const existing = channels.value.find(
    (channel) => channel.name.trim().toLowerCase() === name.toLowerCase()
  );
  if (existing) {
    selectChannel(existing.id);
    newChannelName.value = "";
    return;
  }
  addingChannel.value = true;
  try {
    const created = await invoke<Channel>("create_channel_cmd", {
      name,
      market: newChannelMarket.value,
    });
    channels.value = [...channels.value, created];
    selectChannel(created.id);
    newChannelName.value = "";
  } catch (e) {
    error.value = String(e);
  } finally {
    addingChannel.value = false;
  }
}

async function downloadTemplate(kind: "markdown" | "sheet") {
  error.value = "";
  const picked = await save({
    defaultPath: kind === "markdown" ? t("batch.templateArticleName") : t("batch.templateSheetName"),
    filters:
      kind === "markdown"
        ? [{ name: "Markdown", extensions: ["md"] }]
        : [{ name: "Excel", extensions: ["xlsx"] }],
    title: kind === "markdown" ? t("batch.downloadArticle") : t("batch.downloadSheet"),
  });
  if (!picked || typeof picked !== "string") return;
  try {
    await invoke("save_import_template_cmd", { kind, destPath: picked });
  } catch (e) {
    error.value = String(e);
  }
}

async function pickFiles() {
  error.value = "";
  const picked = await open({
    multiple: true,
    title: t("batch.pick"),
    filters: [
      {
        name: t("batch.formats"),
        extensions: ["md", "markdown", "txt", "docx", "pdf", "html", "htm", "xlsx", "xls", "csv"],
      },
    ],
  });
  if (!picked) return;
  const paths = Array.isArray(picked) ? picked : [picked];
  loading.value = true;
  try {
    channels.value = await invoke<Channel[]>("list_channels_cmd");
    const preview = await invoke<{ items: PreviewItem[]; errors: string[] }>("preview_batch_import_cmd", { paths });
    rows.value = preview.items.map((item) => ({
      ...item,
      included: true,
      images: item.images ?? [],
      missingImages: item.missingImages ?? [],
      imagesAmbiguous: Boolean(item.imagesAmbiguous),
    }));
    errors.value = preview.errors;
    if (!rows.value.length) error.value = preview.errors[0] || t("batch.empty");
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

async function commit() {
  if (!taskCount.value || saving.value) return;
  saving.value = true;
  error.value = "";
  try {
    const items = planned.value
      .filter((item) => item.date && item.channels.length)
      .map((item) => ({
        sourcePath: item.row.sourcePath,
        title: item.row.title,
        body: item.row.body,
        keywords: item.row.keywords,
        language: item.row.language,
        pairKey: item.row.pairKey ?? null,
        scheduledDate: item.date,
        channelIds: item.channels.map((channel) => channel.id),
        mediaIds: item.row.images.filter((image) => image.included).map((image) => image.id),
      }));
    const result = await invoke<{ contentCount: number; taskCount: number; skippedTasks: number }>(
      "commit_batch_import_cmd",
      { items }
    );
    emit(
      "done",
      t("batch.done", {
        content: result.contentCount,
        tasks: result.taskCount,
        skipped: result.skippedTasks,
      })
    );
  } catch (e) {
    error.value = String(e);
  } finally {
    saving.value = false;
  }
}

function thumbAsset(image: PreviewImage): MediaAsset {
  return {
    id: image.id,
    path: image.path,
    fileName: image.fileName,
    kind: "image",
    sizeBytes: 0,
    indexedAt: "",
    thumbPath: image.thumbPath,
  };
}

function nameList(names: string[]) {
  return names.join(t("batch.nameSep"));
}
</script>

<template>
  <div class="overlay" @click.self="emit('close')">
    <section class="dialog">
      <header class="head">
        <div>
          <h2>{{ t("batch.title") }}</h2>
          <p>{{ t("batch.subtitle") }}</p>
        </div>
        <button type="button" class="pk-btn pk-btn--ghost" @click="emit('close')">{{ t("common.close") }}</button>
      </header>

      <div v-if="!rows.length" class="pick">
        <p>{{ t("batch.formatsHint") }}</p>
        <p>{{ t("batch.templateHint") }}</p>
        <div class="template-actions">
          <button type="button" class="pk-btn pk-btn--ghost" @click="downloadTemplate('markdown')">
            {{ t("batch.downloadArticle") }}
          </button>
          <button type="button" class="pk-btn pk-btn--ghost" @click="downloadTemplate('sheet')">
            {{ t("batch.downloadSheet") }}
          </button>
        </div>
        <button type="button" class="pk-btn pk-btn--primary" :disabled="loading" @click="pickFiles">
          {{ loading ? t("batch.reading") : t("batch.pick") }}
        </button>
      </div>

      <div v-else class="plan">
        <div class="controls">
          <label>
            <span>{{ t("batch.start") }}</span>
            <input v-model="startDate" class="pk-input" type="date" />
          </label>
          <label>
            <span>{{ t("batch.interval") }}</span>
            <input v-model.number="intervalDays" class="pk-input" type="number" min="0" max="365" />
          </label>
          <p class="note">{{ t("batch.intervalHint") }}</p>
          <div class="channels">
            <span class="label">{{ t("batch.platforms") }}</span>
            <button
              v-for="channel in channels"
              :key="channel.id"
              type="button"
              class="chip"
              :class="{ on: selected.has(channel.id) }"
              @click="toggleChannel(channel.id)"
            >
              <i :style="{ background: channel.color }" />
              {{ channel.name }}
            </button>
            <form class="add-channel" @submit.prevent="addChannel">
              <p>{{ t("batch.addPlatformHint") }}</p>
              <input v-model="newChannelName" class="pk-input" :placeholder="t('channels.namePlaceholder')" />
              <select v-model="newChannelMarket" class="pk-input">
                <option value="domestic">{{ t("channels.market_domestic") }}</option>
                <option value="overseas">{{ t("channels.market_overseas") }}</option>
                <option value="both">{{ t("channels.market_both") }}</option>
              </select>
              <button type="submit" class="pk-btn pk-btn--ghost" :disabled="addingChannel || !newChannelName.trim()">
                {{ t("channels.add") }}
              </button>
            </form>
          </div>
        </div>

        <ul class="items">
          <li v-for="item in planned" :key="item.row.id" :class="{ off: !item.row.included }">
            <label class="check">
              <input
                type="checkbox"
                :checked="item.row.included"
                @change="item.row.included = ($event.target as HTMLInputElement).checked"
              />
            </label>
            <div class="meta">
              <strong>{{ item.row.title }}</strong>
              <span>{{ item.date }} · {{ item.channels.map((channel) => channel.name).join("、") || t("batch.noPlatform") }}</span>
              <span v-if="item.row.keywords.length">{{ item.row.keywords.join(" · ") }}</span>
              <p>{{ item.row.bodyPreview }}</p>
              <div v-if="item.row.images.length || item.row.missingImages.length" class="images">
                <span>{{ t("batch.images") }}</span>
                <label v-for="image in item.row.images" :key="image.id" class="image-pick">
                  <input v-model="image.included" type="checkbox" />
                  <MediaThumb :asset="thumbAsset(image)" size="sm" :preview="!image.thumbPath" />
                  <span>{{ image.fileName }}</span>
                </label>
                <p
                  v-if="item.row.imagesAmbiguous && !item.row.images.some((image) => image.included)"
                  class="image-note"
                >
                  {{ t("batch.pickImages") }}
                </p>
                <p v-if="item.row.missingImages.length" class="image-note">
                  {{ t("batch.missingImages", { names: nameList(item.row.missingImages) }) }}
                </p>
              </div>
              <small>{{ item.row.sourceName }}</small>
            </div>
          </li>
        </ul>

        <footer>
          <button type="button" class="pk-btn pk-btn--ghost" @click="pickFiles">{{ t("batch.pick") }}</button>
          <button type="button" class="pk-btn pk-btn--primary" :disabled="!taskCount || saving" @click="commit">
            {{ saving ? t("batch.saving") : t("batch.confirm", { count: taskCount }) }}
          </button>
        </footer>
      </div>

      <p v-for="line in errors" :key="line" class="warn">{{ line }}</p>
      <p v-if="error" class="error">{{ error }}</p>
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
  width: min(880px, 100%);
  max-height: 100%;
  overflow: auto;
  background: var(--pk-bg-panel);
  border-radius: 12px;
  padding: 16px 18px 18px;
  box-shadow: var(--pk-shadow-lg);
}
.head,
footer,
.controls,
.channels,
.meta span {
  display: flex;
  gap: 8px;
  align-items: center;
}
.head {
  justify-content: space-between;
  margin-bottom: 12px;
}
.head h2,
.meta p,
.note,
.pick p,
.warn,
.error {
  margin: 0;
}
.head p,
.note,
.meta span,
.meta small,
.pick p {
  color: var(--pk-ink-muted);
  font-size: 12px;
}
.pick {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 12px;
}
.template-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
.controls {
  flex-wrap: wrap;
  margin-bottom: 12px;
}
.controls label {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 12px;
  color: var(--pk-ink-secondary);
}
.note {
  flex-basis: 100%;
}
.channels {
  flex-basis: 100%;
  flex-wrap: wrap;
}
.chip {
  height: 26px;
  padding: 0 8px;
  border-radius: 999px;
  border: 1px solid var(--pk-border);
  background: transparent;
  color: var(--pk-ink-secondary);
  font-size: 12px;
  cursor: pointer;
}
.chip.on {
  background: var(--pk-accent-soft);
  border-color: var(--pk-accent);
  color: var(--pk-ink);
  font-weight: 600;
}
.chip i {
  display: inline-block;
  width: 6px;
  height: 6px;
  border-radius: 99px;
  margin-right: 4px;
}
.add-channel {
  flex-basis: 100%;
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
}
.add-channel p {
  flex-basis: 100%;
  margin: 0;
  color: var(--pk-ink-muted);
  font-size: 12px;
}
.add-channel .pk-input {
  width: 200px;
}
.add-channel select.pk-input {
  width: 128px;
}
.items {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.items li {
  display: flex;
  gap: 8px;
  padding: 8px;
  border: 1px solid var(--pk-border);
  border-radius: 8px;
}
.items li.off {
  opacity: 0.55;
}
.meta {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.meta strong {
  font-size: 13px;
}
.meta p {
  font-size: 12px;
  color: var(--pk-ink-secondary);
}
.images {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  margin-top: 4px;
}
.images > span,
.images p {
  flex-basis: 100%;
}
.image-pick {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--pk-ink-secondary);
}
.images .image-note {
  margin: 0;
  color: var(--pk-warning, #9a6b2f);
}
footer {
  justify-content: flex-end;
  margin-top: 12px;
}
.warn,
.error {
  margin-top: 8px;
  font-size: 12px;
}
.warn {
  color: var(--pk-warning, #9a6b2f);
}
.error {
  color: #9b3a3a;
}
</style>
