import { invoke } from "@tauri-apps/api/core";
import type { MediaAsset, StageImagesResult } from "@publishkit/shared";

export async function copyMediaImage(asset: MediaAsset): Promise<void> {
  if (asset.kind !== "image") {
    throw new Error("Only images can be copied to clipboard");
  }
  await invoke("copy_media_image_cmd", { path: asset.path });
}

export async function revealMediaInFolder(asset: MediaAsset): Promise<void> {
  await invoke("reveal_media_in_folder_cmd", { path: asset.path });
}

export type CopyLinkedImagesResult =
  | { mode: "none"; count: 0 }
  | { mode: "clipboard"; count: 1 }
  | { mode: "folder"; count: number; folderPath: string };

export async function copyLinkedImagesWorkflow(
  contentItemId: string,
  assets: MediaAsset[]
): Promise<CopyLinkedImagesResult> {
  const images = assets.filter((item) => item.kind === "image");
  if (!images.length) {
    return { mode: "none", count: 0 };
  }
  if (images.length === 1) {
    await copyMediaImage(images[0]);
    return { mode: "clipboard", count: 1 };
  }
  const result = await invoke<StageImagesResult>("stage_content_images_cmd", {
    contentItemId,
  });
  return {
    mode: "folder",
    count: result.copiedCount,
    folderPath: result.folderPath,
  };
}
