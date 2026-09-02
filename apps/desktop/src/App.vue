<script setup lang="ts">
import { onMounted, ref } from "vue";
import { RouterView } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import type { WorkspaceSettings } from "@publishkit/shared";
import AppSidebar from "./components/AppSidebar.vue";
import AppTopBar from "./components/AppTopBar.vue";
import OnboardingModal from "./components/OnboardingModal.vue";

const showOnboarding = ref(false);

onMounted(async () => {
  try {
    const settings = await invoke<WorkspaceSettings>("get_workspace_settings");
    showOnboarding.value = !settings.onboardingDone;
  } catch {
    showOnboarding.value = false;
  }
});
</script>

<template>
  <OnboardingModal v-if="showOnboarding" @done="showOnboarding = false" />
  <div v-else class="shell">
    <AppSidebar />
    <div class="main-col">
      <AppTopBar />
      <main class="main">
        <RouterView />
      </main>
    </div>
  </div>
</template>

<style scoped>
.shell {
  display: flex;
  height: 100vh;
}
.main-col {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
}
.main {
  flex: 1;
  overflow: auto;
  min-width: 0;
}
</style>
