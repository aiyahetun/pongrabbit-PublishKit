#!/usr/bin/env node
/**
 * PublishKit live preflight (Phase 1).
 * Run from repo root: node website/scripts/verify-live.mjs
 *
 * Does not create a real Paddle transaction (invalid email only).
 */
import { readFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const UA = "PublishKit-verify-live/1.0";
const errors = [];
const warnings = [];
const skipCn = process.env.PUBLISHKIT_SKIP_CN === "1";

const GET_COM = "https://get.pongrabbit.com";
const GET_CN = "https://get.pongrabbit.cn";
const APP_API = "https://app.pongrabbit.com";
const BAD_API_HOST = "api.pongrabbit.com";

function localSiteJs() {
  const p = join(dirname(fileURLToPath(import.meta.url)), "..", "assets", "site.js");
  if (!existsSync(p)) {
    errors.push("missing website/assets/site.js");
    return "";
  }
  return readFileSync(p, "utf8");
}

async function request(url, options = {}) {
  const ctrl = new AbortController();
  const t = setTimeout(() => ctrl.abort(), options.timeoutMs || 15000);
  try {
    const res = await fetch(url, {
      ...options,
      signal: ctrl.signal,
      headers: {
        "User-Agent": UA,
        ...(options.headers || {}),
      },
      redirect: "follow",
    });
    const text = await res.text();
    return { res, text };
  } catch (e) {
    return { error: e };
  } finally {
    clearTimeout(t);
  }
}

function must(cond, msg) {
  if (!cond) errors.push(msg);
}

async function main() {
  const localJs = localSiteJs();
  if (localJs) {
    must(localJs.includes("pkCheckoutIntl"), "[local site.js] missing pkCheckoutIntl");
    must(localJs.includes("pkCheckoutCn"), "[local site.js] missing pkCheckoutCn");
    must(
      /apiBase\s*:\s*\{[\s\S]*zh:\s*"https:\/\/api\.pongrabbit\.cn"/.test(localJs),
      "[local site.js] zh apiBase must be https://api.pongrabbit.cn"
    );
    must(
      /apiBase\s*:\s*\{[\s\S]*en:\s*"https:\/\/app\.pongrabbit\.com"/.test(localJs),
      "[local site.js] en apiBase must be https://app.pongrabbit.com"
    );
    if (localJs.includes(`en: "https://${BAD_API_HOST}"`) || localJs.includes(`en: 'https://${BAD_API_HOST}'`)) {
      errors.push("[local site.js] en apiBase must not use api.pongrabbit.com (no DNS)");
    }
  }

  const enPage = await request(`${GET_COM}/en/`);
  if (enPage.error) errors.push(`[L1] GET ${GET_COM}/en/ failed: ${enPage.error.message}`);
  else must(enPage.res.ok, `[L1] GET ${GET_COM}/en/ HTTP ${enPage.res.status}`);

  const siteJs = await request(`${GET_COM}/assets/site.js`);
  if (siteJs.error) errors.push(`[L2] GET site.js failed: ${siteJs.error.message}`);
  else {
    must(siteJs.res.ok, `[L2] GET site.js HTTP ${siteJs.res.status}`);
    must(siteJs.text.includes("pkOpenPaddleOverlay"), "[L2] live site.js missing Paddle overlay opener");
    must(siteJs.text.includes("https://app.pongrabbit.com"), "[L2] live site.js must call app.pongrabbit.com");
    if (/apiBase[\s\S]{0,200}en:\s*"https:\/\/api\.pongrabbit\.com"/.test(siteJs.text)) {
      errors.push("[L3] live site.js en apiBase is api.pongrabbit.com (will fail in browser)");
    }
  }

  const health = await request(`${APP_API}/api/v1/health`);
  if (health.error) errors.push(`[L4] health failed: ${health.error.message}`);
  else {
    must(health.res.ok, `[L4] health HTTP ${health.res.status} (get site 200 does not mean API is up)`);
    let json = null;
    try {
      json = JSON.parse(health.text);
    } catch (e) {
      errors.push(`[L4] health is not JSON: ${health.text.slice(0, 180)}`);
    }
    if (json) {
      must(json.ok === true, "[L4] health.ok !== true");
      must(json.region === "intl", `[L4] health.region=${json.region} expected intl`);
      must(json.dbOk === true, "[L4] health.dbOk !== true");
    }
  }

  const checkout = await request(`${APP_API}/api/v1/publishkit/checkout`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Origin: `${GET_COM}`,
    },
    body: JSON.stringify({ email: "not-an-email" }),
  });
  if (checkout.error) errors.push(`[L5] checkout failed: ${checkout.error.message}`);
  else {
    must(checkout.res.status === 400, `[L5] checkout invalid email expected 400, got ${checkout.res.status}`);
    must(/email/i.test(checkout.text), `[L5] checkout 400 body should mention email: ${checkout.text.slice(0, 240)}`);
    const acao = checkout.res.headers.get("access-control-allow-origin");
    must(
      acao === GET_COM,
      `[L6] CORS ACAO expected ${GET_COM}, got ${acao || "(missing)"}`
    );
  }

  if (!skipCn) {
    const zhPage = await request(`${GET_CN}/zh/`);
    if (zhPage.error) warnings.push(`[L7] GET ${GET_CN}/zh/ skipped/failed: ${zhPage.error.message}`);
    else if (!zhPage.res.ok) warnings.push(`[L7] GET ${GET_CN}/zh/ HTTP ${zhPage.res.status}`);
  } else {
    warnings.push("[L7] skipped (PUBLISHKIT_SKIP_CN=1)");
  }

  const dnsApi = await request(`https://${BAD_API_HOST}/api/v1/health`, { timeoutMs: 8000 });
  if (dnsApi.error) {
    warnings.push(`[dns] ${BAD_API_HOST} unreachable (${dnsApi.error.message}) — expected; keep using app.pongrabbit.com`);
  } else {
    warnings.push(`[dns] ${BAD_API_HOST} responded HTTP ${dnsApi.res.status} — still prefer app.pongrabbit.com in site.js`);
  }

  if (warnings.length) {
    console.warn("Warnings:");
    warnings.forEach((w) => console.warn("  -", w));
  }

  console.log("SSH reminders (not scanned):");
  console.log("  - do not compose up / recreate penrabbit-api-1");
  console.log("  - this host: node is PID 1 → docker restart same container after docker cp");
  console.log("  - reload-api-node.js does not replace PID 1");
  console.log("  - Paddle: get.pongrabbit.com must be Approved");
  console.log("  - never POST a real customer email to checkout from this script");

  if (errors.length) {
    console.error("\nLive verification failed:\n");
    errors.forEach((e) => console.error("  x", e));
    process.exit(1);
  }

  console.log("\nLive verification passed.");
}

main().catch((e) => {
  console.error("Live verification crashed:", e);
  process.exit(1);
});
