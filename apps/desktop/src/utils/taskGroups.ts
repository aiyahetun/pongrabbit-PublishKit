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

export function taskEventDate(task: PublishTask): string {
  if (task.status === "published" && task.publishedAt) {
    return task.publishedAt.slice(0, 10);
  }
  if (task.scheduledAt) {
    return task.scheduledAt.slice(0, 10);
  }
  return task.updatedAt.slice(0, 10);
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

  return [
    ...flatSection("draft", "tasks.sectionDraft", byStatus("draft")),
    ...readySections(byStatus("ready")),
    ...flatSection("blocked", "tasks.sectionBlocked", byStatus("blocked")),
    ...publishedSections(byStatus("published")),
  ];
}
