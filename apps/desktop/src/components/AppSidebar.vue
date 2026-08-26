<script setup lang="ts">
import { onMounted, ref } from "vue";
import { RouterLink, useRoute } from "vue-router";
import { useI18n } from "vue-i18n";
import { NAV_COLLAPSE_STORAGE_KEY, NAV_GROUPS } from "../config/nav";

const { t } = useI18n();
const route = useRoute();
const collapsed = ref<Set<string>>(new Set());

function loadCollapsed() {
  try {
    const raw = localStorage.getItem(NAV_COLLAPSE_STORAGE_KEY);
    if (!raw) return;
    const parsed = JSON.parse(raw) as string[];
    collapsed.value = new Set(parsed);
  } catch {
    collapsed.value = new Set();
  }
}

function persistCollapsed() {
  localStorage.setItem(NAV_COLLAPSE_STORAGE_KEY, JSON.stringify([...collapsed.value]));
}

function isCollapsed(groupId: string) {
  return collapsed.value.has(groupId);
}

function toggleGroup(groupId: string) {
  const next = new Set(collapsed.value);
  if (next.has(groupId)) next.delete(groupId);
  else next.add(groupId);
  collapsed.value = next;
  persistCollapsed();
}

function isActive(path: string) {
  return route.path === path || route.path.startsWith(`${path}/`);
}

onMounted(loadCollapsed);
</script>

<template>
  <aside class="sidebar">
    <div class="brand">
      <div class="brand-title">{{ t("app.name") }}</div>
      <div class="brand-sub">{{ t("app.subtitle") }}</div>
      <p class="brand-flow">{{ t("nav.flowHint") }}</p>
    </div>

    <nav class="nav-groups">
      <section v-for="group in NAV_GROUPS" :key="group.id" class="nav-group">
        <button
          type="button"
          class="group-head"
          :aria-expanded="!isCollapsed(group.id)"
          @click="toggleGroup(group.id)"
        >
          <span class="group-title">{{ t(group.titleKey) }}</span>
          <span class="group-chevron" :class="{ collapsed: isCollapsed(group.id) }">›</span>
        </button>

        <ul v-show="!isCollapsed(group.id)" class="group-items">
          <li v-for="item in group.items" :key="item.to">
            <RouterLink :to="item.to" class="nav-link" :class="{ active: isActive(item.to) }">
              <span v-if="item.step" class="step">{{ item.step }}</span>
              <span v-else class="step step--plain">·</span>
              <span class="link-text">
                <span class="link-label">{{ t(item.labelKey) }}</span>
                <span class="link-hint">{{ t(item.hintKey) }}</span>
              </span>
            </RouterLink>
          </li>
        </ul>
      </section>
    </nav>
  </aside>
</template>

<style scoped>
.sidebar {
  width: var(--pk-sidebar-w);
  background: var(--pk-bg-panel);
  border-right: 1px solid var(--pk-border);
  padding: var(--pk-space-4) var(--pk-space-3);
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.brand {
  padding: var(--pk-space-2) var(--pk-space-3) var(--pk-space-4);
  border-bottom: 1px solid var(--pk-border);
  margin-bottom: var(--pk-space-3);
}

.brand-title {
  font-weight: 600;
  font-size: 18px;
}

.brand-sub {
  font-size: 12px;
  color: var(--pk-ink-muted);
  margin-top: 2px;
}

.brand-flow {
  margin: var(--pk-space-3) 0 0;
  font-size: 11px;
  line-height: 1.45;
  color: var(--pk-ink-muted);
}

.nav-groups {
  display: flex;
  flex-direction: column;
  gap: var(--pk-space-2);
  overflow: auto;
}

.nav-group {
  border-radius: var(--pk-radius-md);
}

.group-head {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--pk-space-2);
  padding: 6px var(--pk-space-3);
  border: none;
  background: transparent;
  font: inherit;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.02em;
  text-transform: none;
  color: var(--pk-accent-text);
  cursor: pointer;
  text-align: left;
}

.group-chevron {
  display: inline-block;
  transform: rotate(90deg);
  transition: transform 0.15s ease;
  color: var(--pk-ink-muted);
  font-size: 14px;
}

.group-chevron.collapsed {
  transform: rotate(0deg);
}

.group-items {
  list-style: none;
  margin: 0;
  padding: 0 0 var(--pk-space-1);
}

.nav-link {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 8px var(--pk-space-3);
  border-radius: var(--pk-radius-md);
  color: var(--pk-ink-secondary);
  text-decoration: none;
  margin-bottom: 2px;
}

.nav-link:hover {
  background: var(--pk-bg-alt);
}

.nav-link.active {
  background: var(--pk-accent-soft);
  color: var(--pk-ink);
}

.step {
  flex-shrink: 0;
  width: 22px;
  height: 22px;
  margin-top: 1px;
  border-radius: var(--pk-radius-pill);
  border: 1px solid var(--pk-border-strong);
  background: var(--pk-bg-app);
  color: var(--pk-accent-text);
  font-size: 11px;
  font-weight: 700;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.nav-link.active .step {
  background: var(--pk-accent);
  border-color: var(--pk-accent);
  color: var(--pk-inverse);
}

.step--plain {
  font-size: 16px;
  line-height: 1;
  border-color: transparent;
  background: transparent;
  color: var(--pk-ink-muted);
}

.link-text {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.link-label {
  font-size: 14px;
  font-weight: 500;
  line-height: 1.3;
}

.link-hint {
  font-size: 11px;
  line-height: 1.35;
  color: var(--pk-ink-muted);
}

.nav-link.active .link-hint {
  color: var(--pk-ink-secondary);
}
</style>
