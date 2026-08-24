import { invoke } from "@tauri-apps/api/core";
import type { MediaAsset } from "@publishkit/shared";

export async function copyMediaImage(asset: MediaAsset): Promise<void> {
  if (asset.kind !== "image") {
    throw new Error("Only images can be copied to clipboard");
  }
  await invoke("copy_media_image_cmd", { path: asset.path });
}

export async function revealMediaInFolder(asset: MediaAsset): Promise<void> {
  await invoke("reveal_media_in_folder_cmd", { path: asset.path });
}

export async function copyLinkedImages(assets: MediaAsset[]): Promise<number> {
  const images = assets.filter((item) => item.kind === "image");
  for (const asset of images) {
    await copyMediaImage(asset);
  }
  return images.length;
}
