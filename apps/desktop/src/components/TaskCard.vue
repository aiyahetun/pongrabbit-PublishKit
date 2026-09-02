<script setup lang="ts">
import type { PublishTask } from "@publishkit/shared";
import { useI18n } from "vue-i18n";

defineProps<{
  task: PublishTask;
}>();

const { t } = useI18n();
</script>

<template>
  <article class="task-card">
    <div class="head">
      <span class="channel-dot" :style="{ background: task.channel.color }" />
      <div class="titles">
        <strong>{{ task.content.title }}</strong>
        <span class="sub">
          <span class="channel-name">{{ task.channel.name }}</span>
          <span class="sep">·</span>
          <span>{{ task.content.language }}</span>
        </span>
      </div>
      <span class="pk-status" :data-status="task.status">
        {{ t(`tasks.status_${task.status}`, task.status) }}
      </span>
    </div>
    <p v-if="task.status === 'blocked' && task.blockedReason" class="blocked-reason">
      {{ task.blockedReason }}
    </p>
    <p class="preview">{{ task.content.body.slice(0, 160) }}{{ task.content.body.length > 160 ? "…" : "" }}</p>
    <slot />
  </article>
</template>

<style scoped>
.task-card {
  border: 1px solid var(--pk-border-strong);
  border-radius: var(--pk-radius-lg);
  padding: var(--pk-space-4);
  background: var(--pk-bg-panel);
  box-shadow: var(--pk-shadow-sm);
}

.head {
  display: flex;
  gap: var(--pk-space-3);
  align-items: flex-start;
}

.channel-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  margin-top: 7px;
  flex-shrink: 0;
  box-shadow: var(--pk-shadow-sm);
}

.titles {
  flex: 1;
  min-width: 0;
}

.titles strong {
  display: block;
  margin-bottom: 4px;
  font-size: 15px;
  font-weight: 600;
  line-height: 1.35;
}

.sub {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  align-items: center;
  font-size: 12px;
  color: var(--pk-ink-muted);
}

.channel-name {
  font-weight: 500;
  color: var(--pk-ink-secondary);
}

.sep {
  opacity: 0.6;
}

.preview {
  margin: var(--pk-space-3) 0 0;
  font-size: 13px;
  line-height: 1.5;
  color: var(--pk-ink-secondary);
  white-space: pre-wrap;
}

.blocked-reason {
  margin: var(--pk-space-2) 0 0;
  font-size: 13px;
  line-height: 1.4;
  color: var(--pk-status-blocked);
  font-weight: 500;
}
</style>
