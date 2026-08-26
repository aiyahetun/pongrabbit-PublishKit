import { invoke } from "@tauri-apps/api/core";
import type { MediaAsset } from "@publishkit/shared";
import {
  copyLinkedImagesWorkflow,
  copyMediaImage,
  type CopyLinkedImagesResult,
} from "./mediaActions";

async function listLinkedImages(contentItemId: string): Promise<MediaAsset[]> {
  const assets = await invoke<MediaAsset[]>("list_content_media_cmd", {
    contentItemId,
  });
  return assets.filter((item) => item.kind === "image");
}

export async function copyFirstLinkedImage(
  contentItemId: string
): Promise<CopyLinkedImagesResult> {
  const images = await listLinkedImages(contentItemId);
  if (!images.length) {
    return { mode: "none", count: 0 };
  }
  await copyMediaImage(images[0]);
  return { mode: "clipboard", count: 1 };
}

export async function openLinkedImagesFolder(
  contentItemId: string
): Promise<CopyLinkedImagesResult> {
  const images = await listLinkedImages(contentItemId);
  if (!images.length) {
    return { mode: "none", count: 0 };
  }
  const result = await invoke<{ folderPath: string; copiedCount: number }>(
    "stage_content_images_cmd",
    { contentItemId }
  );
  return {
    mode: "folder",
    count: result.copiedCount,
    folderPath: result.folderPath,
  };
}

/** @deprecated Prefer copyFirstLinkedImage / openLinkedImagesFolder for explicit UX. */
export async function copyTaskLinkedImages(
  contentItemId: string
): Promise<CopyLinkedImagesResult> {
  const assets = await invoke<MediaAsset[]>("list_content_media_cmd", {
    contentItemId,
  });
  return copyLinkedImagesWorkflow(contentItemId, assets);
}
