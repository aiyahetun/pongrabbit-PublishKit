<script setup lang="ts">
import type { PublishTask, TaskStatus } from "@publishkit/shared";
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import TaskCard from "./TaskCard.vue";

const props = defineProps<{
  tasks: PublishTask[];
}>();

const { t } = useI18n();

const columns: Array<{ key: TaskStatus; labelKey: string }> = [
  { key: "draft", labelKey: "tasks.status_draft" },
  { key: "ready", labelKey: "tasks.status_ready" },
  { key: "blocked", labelKey: "tasks.status_blocked" },
  { key: "published", labelKey: "tasks.status_published" },
];

const grouped = computed(() => {
  const map = new Map<TaskStatus, PublishTask[]>();
  for (const col of columns) map.set(col.key, []);
  for (const task of props.tasks) {
    const status = task.status as TaskStatus;
    if (map.has(status)) map.get(status)!.push(task);
  }
  return map;
});
</script>

<template>
  <div class="kanban">
    <section v-for="col in columns" :key="col.key" class="column">
      <header class="col-head">
        <span>{{ t(col.labelKey) }}</span>
        <span class="count">{{ grouped.get(col.key)?.length ?? 0 }}</span>
      </header>
      <ul class="col-list">
        <li v-for="task in grouped.get(col.key)" :key="task.id">
          <TaskCard :task="task">
            <slot :task="task" />
          </TaskCard>
        </li>
      </ul>
      <p v-if="!(grouped.get(col.key)?.length)" class="empty">{{ t("tasks.kanbanEmpty") }}</p>
    </section>
  </div>
</template>

<style scoped>
.kanban {
  display: grid;
  grid-template-columns: repeat(4, minmax(220px, 1fr));
  gap: 12px;
  align-items: start;
  overflow-x: auto;
}
.column {
  min-width: 220px;
  background: var(--pk-bg-alt);
  border: 1px solid var(--pk-border);
  border-radius: var(--pk-radius-md);
  padding: 10px;
  min-height: 120px;
}
.col-head {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
  font-weight: 600;
  margin-bottom: 10px;
}
.count {
  color: var(--pk-ink-muted);
  font-weight: 500;
}
.col-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.empty {
  margin: 0;
  font-size: 12px;
  color: var(--pk-ink-muted);
}
</style>
