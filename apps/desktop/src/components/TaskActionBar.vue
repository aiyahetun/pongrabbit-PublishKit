<script setup lang="ts">
import type { PublishTask } from "@publishkit/shared";
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import FieldCopyBar from "./FieldCopyBar.vue";

const props = defineProps<{
  task: PublishTask;
  copied?: boolean;
  publishUrl: string;
  scheduledDate?: string;
  publishedDate?: string;
  taskNote: string;
  blockedReasonInput: string;
  overdue?: boolean;
}>();

const emit = defineEmits<{
  copy: [];
  copySingleImage: [];
  linkMedia: [];
  openLinkedImagesFolder: [];
  exportPack: [];
  markReady: [];
  markPublished: [];
  backfillPublish: [];
  undoPublish: [];
  markBlocked: [];
  unblock: [];
  saveNote: [];
  saveScheduled: [];
  saveChecklist: [items: string[]];
  archive: [];
  "update:publishUrl": [value: string];
  "update:scheduledDate": [value: string];
  "update:publishedDate": [value: string];
  "update:taskNote": [value: string];
  "update:blockedReasonInput": [value: string];
}>();

const { t } = useI18n();

const checklistOptions = ["missingMedia", "compliance", "utm"] as const;
const checklist = ref<string[]>([...(props.task.checklist ?? [])]);

watch(
  () => props.task.checklist,
  (value) => {
    checklist.value = [...(value ?? [])];
  }
);

function toggleChecklist(key: string) {
  const set = new Set(checklist.value);
  if (set.has(key)) set.delete(key);
  else set.add(key);
  checklist.value = [...set];
}

const blockedPresets = [
  "tasks.blockedReason_missingMedia",
  "tasks.blockedReason_compliance",
  "tasks.blockedReason_pendingReview",
] as const;

const isBlocked = computed(() => props.task.status === "blocked");
const canMarkBlocked = computed(
  () => !props.overdue && (props.task.status === "draft" || props.task.status === "ready")
);

const showUrlField = computed(
  () =>
    !props.overdue &&
    (props.task.status === "ready" || props.task.status === "published")
);

const showScheduleField = computed(
  () =>
    !props.overdue &&
    (props.task.status === "draft" || props.task.status === "ready")
);

function applyBlockedPreset(key: (typeof blockedPresets)[number]) {
  emit("update:blockedReasonInput", t(key));
}
</script>

<template>
  <div class="action-bar">
    <div class="btn-row">
      <FieldCopyBar
        :title="task.content.title"
        :body="task.content.body"
        :keywords="task.content.keywords"
        :channel-id="task.channel.id"
      />

      <button type="button" class="pk-btn pk-btn--ghost" @click="emit('linkMedia')">
        {{ t("content.linkMedia") }}
      </button>

      <button type="button" class="pk-btn pk-btn--ghost" @click="emit('copySingleImage')">
        {{ t("media.copyImage") }}
      </button>

      <button type="button" class="pk-btn pk-btn--ghost" @click="emit('openLinkedImagesFolder')">
        {{ t("media.openLinkedImagesFolder") }}
      </button>

      <button type="button" class="pk-btn pk-btn--ghost" @click="emit('exportPack')">
        {{ t("tasks.exportChannelPack") }}
      </button>

      <template v-if="!overdue">
        <button
          v-if="task.status === 'draft'"
          type="button"
          class="pk-btn pk-btn--secondary"
          @click="emit('markReady')"
        >
          {{ t("tasks.markReady") }}
        </button>

        <button
          v-if="task.status === 'ready'"
          type="button"
          class="pk-btn pk-btn--success"
          @click="emit('markPublished')"
        >
          {{ t("tasks.markPublished") }}
        </button>

        <template v-if="task.status === 'published'">
          <button type="button" class="pk-btn pk-btn--secondary" @click="emit('markPublished')">
            {{ t("tasks.saveUrl") }}
          </button>
          <button type="button" class="pk-btn pk-btn--ghost" @click="emit('undoPublish')">
            {{ t("tasks.undoPublish") }}
          </button>
        </template>

        <button
          v-if="isBlocked"
          type="button"
          class="pk-btn pk-btn--secondary"
          @click="emit('unblock')"
        >
          {{ t("tasks.unblock") }}
        </button>
      </template>
    </div>

    <div v-if="overdue" class="overdue-panel">
      <p class="overdue-title">{{ t("tasks.overdueActionsTitle") }}</p>

      <div class="overdue-action">
        <span class="field-label">{{ t("tasks.backfillPublish") }}</span>
        <label class="inline-field">
          <span class="inline-label">{{ t("tasks.publishedDateLabel") }}</span>
          <input
            class="pk-input schedule-input"
            type="date"
            :value="publishedDate ?? ''"
            @input="emit('update:publishedDate', ($event.target as HTMLInputElement).value)"
          />
        </label>
        <label class="inline-field">
          <span class="inline-label">{{ t("tasks.publishUrlLabel") }}</span>
          <input
            class="pk-input url-input"
            :value="publishUrl"
            :placeholder="t('tasks.publishUrlPlaceholder')"
            @input="emit('update:publishUrl', ($event.target as HTMLInputElement).value)"
          />
        </label>
        <span class="hint">{{ t("tasks.publishedDateHint") }}</span>
        <button type="button" class="pk-btn pk-btn--success" @click="emit('backfillPublish')">
          {{ t("tasks.backfillPublish") }}
        </button>
      </div>

      <div class="overdue-action">
        <span class="field-label">{{ t("tasks.reschedulePublish") }}</span>
        <div class="schedule-inputs">
          <input
            class="pk-input schedule-input"
            type="date"
            :value="scheduledDate ?? ''"
            @input="emit('update:scheduledDate', ($event.target as HTMLInputElement).value)"
          />
          <button type="button" class="pk-btn pk-btn--secondary" @click="emit('saveScheduled')">
            {{ t("tasks.confirmReschedule") }}
          </button>
        </div>
        <span class="hint">{{ t("tasks.rescheduleHint") }}</span>
      </div>

      <div class="overdue-action">
        <button type="button" class="pk-btn pk-btn--ghost archive-btn" @click="emit('archive')">
          {{ t("tasks.archive") }}
        </button>
        <span class="hint">{{ t("tasks.archiveHint") }}</span>
      </div>
    </div>

    <label v-if="showScheduleField" class="schedule-row">
      <span class="field-label">{{ t("tasks.scheduledDateLabel") }}</span>
      <div class="schedule-inputs">
        <input
          class="pk-input schedule-input"
          type="date"
          :value="scheduledDate ?? ''"
          @input="emit('update:scheduledDate', ($event.target as HTMLInputElement).value)"
        />
        <button type="button" class="pk-btn pk-btn--ghost" @click="emit('saveScheduled')">
          {{ t("tasks.saveScheduled") }}
        </button>
      </div>
      <span class="hint">{{ t("tasks.scheduledDateHint") }}</span>
    </label>

    <label v-if="showUrlField" class="url-row">
      <span class="field-label">{{ t("tasks.publishUrlLabel") }}</span>
      <input
        class="pk-input url-input"
        :value="publishUrl"
        :placeholder="t('tasks.publishUrlPlaceholder')"
        @input="emit('update:publishUrl', ($event.target as HTMLInputElement).value)"
      />
    </label>

    <a
      v-if="task.publishUrl"
      class="saved-url"
      :href="task.publishUrl"
      target="_blank"
      rel="noopener noreferrer"
    >
      {{ task.publishUrl }}
    </a>

    <label class="note-row">
      <span class="field-label">{{ t("tasks.noteLabel") }}</span>
      <textarea
        class="pk-input note-input"
        rows="2"
        :value="taskNote"
        :placeholder="t('tasks.notePlaceholder')"
        @input="emit('update:taskNote', ($event.target as HTMLTextAreaElement).value)"
      />
      <button type="button" class="pk-btn pk-btn--ghost note-save" @click="emit('saveNote')">
        {{ t("tasks.saveNote") }}
      </button>
    </label>

    <div class="checklist-row">
      <span class="field-label">{{ t("tasks.checklistLabel") }}</span>
      <div class="preset-row">
        <label v-for="key in checklistOptions" :key="key" class="check-item">
          <input type="checkbox" :checked="checklist.includes(key)" @change="toggleChecklist(key)" />
          {{ t(`tasks.checklist_${key}`) }}
        </label>
      </div>
      <button type="button" class="pk-btn pk-btn--ghost" @click="emit('saveChecklist', checklist)">
        {{ t("tasks.saveChecklist") }}
      </button>
    </div>

    <div v-if="canMarkBlocked" class="block-row">
      <span class="field-label">{{ t("tasks.blockedReasonLabel") }}</span>
      <div class="preset-row">
        <button
          v-for="key in blockedPresets"
          :key="key"
          type="button"
          class="pk-chip preset-chip"
          @click="applyBlockedPreset(key)"
        >
          {{ t(key) }}
        </button>
      </div>
      <input
        class="pk-input"
        :value="blockedReasonInput"
        :placeholder="t('tasks.blockedReasonPlaceholder')"
        @input="emit('update:blockedReasonInput', ($event.target as HTMLInputElement).value)"
      />
      <button type="button" class="pk-btn pk-btn--ghost block-btn" @click="emit('markBlocked')">
        {{ t("tasks.markBlocked") }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.action-bar {
  display: flex;
  flex-direction: column;
  gap: var(--pk-space-3);
  padding-top: var(--pk-space-3);
  border-top: 1px solid var(--pk-border);
  margin-top: var(--pk-space-3);
}

.btn-row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--pk-space-2);
}

.overdue-panel {
  display: flex;
  flex-direction: column;
  gap: var(--pk-space-3);
  padding: var(--pk-space-3);
  border: 1px solid color-mix(in srgb, var(--pk-status-blocked) 25%, var(--pk-border));
  border-radius: var(--pk-radius-md);
  background: color-mix(in srgb, var(--pk-status-blocked) 6%, var(--pk-bg-panel));
}

.overdue-title {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--pk-status-blocked);
}

.overdue-action {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding-top: var(--pk-space-2);
  border-top: 1px solid var(--pk-border);
}

.overdue-action:first-of-type {
  border-top: none;
  padding-top: 0;
}

.schedule-row,
.url-row,
.note-row,
.checklist-row,
.block-row {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.field-label {
  font-size: 12px;
  font-weight: 500;
  color: var(--pk-ink-muted);
}

.inline-field {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--pk-space-2);
}

.inline-label {
  font-size: 12px;
  color: var(--pk-ink-secondary);
  min-width: 88px;
}

.schedule-inputs {
  display: flex;
  flex-wrap: wrap;
  gap: var(--pk-space-2);
  align-items: center;
}

.schedule-input {
  width: 160px;
}

.hint {
  font-size: 11px;
  color: var(--pk-ink-muted);
}

.url-input,
.note-input {
  width: 100%;
  max-width: 480px;
}

.note-input {
  min-height: 56px;
  resize: vertical;
}

.note-save,
.block-btn,
.archive-btn {
  align-self: flex-start;
}

.archive-btn {
  color: var(--pk-status-blocked);
}

.preset-row {
  display: flex;
  flex-wrap: wrap;
  gap: var(--pk-space-2);
}

.preset-chip {
  font-size: 12px;
}

.check-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
}

.saved-url {
  font-size: 12px;
  color: var(--pk-status-published);
  word-break: break-all;
  text-decoration: none;
}

.saved-url:hover {
  text-decoration: underline;
}
</style>
