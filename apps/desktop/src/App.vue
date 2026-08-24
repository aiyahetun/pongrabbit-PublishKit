<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { RouterLink, RouterView } from "vue-router";

const { t } = useI18n();

const nav = [
  { to: "/today", key: "nav.today" },
  { to: "/content", key: "nav.content" },
  { to: "/tasks", key: "nav.tasks" },
  { to: "/sources", key: "nav.sources" },
  { to: "/settings", key: "nav.settings" },
];
</script>

<template>
  <div class="shell">
    <aside class="sidebar">
      <div class="brand">
        <div class="brand-title">{{ t("app.name") }}</div>
        <div class="brand-sub">{{ t("app.subtitle") }}</div>
      </div>
      <nav>
        <RouterLink v-for="item in nav" :key="item.to" :to="item.to" class="nav-link">
          {{ t(item.key) }}
        </RouterLink>
      </nav>
    </aside>
    <main class="main">
      <RouterView />
    </main>
  </div>
</template>

<style scoped>
.shell {
  display: flex;
  height: 100vh;
}
.sidebar {
  width: var(--pk-sidebar-w);
  background: var(--pk-bg-panel);
  border-right: 1px solid var(--pk-border);
  padding: 16px 12px;
}
.brand {
  padding: 8px 10px 16px;
}
.brand-title {
  font-weight: 600;
  font-size: 18px;
}
.brand-sub {
  font-size: 12px;
  color: var(--pk-ink-muted);
}
.nav-link {
  display: block;
  padding: 10px 12px;
  border-radius: 8px;
  color: var(--pk-ink-secondary);
  text-decoration: none;
  margin-bottom: 4px;
}
.nav-link.router-link-active {
  background: var(--pk-accent-soft);
  color: var(--pk-ink);
}
.main {
  flex: 1;
  overflow: auto;
}
</style>
