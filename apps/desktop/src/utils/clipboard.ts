import { invoke } from "@tauri-apps/api/core";

/** Copy markdown as rich text (HTML) + plain text for publishing platforms. */
export async function copyMarkdownAsRichText(markdown: string): Promise<void> {
  await invoke("copy_markdown_rich_text_cmd", { markdown });
}
