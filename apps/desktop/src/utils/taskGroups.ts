import type { PublishTask, TaskStatus } from "@publishkit/shared";

export type TaskSection = {
  key: string;
  labelKey: string;
  labelParams?: Record<string, string | number>;
  tasks: PublishTask[];
  defaultCollapsed: boolean;
};

function todayKey(): string {
  const now = new Date();
  const y = now.getFullYear();
  const m = String(now.getMonth() + 1).padStart(2, "0");
  const d = String(now.getDate()).padStart(2, "0");
  return `${y}-${m}-${d}`;
}

function monthKey(iso: string): string {
  return iso.slice(0, 7);
}

function currentMonthKey(): string {
  return monthKey(`${todayKey()}T00:00:00.000Z`);
}

function localDateKey(iso: string): string {
  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) {
    return iso.slice(0, 10);
  }
  const y = date.getFullYear();
  const m = String(date.getMonth() + 1).padStart(2, "0");
  const d = String(date.getDate()).padStart(2, "0");
  return `${y}-${m}-${d}`;
}

export function taskEventDate(task: PublishTask): string {
  if (task.status === "published" && task.publishedAt) {
    return localDateKey(task.publishedAt);
  }
  if (task.scheduledAt) {
    return localDateKey(task.scheduledAt);
  }
  return localDateKey(task.updatedAt);
}

export function isTaskOverdue(task: PublishTask): boolean {
  if (task.status !== "ready") return false;
  return taskEventDate(task) < todayKey();
}

export function overdueMeta(task: PublishTask): { days: number; scheduledDate: string } | null {
  if (!isTaskOverdue(task)) return null;
  const scheduledDate = taskEventDate(task);
  const start = new Date(`${scheduledDate}T00:00:00`);
  const today = new Date(`${todayKey()}T00:00:00`);
  const days = Math.max(1, Math.round((today.getTime() - start.getTime()) / 86400000));
  return { days, scheduledDate };
}

function compareTaskDesc(a: PublishTask, b: PublishTask): number {
  const aTime = a.publishedAt || a.scheduledAt || a.updatedAt;
  const bTime = b.publishedAt || b.scheduledAt || b.updatedAt;
  return bTime.localeCompare(aTime);
}

function readySections(tasks: PublishTask[]): TaskSection[] {
  const today = todayKey();
  const todayTasks: PublishTask[] = [];
  const futureTasks: PublishTask[] = [];
  const overdueTasks: PublishTask[] = [];

  for (const task of tasks) {
    const date = taskEventDate(task);
    if (date === today) todayTasks.push(task);
    else if (date > today) futureTasks.push(task);
    else overdueTasks.push(task);
  }

  const sections: TaskSection[] = [];
  if (todayTasks.length) {
    sections.push({
      key: "ready_today",
      labelKey: "tasks.sectionReadyToday",
      tasks: todayTasks.sort(compareTaskDesc),
      defaultCollapsed: false,
    });
  }
  if (futureTasks.length) {
    sections.push({
      key: "ready_future",
      labelKey: "tasks.sectionReadyFuture",
      tasks: futureTasks.sort(compareTaskDesc),
      defaultCollapsed: false,
    });
  }
  if (overdueTasks.length) {
    sections.push({
      key: "ready_overdue",
      labelKey: "tasks.sectionReadyOverdue",
      labelParams: { count: overdueTasks.length },
      tasks: overdueTasks.sort(compareTaskDesc),
      defaultCollapsed: true,
    });
  }
  return sections;
}

function publishedSections(tasks: PublishTask[]): TaskSection[] {
  const groups = new Map<string, PublishTask[]>();
  for (const task of tasks) {
    const key = monthKey(task.publishedAt || task.updatedAt);
    const list = groups.get(key) ?? [];
    list.push(task);
    groups.set(key, list);
  }

  const current = currentMonthKey();
  return [...groups.entries()]
    .sort(([a], [b]) => b.localeCompare(a))
    .map(([key, list]) => ({
      key: `published_${key}`,
      labelKey: "tasks.sectionPublishedMonth",
      labelParams: { month: key, count: list.length },
      tasks: list.sort(compareTaskDesc),
      defaultCollapsed: key !== current,
    }));
}

function flatSection(key: string, labelKey: string, tasks: PublishTask[]): TaskSection[] {
  if (!tasks.length) return [];
  return [
    {
      key,
      labelKey,
      labelParams: { count: tasks.length },
      tasks: tasks.sort(compareTaskDesc),
      defaultCollapsed: false,
    },
  ];
}

export function buildTodayQueueSections(tasks: PublishTask[]): TaskSection[] {
  const readySections = buildTaskSections(tasks, "ready");
  const order = ["ready_overdue", "ready_today"];
  return order
    .map((key) => readySections.find((section) => section.key === key))
    .filter((section): section is TaskSection => !!section && section.tasks.length > 0)
    .map((section) => ({
      ...section,
      defaultCollapsed: false,
    }));
}

export function buildTaskSections(
  tasks: PublishTask[],
  filter: "all" | TaskStatus
): TaskSection[] {
  const byStatus = (status: TaskStatus) => tasks.filter((task) => task.status === status);

  if (filter === "draft") {
    return flatSection("draft", "tasks.sectionDraft", byStatus("draft"));
  }
  if (filter === "ready") {
    return readySections(byStatus("ready"));
  }
  if (filter === "published") {
    return publishedSections(byStatus("published"));
  }
  if (filter === "blocked") {
    return flatSection("blocked", "tasks.sectionBlocked", byStatus("blocked"));
  }
  if (filter === "archived") {
    return flatSection("archived", "tasks.sectionArchived", byStatus("archived"));
  }

  return [
    ...flatSection("draft", "tasks.sectionDraft", byStatus("draft")),
    ...readySections(byStatus("ready")),
    ...flatSection("blocked", "tasks.sectionBlocked", byStatus("blocked")),
    ...publishedSections(byStatus("published")),
    ...flatSection("archived", "tasks.sectionArchived", byStatus("archived")),
  ];
}
