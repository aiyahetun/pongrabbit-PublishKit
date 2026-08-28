const DEFAULT_PORT = 17345;

const STRINGS = {
  "zh-CN": {
    hint: "在桌面端「设置 → 浏览器插件」复制端口与 Token。",
    portLabel: "API 端口",
    tokenLabel: "配对 Token",
    tokenPlaceholder: "粘贴配对 Token",
    save: "保存",
    test: "测试连接",
    loadToday: "加载今日待发",
    refreshTasks: "刷新列表",
    connecting: "连接中…",
    connected: "已连接",
    loadingTasks: "加载任务…",
    noTasks: "暂无待发任务",
    loadedTasks: "已加载 {count} 条待发任务",
    configSaved: "配置已保存",
    tokenMissing: "请先在桌面端「设置 → 浏览器插件」复制配对 Token",
    copyBody: "复制富文本",
    copyDone: "富文本已复制，可直接粘贴到编辑器",
    copyFailed: "复制失败",
    copyImage: "复制单张配图",
    copyImageDone: "配图已复制，到平台编辑器 Ctrl+V 粘贴",
    copyImageFailed: "复制配图失败",
    stageImages: "复制多图打开配图文件夹",
    stageImagesDone: "已打开 {count} 张配图文件夹",
    stageImagesFailed: "打开配图失败",
    urlPlaceholder: "发布链接",
    useTabUrl: "当前页",
    markPublished: "标记已发布",
    publishedDone: "已标记发布",
    publishedFailed: "标记失败",
    undoPublish: "撤销发布",
    undoDone: "已撤销，任务恢复待发",
    undoFailed: "撤销失败",
    duplicatePublishTitle: "重复发布提醒",
    duplicatePublishConfirm: "「{title}」在 {channel} 已于 {when}（{days} 天前）标记发布。{detail}仍要再次标记已发布吗？",
    duplicatePublishWithUrl: "上次链接：{url}。",
    duplicatePublishNoUrl: "上次未填写链接。",
  },
  en: {
    hint: "Copy port and token from desktop Settings → Browser extension.",
    portLabel: "API port",
    tokenLabel: "Pairing token",
    tokenPlaceholder: "Paste pairing token",
    save: "Save",
    test: "Test connection",
    loadToday: "Load today's tasks",
    refreshTasks: "Refresh list",
    connecting: "Connecting…",
    connected: "Connected",
    loadingTasks: "Loading tasks…",
    noTasks: "No ready tasks",
    loadedTasks: "Loaded {count} ready task(s)",
    configSaved: "Settings saved",
    tokenMissing: "Copy the pairing token from desktop Settings → Browser extension",
    copyBody: "Copy rich text",
    copyDone: "Rich text copied — paste into the editor",
    copyFailed: "Copy failed",
    copyImage: "Copy single image",
    copyImageDone: "Image copied — paste with Ctrl+V in the editor",
    copyImageFailed: "Image copy failed",
    stageImages: "Copy multiple & open folder",
    stageImagesDone: "Opened folder with {count} image(s)",
    stageImagesFailed: "Could not open images",
    urlPlaceholder: "Published URL",
    useTabUrl: "This tab",
    markPublished: "Mark published",
    publishedDone: "Marked as published",
    publishedFailed: "Publish failed",
    undoPublish: "Undo publish",
    undoDone: "Reverted to ready",
    undoFailed: "Undo failed",
    duplicatePublishTitle: "Duplicate publish warning",
    duplicatePublishConfirm: "\"{title}\" on {channel} was marked published on {when} ({days} days ago). {detail}Mark as published again?",
    duplicatePublishWithUrl: "Previous URL: {url}. ",
    duplicatePublishNoUrl: "No URL was saved last time. ",
  },
};

let uiLocale = "zh-CN";

function t(key, vars = {}) {
  const table = STRINGS[uiLocale] ?? STRINGS.en;
  let text = table[key] ?? STRINGS.en[key] ?? key;
  for (const [name, value] of Object.entries(vars)) {
    text = text.replace(`{${name}}`, String(value));
  }
  return text;
}

function applyLocale() {
  document.getElementById("hint").textContent = t("hint");
  document.querySelector('label[for="port"]').textContent = t("portLabel");
  document.querySelector('label[for="token"]').textContent = t("tokenLabel");
  document.getElementById("token").placeholder = t("tokenPlaceholder");
  document.getElementById("save").textContent = t("save");
  document.getElementById("test").textContent = t("test");
  document.getElementById("today").textContent = t("loadToday");
}

async function loadConfig() {
  const stored = await chrome.storage.local.get(["apiPort", "pairingToken"]);
  document.getElementById("port").value = stored.apiPort ?? DEFAULT_PORT;
  document.getElementById("token").value = stored.pairingToken ?? "";
}

async function saveConfig() {
  const apiPort = Number(document.getElementById("port").value) || DEFAULT_PORT;
  const pairingToken = document.getElementById("token").value.trim();
  await chrome.storage.local.set({ apiPort, pairingToken });
  setStatus(t("configSaved"), "ok");
}

function setStatus(text, kind = "") {
  const el = document.getElementById("status");
  el.textContent = text;
  el.className = kind;
}

async function apiFetch(path, options = {}) {
  const stored = await chrome.storage.local.get(["apiPort", "pairingToken"]);
  const port = stored.apiPort ?? DEFAULT_PORT;
  const token = stored.pairingToken ?? "";
  if (!token) {
    throw new Error(t("tokenMissing"));
  }
  const response = await fetch(`http://127.0.0.1:${port}${path}`, {
    ...options,
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
      ...(options.headers ?? {}),
    },
  });
  const data = await response.json().catch(() => ({}));
  if (!response.ok) {
    throw new Error(data?.error?.message ?? `HTTP ${response.status}`);
  }
  return data;
}

async function syncUiLocale() {
  try {
    const data = await apiFetch("/settings/ui-locale");
    if (data.uiLocale === "en" || data.uiLocale === "zh-CN") {
      uiLocale = data.uiLocale;
      applyLocale();
    }
  } catch {
    applyLocale();
  }
}

async function testConnection() {
  setStatus(t("connecting"));
  try {
    const data = await apiFetch("/health");
    uiLocale = data.uiLocale === "en" ? "en" : "zh-CN";
    applyLocale();
    setStatus(`${t("connected")} · v${data.version}`, "ok");
    await loadToday();
  } catch (error) {
    setStatus(String(error), "error");
  }
}

async function copyRichText(html, plain) {
  const fragment = `<!DOCTYPE html><html><body><!--StartFragment-->${html}<!--EndFragment--></body></html>`;
  if (typeof ClipboardItem !== "undefined" && navigator.clipboard.write) {
    await navigator.clipboard.write([
      new ClipboardItem({
        "text/html": new Blob([fragment], { type: "text/html" }),
        "text/plain": new Blob([plain], { type: "text/plain" }),
      }),
    ]);
    return;
  }
  await navigator.clipboard.writeText(plain);
}

async function copyTaskBody(taskId) {
  const data = await apiFetch(`/tasks/${encodeURIComponent(taskId)}/prepare`, { method: "POST" });
  if (data.copyMode === "plain") {
    await navigator.clipboard.writeText(data.bodyPlain ?? data.body ?? "");
    return;
  }
  await copyRichText(data.bodyHtml ?? data.body ?? "", data.bodyPlain ?? data.body ?? "");
}

async function copyTaskImage(taskId) {
  return apiFetch(`/tasks/${encodeURIComponent(taskId)}/copy-image`, { method: "POST" });
}

async function stageTaskImages(taskId) {
  return apiFetch(`/tasks/${encodeURIComponent(taskId)}/stage-images`, { method: "POST" });
}

async function getActiveTabUrl() {
  const tabs = await chrome.tabs.query({ active: true, currentWindow: true });
  return tabs[0]?.url ?? "";
}

async function undoTaskPublish(taskId) {
  await apiFetch(`/tasks/${encodeURIComponent(taskId)}/unpublish`, { method: "POST" });
}

async function fetchDuplicateWarning(taskId) {
  return apiFetch(`/tasks/${encodeURIComponent(taskId)}/duplicate-publish-warning`);
}

async function confirmPublishIfDuplicate(task) {
  const warning = await fetchDuplicateWarning(task.id);
  if (!warning) return true;
  const when = warning.previousPublishedAt.slice(0, 10);
  const detail = warning.previousPublishUrl
    ? t("duplicatePublishWithUrl", { url: warning.previousPublishUrl })
    : t("duplicatePublishNoUrl");
  return confirm(
    t("duplicatePublishConfirm", {
      title: task.contentTitle,
      channel: task.channelName,
      when,
      days: warning.daysSince,
      detail,
    })
  );
}

async function markTaskPublished(taskId, urlInput) {
  const url = urlInput.value.trim() || (await getActiveTabUrl());
  await apiFetch(`/tasks/${encodeURIComponent(taskId)}/publish`, {
    method: "POST",
    body: JSON.stringify({ url: url || undefined }),
  });
}

function renderTaskItem(task) {
  const item = document.createElement("li");
  item.className = "task-item";

  const title = document.createElement("strong");
  title.textContent = task.contentTitle;
  item.appendChild(title);

  const channel = document.createElement("span");
  channel.textContent = task.channelName;
  item.appendChild(channel);

  const actions = document.createElement("div");
  actions.className = "task-actions";

  const copyBtn = document.createElement("button");
  copyBtn.type = "button";
  copyBtn.textContent = t("copyBody");
  copyBtn.addEventListener("click", async () => {
    setStatus(t("connecting"));
    try {
      await copyTaskBody(task.id);
      setStatus(t("copyDone"), "ok");
    } catch (error) {
      setStatus(`${t("copyFailed")}: ${error}`, "error");
    }
  });
  actions.appendChild(copyBtn);

  const imageBtn = document.createElement("button");
  imageBtn.type = "button";
  imageBtn.textContent = t("copyImage");
  imageBtn.addEventListener("click", async () => {
    setStatus(t("connecting"));
    try {
      await copyTaskImage(task.id);
      setStatus(t("copyImageDone"), "ok");
    } catch (error) {
      setStatus(`${t("copyImageFailed")}: ${error}`, "error");
    }
  });
  actions.appendChild(imageBtn);

  const folderBtn = document.createElement("button");
  folderBtn.type = "button";
  folderBtn.textContent = t("stageImages");
  folderBtn.addEventListener("click", async () => {
    setStatus(t("connecting"));
    try {
      const result = await stageTaskImages(task.id);
      setStatus(t("stageImagesDone", { count: result.copiedCount }), "ok");
    } catch (error) {
      setStatus(`${t("stageImagesFailed")}: ${error}`, "error");
    }
  });
  actions.appendChild(folderBtn);

  const urlInput = document.createElement("input");
  urlInput.type = "url";
  urlInput.className = "url-input";
  urlInput.placeholder = t("urlPlaceholder");
  if (task.publishUrl) {
    urlInput.value = task.publishUrl;
  }
  actions.appendChild(urlInput);

  const tabBtn = document.createElement("button");
  tabBtn.type = "button";
  tabBtn.textContent = t("useTabUrl");
  tabBtn.addEventListener("click", async () => {
    urlInput.value = await getActiveTabUrl();
  });
  actions.appendChild(tabBtn);

  const publishBtn = document.createElement("button");
  publishBtn.type = "button";
  publishBtn.className = "primary";
  publishBtn.textContent = t("markPublished");
  publishBtn.addEventListener("click", async () => {
    setStatus(t("connecting"));
    try {
      if (!(await confirmPublishIfDuplicate(task))) {
        setStatus("", "");
        return;
      }
      await markTaskPublished(task.id, urlInput);
      setStatus(t("publishedDone"), "ok");
      await loadToday();
    } catch (error) {
      setStatus(`${t("publishedFailed")}: ${error}`, "error");
    }
  });
  actions.appendChild(publishBtn);

  const undoBtn = document.createElement("button");
  undoBtn.type = "button";
  undoBtn.className = "undo";
  undoBtn.textContent = t("undoPublish");
  undoBtn.addEventListener("click", async () => {
    setStatus(t("connecting"));
    try {
      await undoTaskPublish(task.id);
      setStatus(t("undoDone"), "ok");
      await loadToday();
    } catch (error) {
      setStatus(`${t("undoFailed")}: ${error}`, "error");
    }
  });
  actions.appendChild(undoBtn);

  item.appendChild(actions);
  return item;
}

async function loadToday() {
  setStatus(t("loadingTasks"));
  try {
    const tasks = await apiFetch("/tasks/today");
    const list = document.getElementById("tasks");
    list.innerHTML = "";
    if (!tasks.length) {
      const empty = document.createElement("li");
      empty.className = "empty";
      empty.textContent = t("noTasks");
      list.appendChild(empty);
      setStatus(t("noTasks"), "ok");
      return;
    }
    for (const task of tasks) {
      list.appendChild(renderTaskItem(task));
    }
    setStatus(t("loadedTasks", { count: tasks.length }), "ok");
  } catch (error) {
    setStatus(String(error), "error");
  }
}

document.getElementById("save").addEventListener("click", saveConfig);
document.getElementById("test").addEventListener("click", testConnection);
document.getElementById("today").addEventListener("click", loadToday);

document.addEventListener("visibilitychange", () => {
  if (!document.hidden) {
    loadToday().catch(() => {});
  }
});

(async () => {
  applyLocale();
  await loadConfig();
  await syncUiLocale();
  const stored = await chrome.storage.local.get(["pairingToken"]);
  if (stored.pairingToken) {
    await loadToday().catch(() => {});
  }
})();
