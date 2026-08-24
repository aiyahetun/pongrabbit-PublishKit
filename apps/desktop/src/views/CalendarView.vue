<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import type { CalendarEntry } from "@publishkit/shared";

const { t, locale } = useI18n();
const today = new Date();
const viewYear = ref(today.getFullYear());
const viewMonth = ref(today.getMonth() + 1);
const entries = ref<CalendarEntry[]>([]);
const loading = ref(false);
const error = ref("");

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

function statusLabel(status: string) {
  return t(`tasks.status_${status}`);
}

async function loadEntries() {
  loading.value = true;
  error.value = "";
  try {
    entries.value = await invoke<CalendarEntry[]>("list_calendar_entries_cmd", {
      year: viewYear.value,
      month: viewMonth.value,
    });
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
}

watch([viewYear, viewMonth], loadEntries);
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
        <button type="button" class="pk-btn pk-btn--ghost" @click="prevMonth">‹</button>
        <span class="month">{{ monthLabel }}</span>
        <button type="button" class="pk-btn pk-btn--ghost" @click="nextMonth">›</button>
        <button type="button" class="pk-btn pk-btn--secondary" @click="goToday">
          {{ t("calendar.today") }}
        </button>
      </div>
    </header>

    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="loading" class="muted">{{ t("calendar.loading") }}</p>

    <div class="calendar">
      <div v-for="label in weekdayLabels" :key="label" class="weekday">{{ label }}</div>
      <div
        v-for="(cell, index) in calendarCells"
        :key="`${cell.date}-${index}`"
        class="day"
        :class="{ empty: !cell.inMonth, today: cell.date === `${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}-${String(today.getDate()).padStart(2, '0')}` }"
      >
        <div v-if="cell.inMonth" class="day-num">{{ cell.day }}</div>
        <div v-if="cell.inMonth && entriesByDate.get(cell.date)?.length" class="events">
          <div
            v-for="entry in entriesByDate.get(cell.date)"
            :key="entry.id"
            class="event"
            :style="{ borderLeftColor: entry.channelColor }"
            :title="`${entry.channelName} · ${entry.contentTitle}`"
          >
            <span class="dot" :style="{ background: entry.channelColor }" />
            <span class="event-text">{{ entry.channelName }} · {{ entry.contentTitle }}</span>
            <span class="event-status">{{ statusLabel(entry.status) }}</span>
          </div>
        </div>
      </div>
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
  vertical-align: top;
}
.day.empty {
  background: var(--pk-bg-alt);
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
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 4px 6px;
  border-left: 3px solid var(--pk-accent);
  background: var(--pk-bg-alt);
  border-radius: 4px;
  font-size: 11px;
}
.dot {
  display: none;
}
.event-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--pk-ink);
}
.event-status {
  color: var(--pk-ink-muted);
}
.error {
  color: #b42318;
  font-size: 13px;
}
.muted {
  color: var(--pk-ink-muted);
  font-size: 13px;
}
</style>
