window.PK_SITE = {
  locale: document.documentElement.lang.startsWith("zh") ? "zh" : "en",
  version: "0.3.12",
  downloadUrl: "https://github.com/aiyahetun/pongrabbit-PublishKit/releases/latest",
  chromeWebStoreUrl:
    "https://chromewebstore.google.com/detail/publishkit-companion/dijkbipanbpkgndbhonecpfnladaiipj",
  githubUrl: "https://github.com/aiyahetun/pongrabbit-PublishKit",
  contactEmail: {
    zh: "support@pongrabbit.cn",
    en: "support@pongrabbit.com",
  },
  apiBase: {
    zh: "https://api.pongrabbit.cn",
    // Overseas API hostname is app.pongrabbit.com (api.pongrabbit.com DNS is not live).
    en: "https://app.pongrabbit.com",
  },
  checkoutUrl: {
    en: "", // legacy static URL fallback
    zh: "", // WeChat Native checkout URL (Phase 2)
  },
  pricing: {
    en: { pro: 38, proWas: 79, currency: "$" },
    zh: { pro: 139, proWas: 299, currency: "¥" },
  },
};

function pkFormatPrice(value, currency) {
  return currency === "¥" ? `¥${value}` : `$${value}`;
}

function pkApplyPricing() {
  const cfg = window.PK_SITE;
  const p = cfg.pricing[cfg.locale] || cfg.pricing.en;
  document.querySelectorAll("[data-pro-price]").forEach((el) => {
    el.textContent = pkFormatPrice(p.pro, p.currency);
  });
  document.querySelectorAll("[data-pro-was]").forEach((el) => {
    el.textContent = pkFormatPrice(p.proWas, p.currency);
  });
  document.querySelectorAll("[data-app-version]").forEach((el) => {
    el.textContent = `v${cfg.version}`;
  });
}

function pkInstallerUrl(fileName) {
  const version = window.PK_SITE.version;
  return (
    "https://github.com/aiyahetun/pongrabbit-PublishKit/releases/download/v" +
    version +
    "/" +
    fileName
  );
}

function pkDownload(kind) {
  const version = window.PK_SITE.version;
  const file =
    kind === "mac"
      ? "PublishKit_" + version + "_aarch64.dmg"
      : kind === "mac-intel"
        ? "PublishKit_" + version + "_x64.dmg"
        : "PublishKit_" + version + "_x64-setup.exe";
  window.open(pkInstallerUrl(file), "_blank", "noopener,noreferrer");
}

function pkChromeStore() {
  const url = window.PK_SITE.chromeWebStoreUrl;
  if (url) window.open(url, "_blank", "noopener,noreferrer");
}

function pkCheckout() {
  const cfg = window.PK_SITE;
  const url = cfg.checkoutUrl[cfg.locale];
  if (url) {
    window.open(url, "_blank", "noopener,noreferrer");
    return;
  }
  if (cfg.locale === "en") {
    pkCheckoutIntl();
    return;
  }
  pkCheckoutCn();
}

async function pkCheckoutIntl() {
  const cfg = window.PK_SITE;
  const email = window.prompt(
    "Enter your email to receive the PublishKit Pro license key after checkout:",
    ""
  );
  if (!email) return;
  const apiBase = (cfg.apiBase && cfg.apiBase.en) || "https://app.pongrabbit.com";
  try {
    const res = await fetch(apiBase + "/api/v1/publishkit/checkout", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email: email.trim() }),
    });
    const json = await res.json();
    if (!res.ok || !json.ok) {
      throw new Error(json.message || "Checkout failed");
    }
    const txnId =
      json.transactionId ||
      pkExtractPtxn(json.checkoutUrl);
    if (!txnId) throw new Error("Missing checkout session");
    await pkOpenPaddleOverlay(txnId);
  } catch (err) {
    const support = cfg.contactEmail.en;
    alert(
      "Checkout is temporarily unavailable. Email " +
        support +
        " for Pro purchase.\n\n" +
        (err && err.message ? err.message : "")
    );
  }
}

async function pkCheckoutCn() {
  const cfg = window.PK_SITE;
  const email = window.prompt("请输入邮箱，支付成功后激活码会发到这里：", "");
  if (!email) return;
  const apiBase = (cfg.apiBase && cfg.apiBase.zh) || "https://api.pongrabbit.cn";
  try {
    const res = await fetch(apiBase + "/api/v1/publishkit/orders", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email: email.trim() }),
    });
    const json = await res.json();
    if (!res.ok || !json.ok) {
      throw new Error(json.message || "下单失败");
    }
    const orderId = json.orderId;
    const codeUrl = json.codeUrl;
    if (!orderId || !codeUrl) throw new Error("未返回微信支付码");
    await pkShowWxPay(apiBase, orderId, codeUrl, email.trim());
  } catch (err) {
    const support = cfg.contactEmail.zh;
    alert(
      "微信支付暂不可用。请发邮件至 " +
        support +
        " 咨询 Pro。\n\n" +
        (err && err.message ? err.message : "")
    );
  }
}

function pkWxPayRoot() {
  let el = document.getElementById("pk-wx-pay");
  if (el) return el;
  el = document.createElement("div");
  el.id = "pk-wx-pay";
  el.setAttribute("role", "dialog");
  el.setAttribute("aria-modal", "true");
  el.style.cssText =
    "position:fixed;inset:0;z-index:130;display:flex;align-items:center;justify-content:center;background:rgba(48,49,45,.4);backdrop-filter:blur(4px);padding:1rem;";
  el.innerHTML =
    '<div style="width:min(22rem,100%);background:#fbf9f4;border-radius:1rem;padding:1.25rem 1.25rem 1rem;box-shadow:0 16px 40px rgba(0,0,0,.18);text-align:center;">' +
    '<p style="margin:0 0 .5rem;font-size:16px;font-weight:600;">微信扫码支付 ¥139</p>' +
    '<p style="margin:0 0 1rem;font-size:13px;color:#4e4638;">请使用微信扫描二维码。付完后激活码会发到你填的邮箱。</p>' +
    '<div id="pk-wx-pay-qr" style="display:flex;justify-content:center;align-items:center;width:100%;">' +
    '<canvas id="pk-wx-pay-canvas" width="220" height="220" style="display:block;width:220px;height:220px;margin:0 auto;background:#fff;"></canvas>' +
    "</div>" +
    '<p id="pk-wx-pay-status" style="margin:0.85rem 0 0;font-size:13px;color:#5f5e59;">等待支付…</p>' +
    '<button type="button" id="pk-wx-pay-close" style="margin-top:0.85rem;padding:0.45rem 0.9rem;border:1px solid #d1c5b3;border-radius:0.5rem;background:#fff;cursor:pointer;font-size:13px;">关闭</button>' +
    "</div>";
  document.body.appendChild(el);
  return el;
}

let pkWxPollTimer = null;

function pkStopWxPoll() {
  if (pkWxPollTimer) {
    clearInterval(pkWxPollTimer);
    pkWxPollTimer = null;
  }
}

function pkHideWxPay() {
  pkStopWxPoll();
  const el = document.getElementById("pk-wx-pay");
  if (el) el.style.display = "none";
}

async function pkRenderWxQr(codeUrl) {
  const canvas = document.getElementById("pk-wx-pay-canvas");
  if (!canvas) return;
  try {
    if (!(window.QRCode && typeof window.QRCode.toCanvas === "function")) {
      await new Promise((resolve, reject) => {
        const s = document.createElement("script");
        s.src = "https://cdn.jsdelivr.net/npm/qrcode@1.5.3/build/qrcode.min.js";
        s.onload = resolve;
        s.onerror = () => reject(new Error("qrcode"));
        document.head.appendChild(s);
      });
    }
    await window.QRCode.toCanvas(canvas, codeUrl, { width: 220, margin: 1 });
    canvas.style.display = "block";
    canvas.style.margin = "0 auto";
  } catch (_) {
    const img = document.createElement("img");
    img.width = 220;
    img.height = 220;
    img.alt = "微信支付码";
    img.style.cssText = "display:block;width:220px;height:220px;margin:0 auto;";
    img.src =
      "https://api.qrserver.com/v1/create-qr-code/?size=220x220&data=" +
      encodeURIComponent(codeUrl);
    canvas.replaceWith(img);
    img.id = "pk-wx-pay-canvas";
  }
}

async function pkShowWxPay(apiBase, orderId, codeUrl, email) {
  const root = pkWxPayRoot();
  root.style.display = "flex";
  const closeBtn = document.getElementById("pk-wx-pay-close");
  if (closeBtn) closeBtn.onclick = pkHideWxPay;
  root.onclick = (e) => {
    if (e.target === root) pkHideWxPay();
  };
  await pkRenderWxQr(codeUrl);
  const statusEl = document.getElementById("pk-wx-pay-status");
  pkStopWxPoll();
  const tick = async () => {
    try {
      const res = await fetch(apiBase + "/api/v1/publishkit/orders/" + encodeURIComponent(orderId));
      const json = await res.json();
      if (!res.ok || !json.ok) return;
      if (json.status === "paid") {
        pkStopWxPoll();
        const key = json.licenseKey;
        if (statusEl) {
          statusEl.style.color = "#1b1c19";
          statusEl.textContent = key
            ? "已支付。激活码：" + key + "（也已发到 " + email + "）"
            : "已支付，正在发码到 " + email + "…";
        }
        pkNotice(
          key
            ? "支付成功。发稿匣 → 设置 → 许可证 粘贴激活码。"
            : "支付成功，激活码将发到邮箱。"
        );
      }
    } catch (_) { /* keep polling */ }
  };
  pkWxPollTimer = setInterval(tick, 2500);
  tick();
}

function pkExtractPtxn(url) {
  if (!url) return "";
  try {
    return new URL(url, window.location.origin).searchParams.get("_ptxn") || "";
  } catch (_) {
    const m = /[?&]_ptxn=([^&]+)/.exec(url);
    return m ? decodeURIComponent(m[1]) : "";
  }
}

function pkNotice(msg) {
  let el = document.getElementById("pk-pay-notice");
  if (!el) {
    el = document.createElement("div");
    el.id = "pk-pay-notice";
    el.setAttribute("role", "status");
    el.style.cssText =
      "position:fixed;left:50%;top:4.5rem;transform:translateX(-50%);z-index:80;max-width:32rem;padding:0.75rem 1rem;border-radius:0.5rem;background:#1b1c19;color:#fbf9f4;font-size:13px;line-height:1.4;box-shadow:0 8px 24px rgba(0,0,0,.18);";
    document.body.appendChild(el);
  }
  el.textContent = msg;
}

let pkPaddleScriptPromise = null;
let pkPaddleInited = false;

function pkLoadPaddleScript() {
  if (window.Paddle && window.Paddle.Checkout) return Promise.resolve();
  if (pkPaddleScriptPromise) return pkPaddleScriptPromise;
  pkPaddleScriptPromise = new Promise((resolve, reject) => {
    const s = document.createElement("script");
    s.src = "https://cdn.paddle.com/paddle/v2/paddle.js";
    s.onload = () => resolve();
    s.onerror = () => reject(new Error("Paddle.js failed to load. Check your network or ad blocker."));
    document.head.appendChild(s);
  });
  return pkPaddleScriptPromise;
}

async function pkEnsurePaddle() {
  await pkLoadPaddleScript();
  if (pkPaddleInited) return;
  const cfg = window.PK_SITE;
  const apiBase = (cfg.apiBase && cfg.apiBase.en) || "https://app.pongrabbit.com";
  const res = await fetch(apiBase + "/api/v1/auth/intl/config");
  const data = await res.json();
  const paddle = data.paddle || {};
  if (!res.ok || !data.ok || !paddle.enabled || !paddle.clientToken) {
    throw new Error("Paddle client token is not configured.");
  }
  const env = paddle.environment === "production" ? "production" : "sandbox";
  if (window.Paddle.Environment && typeof window.Paddle.Environment.set === "function") {
    window.Paddle.Environment.set(env);
  }
  const successUrl = "https://get.pongrabbit.com/en/?pkpaid=1";
  window.Paddle.Initialize({
    token: paddle.clientToken,
    checkout: {
      settings: { displayMode: "overlay", theme: "light", locale: "en", successUrl },
    },
    eventCallback: function (event) {
      if (event && event.name === "checkout.completed") {
        pkNotice("Payment received. The license key will be emailed to you. Paste it in PublishKit → Settings → License.");
      }
    },
  });
  pkPaddleInited = true;
}

async function pkOpenPaddleOverlay(txnId) {
  await pkEnsurePaddle();
  pkNotice("Complete payment in the Paddle window…");
  window.Paddle.Checkout.open({
    transactionId: txnId,
    settings: {
      displayMode: "overlay",
      theme: "light",
      locale: "en",
      successUrl: "https://get.pongrabbit.com/en/?pkpaid=1",
    },
  });
}

function pkInitFaq() {
  document.querySelectorAll(".faq-toggle").forEach((button) => {
    button.addEventListener("click", () => {
      const content = button.nextElementSibling;
      const icon = button.querySelector(".material-symbols-outlined");
      const isHidden = content.classList.contains("hidden");

      document.querySelectorAll(".faq-content").forEach((c) => c.classList.add("hidden"));
      document.querySelectorAll(".faq-toggle .material-symbols-outlined").forEach((i) => {
        i.style.transform = "rotate(0deg)";
      });

      if (isHidden) {
        content.classList.remove("hidden");
        if (icon) icon.style.transform = "rotate(180deg)";
      }
    });
  });
}

function pkOpenProModal() {
  const modal = document.getElementById("pro-modal");
  const modalCard = document.getElementById("pro-modal-card");
  if (!modal || !modalCard) return;
  modal.classList.remove("hidden");
  modal.classList.add("flex");
  modal.setAttribute("aria-hidden", "false");
  requestAnimationFrame(() => {
    modalCard.classList.remove("scale-95", "opacity-0");
    modalCard.classList.add("scale-100", "opacity-100");
  });
  document.body.style.overflow = "hidden";
}

function pkCloseProModal() {
  const modal = document.getElementById("pro-modal");
  const modalCard = document.getElementById("pro-modal-card");
  if (!modal || !modalCard) return;
  modalCard.classList.remove("scale-100", "opacity-100");
  modalCard.classList.add("scale-95", "opacity-0");
  setTimeout(() => {
    modal.classList.remove("flex");
    modal.classList.add("hidden");
    modal.setAttribute("aria-hidden", "true");
    document.body.style.overflow = "";
  }, 200);
}

function pkInitProModal() {
  document.querySelectorAll(".pro-modal-trigger").forEach((btn) => {
    btn.addEventListener("click", (e) => {
      e.preventDefault();
      pkOpenProModal();
    });
  });
  ["close-modal-btn", "dismiss-modal-btn"].forEach((id) => {
    const el = document.getElementById(id);
    if (el) el.addEventListener("click", pkCloseProModal);
  });
  const modal = document.getElementById("pro-modal");
  if (modal) {
    modal.addEventListener("click", (e) => {
      if (e.target === modal) pkCloseProModal();
    });
  }
  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape" && modal && !modal.classList.contains("hidden")) {
      pkCloseProModal();
    }
  });
  const checkoutBtn = document.getElementById("checkout-btn");
  if (checkoutBtn) {
    checkoutBtn.addEventListener("click", () => {
      pkCheckout();
      pkCloseProModal();
    });
  }
}

function pkInitDownload() {
  document.querySelectorAll(".download-trigger").forEach((btn) => {
    btn.addEventListener("click", (e) => {
      e.preventDefault();
      pkDownload("windows");
    });
  });
  document.querySelectorAll(".download-mac-trigger").forEach((btn) => {
    btn.addEventListener("click", (e) => {
      e.preventDefault();
      pkDownload("mac");
    });
  });
  document.querySelectorAll(".download-mac-intel-trigger").forEach((btn) => {
    btn.addEventListener("click", (e) => {
      e.preventDefault();
      pkDownload("mac-intel");
    });
  });
}

function pkInitChromeStore() {
  const url = window.PK_SITE.chromeWebStoreUrl;
  if (!url) return;
  document.querySelectorAll(".chrome-store-link").forEach((el) => {
    if (el.tagName === "A" && (!el.getAttribute("href") || el.getAttribute("href") === "#")) {
      el.setAttribute("href", url);
    }
  });
  document.querySelectorAll(".chrome-store-trigger").forEach((btn) => {
    btn.addEventListener("click", (e) => {
      e.preventDefault();
      pkChromeStore();
    });
  });
}

document.addEventListener("DOMContentLoaded", () => {
  pkApplyPricing();
  pkInitFaq();
  pkInitProModal();
  pkInitDownload();
  pkInitChromeStore();
  const params = new URLSearchParams(window.location.search);
  if (params.get("pkpaid") === "1") {
    pkNotice(
      window.PK_SITE.locale === "zh"
        ? "支付成功。激活码会发到邮箱。请在发稿匣 → 设置 → 许可证 粘贴。"
        : "Payment received. The license key will be emailed to you. Paste it in PublishKit → Settings → License."
    );
  } else if (window.PK_SITE.locale === "en" && params.get("_ptxn")) {
    pkOpenPaddleOverlay(params.get("_ptxn")).catch((err) => {
      alert(err && err.message ? err.message : "Checkout failed");
    });
  }
});
