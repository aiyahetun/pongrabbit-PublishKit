<script setup lang="ts">
import { RouterView } from "vue-router";
import AppSidebar from "./components/AppSidebar.vue";
</script>

<template>
  <div class="shell">
    <AppSidebar />
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

.main {
  flex: 1;
  overflow: auto;
  min-width: 0;
}
</style>
