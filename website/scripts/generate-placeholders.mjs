import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..");

function svg(w, h, id, label) {
  const fs = Math.max(14, Math.min(28, Math.round(w / 50)));
  return `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="${w}" height="${h}" viewBox="0 0 ${w} ${h}" role="img" aria-label="${label}">
  <rect width="100%" height="100%" fill="#F7F6F3"/>
  <rect x="${Math.round(w * 0.03)}" y="${Math.round(h * 0.05)}" width="${Math.round(w * 0.94)}" height="${Math.round(h * 0.9)}" fill="#FFFFFF" stroke="#D1C5B3" stroke-width="2" stroke-dasharray="12 8" rx="12"/>
  <text x="50%" y="46%" dominant-baseline="middle" text-anchor="middle" fill="#795916" font-family="Inter,Segoe UI,sans-serif" font-size="${fs}" font-weight="600">${id}</text>
  <text x="50%" y="54%" dominant-baseline="middle" text-anchor="middle" fill="#8A8278" font-family="Inter,Segoe UI,sans-serif" font-size="${Math.max(12, fs - 6)}">${w} × ${h} · PLACEHOLDER</text>
  <text x="50%" y="62%" dominant-baseline="middle" text-anchor="middle" fill="#B8924A" font-family="Inter,Segoe UI,sans-serif" font-size="${Math.max(11, fs - 8)}">Replace with AI asset · see PublishKit-官网与Chrome商店配图AI描述词.md</text>
</svg>`;
}

const websiteImages = [
  ["website/assets/images/publishkit_icon_1024.svg", 1024, 1024, "LP-01", "App icon"],
  ["website/assets/images/favicon_32.svg", 32, 32, "LP-02", "Favicon"],
  ["website/assets/images/apple_touch_180.svg", 180, 180, "LP-02", "Apple touch icon"],
  ["website/assets/images/publishkit_hero_desktop_2880x1620.svg", 2880, 1620, "LP-04", "Hero desktop"],
  ["website/assets/images/publishkit_og_1200x630.svg", 1200, 630, "LP-06", "OG share"],
  ["website/assets/images/publishkit_feat_today_1920x1080.svg", 1920, 1080, "LP-07", "Today queue"],
  ["website/assets/images/publishkit_feat_content_inspector_1920x1080.svg", 1920, 1080, "LP-08", "Content inspector"],
  ["website/assets/images/publishkit_feat_extension_1920x1080.svg", 1920, 1080, "LP-09", "Browser extension"],
  ["website/assets/images/publishkit_feat_kanban_1920x1080.svg", 1920, 1080, "LP-10", "Kanban"],
  ["website/assets/images/publishkit_feat_calendar_week_1920x1080.svg", 1920, 1080, "LP-11", "Week calendar"],
];

const cwsImages = [
  ["apps/extension/store/assets/icon_128.svg", 128, 128, "CWS-01", "Store icon"],
  ["apps/extension/store/assets/promo_small_440x280.svg", 440, 280, "CWS-02", "Small promo"],
  ["apps/extension/store/assets/promo_marquee_1400x560.svg", 1400, 560, "CWS-03", "Marquee promo"],
  ["apps/extension/store/assets/screenshot_01_sidepanel_1280x800.svg", 1280, 800, "CWS-04", "Screenshot 1"],
  ["apps/extension/store/assets/screenshot_02_copy_1280x800.svg", 1280, 800, "CWS-05", "Screenshot 2"],
  ["apps/extension/store/assets/screenshot_03_publish_1280x800.svg", 1280, 800, "CWS-06", "Screenshot 3"],
  ["apps/extension/store/assets/screenshot_04_duplicate_warn_1280x800.svg", 1280, 800, "CWS-07", "Screenshot 4"],
  ["apps/extension/store/assets/screenshot_05_pairing_1280x800.svg", 1280, 800, "CWS-08", "Screenshot 5"],
];

async function writePlaceholders(list) {
  for (const [rel, w, h, id, label] of list) {
    const file = path.join(root, rel);
    await mkdir(path.dirname(file), { recursive: true });
    await writeFile(file, svg(w, h, id, label), "utf8");
    console.log("wrote", rel);
  }
}

await writePlaceholders(websiteImages);
await writePlaceholders(cwsImages);
console.log("Done.");
