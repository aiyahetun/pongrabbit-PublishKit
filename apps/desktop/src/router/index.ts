import { createRouter, createWebHistory } from "vue-router";
import TodayView from "../views/TodayView.vue";
import ContentView from "../views/ContentView.vue";
import TasksView from "../views/TasksView.vue";
import SourcesView from "../views/SourcesView.vue";
import SettingsView from "../views/SettingsView.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", redirect: "/today" },
    { path: "/today", component: TodayView },
    { path: "/content", component: ContentView },
    { path: "/tasks", component: TasksView },
    { path: "/sources", component: SourcesView },
    { path: "/settings", component: SettingsView },
  ],
});
