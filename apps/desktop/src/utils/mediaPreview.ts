import { convertFileSrc } from "@tauri-apps/api/core";
import type { MediaAsset } from "@publishkit/shared";

export function mediaPreviewSrc(asset: MediaAsset): string | null {
  if (asset.kind !== "image") return null;
  if (!asset.thumbPath) return null;
  return convertFileSrc(asset.thumbPath);
}

export function mediaOriginalSrc(asset: MediaAsset): string | null {
  if (asset.kind !== "image") return null;
  return convertFileSrc(asset.path);
}
