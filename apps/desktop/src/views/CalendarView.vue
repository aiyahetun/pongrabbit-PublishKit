<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import type { CalendarEntry } from "@publishkit/shared";

const { t, locale } = useI18n();
const today = new Date();
const todayIso = computed(() => {
  const now = new Date();
  return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, "0")}-${String(now.getDate()).padStart(2, "0")}`;
});
const viewYear = ref(today.getFullYear());
const viewMonth = ref(today.getMonth() + 1);
const calendarMode = ref<"month" | "week">("month");
const weekAnchor = ref(todayIso.value);
const entries = ref<CalendarEntry[]>([]);
const loading = ref(false);
const error = ref("");
const selectedDate = ref("");

const weekdayLabels = computed(() => {
  if (locale.value.startsWith("zh")) {
    return ["一", "二", "三", "四", "五", "六", "日"];
  }
  return ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
});

const monthLabel = computed(() => {
  if (locale.value.startsWith("zh")) {
    return `${viewYear.value} 年 ${viewMonth.value} 月`;
  }
  return new Date(viewYear.value, viewMonth.value - 1, 1).toLocaleDateString("en", {
    month: "long",
    year: "numeric",
  });
});

const calendarCells = computed(() => {
  const first = new Date(viewYear.value, viewMonth.value - 1, 1);
  const startOffset = (first.getDay() + 6) % 7;
  const daysInMonth = new Date(viewYear.value, viewMonth.value, 0).getDate();
  const cells: Array<{ date: string; day: number; inMonth: boolean }> = [];

  for (let i = 0; i < startOffset; i += 1) {
    cells.push({ date: "", day: 0, inMonth: false });
  }
  for (let day = 1; day <= daysInMonth; day += 1) {
    const date = `${viewYear.value}-${String(viewMonth.value).padStart(2, "0")}-${String(day).padStart(2, "0")}`;
    cells.push({ date, day, inMonth: true });
  }
  return cells;
});

const entriesByDate = computed(() => {
  const map = new Map<string, CalendarEntry[]>();
  for (const entry of entries.value) {
    const list = map.get(entry.date) ?? [];
    list.push(entry);
    map.set(entry.date, list);
  }
  return map;
});

const selectedEntries = computed(() => {
  if (!selectedDate.value) return [];
  return entriesByDate.value.get(selectedDate.value) ?? [];
});

const selectedDateLabel = computed(() => {
  if (!selectedDate.value) return "";
  const [year, month, day] = selectedDate.value.split("-").map(Number);
  if (locale.value.startsWith("zh")) {
    return `${year}年${month}月${day}日`;
  }
  return new Date(year, month - 1, day).toLocaleDateString("en", {
    weekday: "long",
    month: "long",
    day: "numeric",
    year: "numeric",
  });
});

function statusLabel(status: string) {
  return t(`tasks.status_${status}`);
}

function selectDate(date: string) {
  selectedDate.value = selectedDate.value === date ? "" : date;
}

async function loadEntries() {
  loading.value = true;
  error.value = "";
  try {
    if (calendarMode.value === "week") {
      entries.value = await invoke<CalendarEntry[]>("list_calendar_week_entries_cmd", {
        anchorDate: weekAnchor.value,
      });
    } else {
      entries.value = await invoke<CalendarEntry[]>("list_calendar_entries_cmd", {
        year: viewYear.value,
        month: viewMonth.value,
      });
    }
    if (selectedDate.value && !entriesByDate.value.has(selectedDate.value)) {
      selectedDate.value = "";
    }
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

function prevMonth() {
  if (viewMonth.value === 1) {
    viewMonth.value = 12;
    viewYear.value -= 1;
  } else {
    viewMonth.value -= 1;
  }
}

function nextMonth() {
  if (viewMonth.value === 12) {
    viewMonth.value = 1;
    viewYear.value += 1;
  } else {
    viewMonth.value += 1;
  }
}

function goToday() {
  viewYear.value = today.getFullYear();
  viewMonth.value = today.getMonth() + 1;
  selectedDate.value = todayIso.value;
}

function prevWeek() {
  const d = new Date(weekAnchor.value);
  d.setDate(d.getDate() - 7);
  weekAnchor.value = d.toISOString().slice(0, 10);
  void loadEntries();
}

function nextWeek() {
  const d = new Date(weekAnchor.value);
  d.setDate(d.getDate() + 7);
  weekAnchor.value = d.toISOString().slice(0, 10);
  void loadEntries();
}

watch([viewYear, viewMonth], () => {
  if (calendarMode.value === "month") void loadEntries();
});
watch(calendarMode, loadEntries);
onMounted(loadEntries);
</script>

<template>
  <section class="page">
    <header class="header">
      <div>
        <h1>{{ t("calendar.title") }}</h1>
        <p class="subtitle">{{ t("calendar.subtitle") }}</p>
      </div>
      <div class="nav">
        <button
          type="button"
          class="pk-chip"
          :class="{ active: calendarMode === 'month' }"
          @click="calendarMode = 'month'"
        >
          {{ t("calendar.viewMonth") }}
        </button>
        <button
          type="button"
          class="pk-chip"
          :class="{ active: calendarMode === 'week' }"
          @click="calendarMode = 'week'"
        >
          {{ t("calendar.viewWeek") }}
        </button>
        <template v-if="calendarMode === 'month'">
          <button type="button" class="pk-btn pk-btn--ghost" @click="prevMonth">‹</button>
          <span class="month">{{ monthLabel }}</span>
          <button type="button" class="pk-btn pk-btn--ghost" @click="nextMonth">›</button>
        </template>
        <template v-else>
          <button type="button" class="pk-btn pk-btn--ghost" @click="prevWeek">‹</button>
          <span class="month">{{ weekAnchor }}</span>
          <button type="button" class="pk-btn pk-btn--ghost" @click="nextWeek">›</button>
        </template>
        <button type="button" class="pk-btn pk-btn--secondary" @click="goToday">
          {{ t("calendar.today") }}
        </button>
      </div>
    </header>

    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="loading" class="muted">{{ t("calendar.loading") }}</p>

    <div v-if="calendarMode === 'month'" class="layout">
      <div class="calendar">
        <div v-for="label in weekdayLabels" :key="label" class="weekday">{{ label }}</div>
        <button
          v-for="(cell, index) in calendarCells"
          :key="`${cell.date}-${index}`"
          type="button"
          class="day"
          :class="{
            empty: !cell.inMonth,
            today: cell.date === todayIso,
            selected: cell.date === selectedDate,
          }"
          :disabled="!cell.inMonth"
          @click="cell.inMonth && selectDate(cell.date)"
        >
          <div v-if="cell.inMonth" class="day-num">{{ cell.day }}</div>
          <div v-if="cell.inMonth && entriesByDate.get(cell.date)?.length" class="events">
            <div
              v-for="entry in entriesByDate.get(cell.date)?.slice(0, 3)"
              :key="entry.id"
              class="event"
              :style="{ borderLeftColor: entry.channelColor }"
            >
              <span class="event-text">{{ entry.channelName }}</span>
            </div>
            <span v-if="(entriesByDate.get(cell.date)?.length ?? 0) > 3" class="more">
              +{{ (entriesByDate.get(cell.date)?.length ?? 0) - 3 }}
            </span>
          </div>
        </button>
      </div>

      <aside v-if="selectedDate" class="detail">
        <header class="detail-head">
          <h2>{{ selectedDateLabel }}</h2>
          <button type="button" class="pk-btn pk-btn--ghost" @click="selectedDate = ''">
            {{ t("calendar.closeDetail") }}
          </button>
        </header>
        <p v-if="!selectedEntries.length" class="muted">{{ t("calendar.dayEmpty") }}</p>
        <ul v-else class="detail-list">
          <li v-for="entry in selectedEntries" :key="entry.id">
            <div class="detail-row">
              <span class="dot" :style="{ background: entry.channelColor }" />
              <div>
                <strong>{{ entry.contentTitle }}</strong>
                <span class="meta">{{ entry.channelName }} · {{ statusLabel(entry.status) }}</span>
              </div>
            </div>
            <a
              v-if="entry.publishUrl"
              class="link"
              :href="entry.publishUrl"
              target="_blank"
              rel="noopener noreferrer"
            >
              {{ entry.publishUrl }}
            </a>
          </li>
        </ul>
      </aside>
    </div>

    <div v-else class="week-layout">
      <p v-if="loading" class="muted">{{ t("calendar.loading") }}</p>
      <ul v-else class="week-list">
        <li v-for="entry in entries" :key="entry.id" class="week-item">
          <span class="date">{{ entry.date }}</span>
          <span class="dot" :style="{ background: entry.channelColor }" />
          <strong>{{ entry.contentTitle }}</strong>
          <span class="meta">{{ entry.channelName }} · {{ statusLabel(entry.status) }}</span>
        </li>
      </ul>
      <p v-if="!loading && !entries.length" class="muted">{{ t("calendar.dayEmpty") }}</p>
    </div>
  </section>
</template>

<style scoped>
.page {
  padding: 24px;
}
.header {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  align-items: flex-start;
  margin-bottom: 16px;
}
.header h1 {
  margin: 0 0 6px;
}
.subtitle {
  margin: 0;
  color: var(--pk-ink-muted);
  font-size: 13px;
}
.nav {
  display: flex;
  align-items: center;
  gap: 8px;
}
.month {
  min-width: 140px;
  text-align: center;
  font-weight: 600;
}
.layout {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 280px;
  gap: 16px;
  align-items: start;
}
.calendar {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 1px;
  background: var(--pk-border);
  border: 1px solid var(--pk-border);
  border-radius: var(--pk-radius-md);
  overflow: hidden;
}
.weekday,
.day {
  background: var(--pk-bg-panel);
  min-height: 96px;
}
.weekday {
  min-height: auto;
  padding: 8px;
  text-align: center;
  font-size: 12px;
  color: var(--pk-ink-muted);
  font-weight: 600;
}
.day {
  padding: 6px;
  border: none;
  text-align: left;
  cursor: pointer;
  font: inherit;
  color: inherit;
}
.day.empty {
  background: var(--pk-bg-alt);
  cursor: default;
}
.day.today .day-num {
  background: var(--pk-accent-soft);
  color: var(--pk-accent-text);
  border-radius: var(--pk-radius-pill);
  width: 24px;
  height: 24px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}
.day.selected {
  outline: 2px solid var(--pk-accent);
  outline-offset: -2px;
  z-index: 1;
}
.day-num {
  font-size: 12px;
  font-weight: 600;
  margin-bottom: 4px;
}
.events {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.event {
  padding: 2px 4px;
  border-left: 3px solid var(--pk-accent);
  background: var(--pk-bg-alt);
  border-radius: 4px;
  font-size: 10px;
}
.event-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--pk-ink);
}
.more {
  font-size: 10px;
  color: var(--pk-ink-muted);
}
.detail {
  border: 1px solid var(--pk-border-strong);
  border-radius: var(--pk-radius-md);
  background: var(--pk-bg-panel);
  padding: 16px;
  min-height: 200px;
}
.detail-head {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 8px;
  margin-bottom: 12px;
}
.detail-head h2 {
  margin: 0;
  font-size: 15px;
}
.detail-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.detail-row {
  display: flex;
  gap: 8px;
  align-items: flex-start;
}
.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  margin-top: 6px;
  flex-shrink: 0;
}
.meta {
  display: block;
  font-size: 12px;
  color: var(--pk-ink-muted);
  margin-top: 2px;
}
.link {
  display: block;
  margin-top: 6px;
  font-size: 11px;
  color: var(--pk-status-published);
  word-break: break-all;
}
.error {
  color: #b42318;
  font-size: 13px;
}
.muted {
  color: var(--pk-ink-muted);
  font-size: 13px;
}
@media (max-width: 960px) {
  .layout {
    grid-template-columns: 1fr;
  }
}
.week-layout {
  margin-top: 12px;
}
.week-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.week-item {
  display: grid;
  grid-template-columns: 100px 10px 1fr auto;
  gap: 8px;
  align-items: center;
  padding: 10px 12px;
  border: 1px solid var(--pk-border);
  border-radius: var(--pk-radius-md);
  background: var(--pk-bg-panel);
  font-size: 13px;
}
.week-item .date {
  color: var(--pk-ink-muted);
  font-size: 12px;
}
.week-item .dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}
.week-item .meta {
  color: var(--pk-ink-muted);
  font-size: 12px;
}
</style>
