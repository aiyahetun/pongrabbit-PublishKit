import { invoke } from "@tauri-apps/api/core";
import type { WorkspaceSettings } from "@publishkit/shared";

export async function setProjectRoot(
  command: "set_copy_root" | "set_media_root" | "set_video_root",
  path: string,
  confirmBound: (projectName: string) => boolean
): Promise<WorkspaceSettings> {
  try {
    return await invoke<WorkspaceSettings>(command, { path, force: false });
  } catch (error) {
    const message = String(error);
    const marker = "ROOT_BOUND:";
    const index = message.indexOf(marker);
    if (index === -1) throw error;
    const projectName = message.slice(index + marker.length).trim();
    if (!confirmBound(projectName)) {
      throw new Error("cancelled");
    }
    return await invoke<WorkspaceSettings>(command, { path, force: true });
  }
}
