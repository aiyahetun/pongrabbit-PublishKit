<script setup lang="ts">
import { computed } from "vue";
import type { MediaAsset } from "@publishkit/shared";
import { mediaOriginalSrc, mediaPreviewSrc } from "../utils/mediaPreview";

const props = withDefaults(
  defineProps<{
    asset: MediaAsset;
    size?: "sm" | "md" | "lg";
    preview?: boolean;
  }>(),
  {
    size: "md",
    preview: false,
  }
);

const src = computed(() =>
  props.preview ? mediaOriginalSrc(props.asset) : mediaPreviewSrc(props.asset)
);
</script>

<template>
  <div class="media-thumb" :class="size" :data-kind="asset.kind">
    <img v-if="src" :src="src" :alt="asset.fileName" loading="lazy" />
    <span v-else-if="asset.kind === 'video'" class="placeholder">▶</span>
    <span v-else class="placeholder">◻</span>
  </div>
</template>

<style scoped>
.media-thumb {
  border-radius: var(--pk-radius-sm);
  background: var(--pk-bg-alt);
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}
.media-thumb.sm {
  width: 48px;
  height: 48px;
}
.media-thumb.md {
  width: 72px;
  height: 72px;
}
.media-thumb.lg {
  width: 120px;
  height: 120px;
}
.media-thumb img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}
.placeholder {
  color: var(--pk-ink-muted);
  font-size: 18px;
}
.media-thumb[data-kind="video"] {
  background: #eef2ff;
  color: #4338ca;
}
</style>
