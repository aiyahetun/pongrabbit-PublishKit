#!/usr/bin/env node
/**
 * PublishKit website SEO smoke checks (Phase 0-SEO).
 * Run from repo root: node website/scripts/verify-seo.mjs
 */
import { readFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const errors = [];
const warnings = [];

function read(rel) {
  const p = join(root, rel);
  if (!existsSync(p)) {
    errors.push(`missing file: ${rel}`);
    return "";
  }
  return readFileSync(p, "utf8");
}

function expect(html, label, pattern, msg) {
  if (!pattern.test(html)) errors.push(`[${label}] ${msg}`);
}

function rejectIfFound(html, label, pattern, msg) {
  if (pattern.test(html)) errors.push(`[${label}] ${msg}`);
}

function rejectTaizhang(html, label) {
  if (html.includes("\u8d26\u518c")) errors.push(`[${label}] remove 台账 from user-facing copy`);
}

function expectAbsOg(html, label, domain) {
  expect(
    html,
    label,
    new RegExp(`property="og:image" content="https://${domain.replace(/\./g, "\\.")}/assets/images/publishkit_og_1200x630\\.png"`),
    "og:image must be absolute URL on correct domain"
  );
}

function parseJsonLdBlocks(html, label) {
  const re = /<script type="application\/ld\+json">([\s\S]*?)<\/script>/g;
  let m;
  let count = 0;
  while ((m = re.exec(html)) !== null) {
    count++;
    try {
      JSON.parse(m[1].trim());
    } catch (e) {
      errors.push(`[${label}] invalid JSON-LD: ${e.message}`);
    }
  }
  if (count < 2) {
    errors.push(`[${label}] expected at least 2 JSON-LD blocks, found ${count}`);
  }
}

// --- HTML pages ---
const zh = read("zh/index.html");
const en = read("en/index.html");
const privacy = read("privacy.html");

if (zh) {
  expect(zh, "zh", /rel="canonical" href="https:\/\/get\.pongrabbit\.cn\/zh\/"/, "canonical URL");
  expect(zh, "zh", /hreflang="zh-CN"/, "hreflang zh-CN");
  expect(zh, "zh", /hreflang="en"/, "hreflang en");
  expect(zh, "zh", /hreflang="x-default"/, "hreflang x-default");
  expectAbsOg(zh, "zh", "get.pongrabbit.cn");
  expect(zh, "zh", /name="twitter:card" content="summary_large_image"/, "twitter:card");
  expect(zh, "zh", /能够帮助你管理自媒体发布任务和内容的轻应用/, "meta description");
  expect(zh, "zh", /更「轻」的自媒体发布内容管理助手/, "title");
  rejectTaizhang(zh, "zh");
  rejectIfFound(zh, "zh", /og:image" content="\.\./, "og:image must not be relative");
  rejectIfFound(zh, "zh", /爱发电|面包多/, "remove stale payment vendor names from zh page");
  parseJsonLdBlocks(zh, "zh");
}

if (en) {
  expect(en, "en", /rel="canonical" href="https:\/\/get\.pongrabbit\.com\/en\/"/, "canonical URL");
  expect(en, "en", /hreflang="zh-CN"/, "hreflang zh-CN");
  expectAbsOg(en, "en", "get.pongrabbit.com");
  expect(en, "en", /PublishKit is a light Windows app plus a Chrome extension for managing social posts/, "meta description");
  expect(en, "en", /A lighter assistant for your posts/, "title");
  expect(en, "en", /A one-time buffer alternative if you still paste each post yourself\./, "buffer sentence on the pricing card");
  const enMeta = en.match(/<meta name="description" content="([^"]*)"/);
  if (enMeta && /buffer/i.test(enMeta[1])) {
    errors.push("[en] buffer sentence must stay off the meta description");
  }
  const enTitle = en.match(/<title>([^<]*)<\/title>/);
  if (enTitle && /buffer/i.test(enTitle[1])) {
    errors.push("[en] buffer sentence must stay off the title");
  }
  rejectIfFound(en, "en", /ledger/i, "remove ledger from user-facing copy");
  rejectIfFound(en, "en", /Lemon Squeezy/i, "remove stale Lemon Squeezy copy from page");
  rejectIfFound(en, "en", /Proceed to Checkout \(\$69\)/, "remove wrong checkout price");
  parseJsonLdBlocks(en, "en");
}

// --- Static SEO files ---
const robots = read("robots.txt");
if (robots) {
  expect(robots, "robots", /Sitemap: https:\/\/get\.pongrabbit\.cn\/sitemap\.xml/, "cn sitemap");
  expect(robots, "robots", /Sitemap: https:\/\/get\.pongrabbit\.com\/sitemap\.xml/, "com sitemap");
}

const sitemapCom = read("sitemap.com.xml") || read("sitemap.xml");
const sitemapCn = read("sitemap.cn.xml");
if (sitemapCom) {
  for (const url of [
    "https://get.pongrabbit.com/en/",
    "https://get.pongrabbit.com/privacy.html",
  ]) {
    if (!sitemapCom.includes(url)) errors.push(`[sitemap.com] missing ${url}`);
  }
  if (sitemapCom.includes("get.pongrabbit.cn")) {
    errors.push("[sitemap.com] must not include .cn URLs (GSC cross-domain)");
  }
}
if (sitemapCn) {
  for (const url of [
    "https://get.pongrabbit.cn/zh/",
    "https://get.pongrabbit.cn/privacy.html",
  ]) {
    if (!sitemapCn.includes(url)) errors.push(`[sitemap.cn] missing ${url}`);
  }
  if (sitemapCn.includes("get.pongrabbit.com")) {
    errors.push("[sitemap.cn] must not include .com URLs");
  }
}

const llms = read("llms.txt");
if (llms) {
  expect(llms, "llms", /Does NOT auto-post/, "product boundary in llms.txt");
  expect(llms, "llms", /get\.pongrabbit\.com\/en\//, "en canonical in llms.txt");
}

if (privacy && !privacy.includes('rel="canonical"')) {
  warnings.push("[privacy] no canonical link yet");
}

// --- Report ---
if (warnings.length) {
  console.warn("Warnings:");
  warnings.forEach((w) => console.warn("  ⚠", w));
}

if (errors.length) {
  console.error("SEO verification failed:\n");
  errors.forEach((e) => console.error("  ✗", e));
  process.exit(1);
}

console.log("SEO verification passed (Phase 0-SEO E1–E9).");
console.log("  zh/index.html  canonical · hreflang · og · JSON-LD");
console.log("  en/index.html  canonical · hreflang · og · JSON-LD");
console.log("  robots.txt · sitemap.xml · llms.txt");
