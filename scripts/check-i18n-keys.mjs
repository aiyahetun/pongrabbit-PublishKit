import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const zh = JSON.parse(readFileSync(join(root, "packages/i18n/locales/zh-CN.json"), "utf8"));
const en = JSON.parse(readFileSync(join(root, "packages/i18n/locales/en.json"), "utf8"));

const zhKeys = new Set(Object.keys(zh));
const enKeys = new Set(Object.keys(en));

const missingInEn = [...zhKeys].filter((k) => !enKeys.has(k));
const missingInZh = [...enKeys].filter((k) => !zhKeys.has(k));

if (missingInEn.length || missingInZh.length) {
  if (missingInEn.length) console.error("Missing in en.json:", missingInEn.join(", "));
  if (missingInZh.length) console.error("Missing in zh-CN.json:", missingInZh.join(", "));
  process.exit(1);
}

console.log(`i18n OK: ${zhKeys.size} keys matched`);
