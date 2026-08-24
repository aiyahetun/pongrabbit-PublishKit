import { marked } from "marked";

marked.setOptions({
  gfm: true,
  breaks: true,
});

function markdownToPlain(text: string): string {
  return text
    .replace(/^#{1,6}\s+/gm, "")
    .replace(/\*\*(.+?)\*\*/g, "$1")
    .replace(/\*(.+?)\*/g, "$1")
    .replace(/`(.+?)`/g, "$1")
    .replace(/^\s*[-*+]\s+/gm, "• ")
    .replace(/^\s*\d+\.\s+/gm, "")
    .replace(/\[([^\]]+)\]\([^)]+\)/g, "$1")
    .replace(/^>\s?/gm, "")
    .trim();
}

function wrapHtmlFragment(html: string): string {
  return `<!DOCTYPE html><html><body><!--StartFragment-->${html}<!--EndFragment--></body></html>`;
}

/** Copy markdown as rich text (HTML) + plain text for publishing platforms. */
export async function copyMarkdownAsRichText(markdown: string): Promise<void> {
  const html = marked.parse(markdown, { async: false }) as string;
  const plain = markdownToPlain(markdown);

  if (typeof ClipboardItem !== "undefined" && navigator.clipboard.write) {
    await navigator.clipboard.write([
      new ClipboardItem({
        "text/html": new Blob([wrapHtmlFragment(html)], { type: "text/html" }),
        "text/plain": new Blob([plain], { type: "text/plain" }),
      }),
    ]);
    return;
  }

  await navigator.clipboard.writeText(plain);
}
