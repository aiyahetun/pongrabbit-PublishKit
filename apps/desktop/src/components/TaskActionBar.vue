<script setup lang="ts">
import type { PublishTask } from "@publishkit/shared";
import { computed } from "vue";
import { useI18n } from "vue-i18n";

const props = defineProps<{
  task: PublishTask;
  copied?: boolean;
  publishUrl: string;
}>();

const emit = defineEmits<{
  copy: [];
  markReady: [];
  markPublished: [];
  undoPublish: [];
  "update:publishUrl": [value: string];
}>();

const { t } = useI18n();

const showUrlField = computed(
  () => props.task.status === "ready" || props.task.status === "published"
);
</script>

<template>
  <div class="action-bar">
    <div class="btn-row">
      <button type="button" class="pk-btn pk-btn--ghost" @click="emit('copy')">
        {{ copied ? t("content.copied") : t("content.copyRich") }}
      </button>

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
        class="pk-btn pk-btn--primary"
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
    </div>

    <label v-if="showUrlField" class="url-row">
      <span class="url-label">{{ t("tasks.publishUrlLabel") }}</span>
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

.url-row {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.url-label {
  font-size: 12px;
  font-weight: 500;
  color: var(--pk-ink-muted);
}

.url-input {
  width: 100%;
  max-width: 480px;
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
