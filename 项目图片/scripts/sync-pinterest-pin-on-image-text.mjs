/**
 * Sync on-image 文字 blocks in PublishKit-Pinterest配图AI描述词.md
 * from PublishKit-Pinterest营销文案.md (titles + body, no URLs/prices except 定价).
 */
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const dir = path.dirname(fileURLToPath(import.meta.url));
const root = path.join(dir, "..");
const marketingPath = path.join(root, "PublishKit-Pinterest营销文案.md");
const aiPath = path.join(root, "PublishKit-Pinterest配图AI描述词.md");

const marketing = fs.readFileSync(marketingPath, "utf8");
const ai = fs.readFileSync(aiPath, "utf8");

function parseMarketing(md) {
  const sections = new Map();
  const parts = md.split(/\n## /);
  for (const part of parts.slice(1)) {
    const nl = part.indexOf("\n");
    const key = part.slice(0, nl).trim();
    const body = part.slice(nl + 1);
    const zhTitle = body.match(/\*\*标题（中文）：\*\*\s*(.+)/)?.[1]?.trim();
    const enTitle = body.match(/\*\*Title:\*\*\s*(.+)/)?.[1]?.trim();
    const zhBody = body.match(/\*\*正文（中文）：\*\*\s*\n([\s\S]*?)\n\s*https?:/m)?.[1]?.trim();
    const enBody = body.match(/\*\*Description:\*\*\s*\n([\s\S]*?)\n\s*https?:/m)?.[1]?.trim();
    const zhKw = body.match(/\*\*关联词（中文）：\*\*\s*(.+)/)?.[1]?.trim();
    const enKw = body.match(/\*\*Keywords:\*\*\s*(.+)/)?.[1]?.trim();
    if (zhTitle) {
      sections.set(key, { zhTitle, enTitle, zhBody, enBody, zhKw, enKw });
    }
  }
  return sections;
}

function cleanZhBody(text, isPricing) {
  if (!text) return "";
  const lines = text
    .split("\n")
    .map((l) => l.trim())
    .filter((l) => l && !l.startsWith("http") && !l.startsWith("插件："));
  if (isPricing) return lines.join("\n");
  return lines
    .map((l) =>
      l
        .replace(/Free\s*50\s*条[^。]*[。]?/g, "")
        .replace(/Pro\s*买断[^。]*[。]?/g, "")
        .replace(/[（(]以官网为准[）)][。]?/g, "")
        .replace(/¥\d+[^。]*[。]?/g, "")
        .replace(/国内首发[^。]*[。]?/g, "")
        .replace(/海外首发[^。]*[。]?/g, "")
        .replace(/\s{2,}/g, " ")
        .trim()
    )
    .filter(Boolean)
    .join("\n");
}

function cleanEnBody(text, isPricing) {
  if (!text) return "";
  const lines = text
    .split("\n")
    .map((l) => l.trim())
    .filter((l) => l && !l.startsWith("http"));
  if (isPricing) return lines.join("\n");
  return lines
    .map((l) =>
      l
        .replace(/\s*Free for \d+ items\.?/gi, "")
        .replace(/\s*Pro is a one-time[^.]*\.?/gi, "")
        .replace(/\s*Pro one-time[^.]*\.?/gi, "")
        .replace(/\s*\(\$[\d.]+\)[^.]*\.?/g, "")
        .replace(/\s*Free \d+ items[^.]*\.?/gi, "")
        .replace(/\s*Check the site for[^.]*\.?/gi, "")
        .replace(/\s{2,}/g, " ")
        .trim()
    )
    .filter(Boolean)
    .join("\n");
}

function kwLine(kw) {
  if (!kw) return "";
  return kw.split(/[,，、]/).map((s) => s.trim()).filter(Boolean).join(" · ");
}

function buildTextBlock(title, body, kwLineStr) {
  const lines = [title, "", ...body.split("\n").filter(Boolean)];
  if (kwLineStr) lines.push("", kwLineStr);
  return lines.join("\n");
}

function quoteBlock(text) {
  return text
    .split("\n")
    .map((l) => (l === "" ? ">" : `> ${l}`))
    .join("\n");
}

const data = parseMarketing(marketing);
let out = ai;
let updated = 0;
let missing = [];

for (const [key, v] of data) {
  const isPricing = key.includes("定价区背景");
  const zhText = buildTextBlock(v.zhTitle, cleanZhBody(v.zhBody, isPricing), kwLine(v.zhKw));
  const enText = buildTextBlock(v.enTitle, cleanEnBody(v.enBody, isPricing), kwLine(v.enKw));

  const esc = key.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const sectionRe = new RegExp(
    `(## ${esc}[\\s\\S]*?### 中文 Pin[\\s\\S]*?> 文字[\\s\\S]*?：\\r?\\n)([\\s\\S]*?)(\\r?\\n>\\r?\\n> 底部统一说明：)`,
    "m"
  );
  const m1 = out.match(sectionRe);
  if (m1) {
    out = out.replace(sectionRe, `$1${quoteBlock(zhText)}$3`);
    updated++;
  } else {
    missing.push(`zh:${key}`);
  }

  const enRe = new RegExp(
    `(## ${esc}[\\s\\S]*?### 英文 Pin[\\s\\S]*?> 文字[\\s\\S]*?：\\r?\\n)([\\s\\S]*?)(\\r?\\n>\\r?\\n> 底部统一说明：)`,
    "m"
  );
  if (enRe.test(out)) {
    out = out.replace(enRe, `$1${quoteBlock(enText)}$3`);
  } else {
    missing.push(`en:${key}`);
  }
}

fs.writeFileSync(aiPath, out, "utf8");
console.log(`Updated sections: ${updated}, missing: ${missing.join(", ") || "none"}`);
