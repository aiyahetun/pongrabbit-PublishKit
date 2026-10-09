# PublishKit / 发稿匣 · 站外营销文案包

| 属性 | 内容 |
|------|------|
| 版本 | v1.0（2026-10-03） |
| 用途 | 各平台发帖、目录提交、投稿时 **直接复制**；视频/GIF 自行录制后附在帖内 |
| 维护 | 改价、改链接时同步更新本文「固定链接」与对应段落 |

---

## 1. 固定链接（无 UTM）

发帖时务必用 **带 UTM 的版本**（见 §2），内部对账用下表核对最终落地页。

| 名称 | URL |
|------|-----|
| 官网（英文） | https://get.pongrabbit.com/en/ |
| 官网（中文） | https://get.pongrabbit.cn/zh/ |
| 隐私政策（英文站） | https://get.pongrabbit.com/privacy.html |
| 隐私政策（中文站） | https://get.pongrabbit.cn/privacy.html |
| GitHub 仓库 | https://github.com/aiyahetun/pongrabbit-PublishKit |
| 桌面端最新版下载 | https://github.com/aiyahetun/pongrabbit-PublishKit/releases/latest |
| Chrome 插件（发稿匣助手） | https://chromewebstore.google.com/detail/publishkit-companion/dijkbipanbpkgndbhonecpfnladaiipj |
| 联系（海外） | mailto:support@pongrabbit.com |
| 联系（国内） | mailto:support@pongrabbit.cn |

**定价事实（全渠道一致）**

- Free：50 条内容条目，核心闭环可用  
- Pro 买断：海外 **$38** 首发（划线 $79）· 国内 **¥139** 首发（划线 ¥299）· 非订阅  
- 激活码格式：`PKPRO-` · 付款后邮件发送 · 在桌面端「设置 → 许可证」激活  

---

## 2. UTM 规范与链接表

### 2.1 参数约定

| 参数 | 取值示例 | 说明 |
|------|----------|------|
| `utm_source` | `reddit` / `indiehackers` / `producthunt` / `zhihu` / `alternativeto` | 平台名，小写 |
| `utm_medium` | `social` / `community` / `directory` / `launch` | 渠道类型 |
| `utm_campaign` | `launch_2026` / `sideproject_post` / `compare_buffer` | 同一活动内保持一致 |
| `utm_content` | `en_post` / `cn_post` / `comment_link` | 可选，区分同一帖内多链接 |

**拼接规则**：在落地页 URL 后加 `?utm_source=...&utm_medium=...&utm_campaign=...`（已有查询参数则用 `&` 接续）。

### 2.2 常用落地链接（复制即用）

**英文官网（海外帖默认）**

```text
https://get.pongrabbit.com/en/?utm_source=SOURCE&utm_medium=medium&utm_campaign=launch_2026
```

**中文官网（国内帖默认）**

```text
https://get.pongrabbit.cn/zh/?utm_source=SOURCE&utm_medium=medium&utm_campaign=launch_2026
```

**GitHub Releases（强调下载）**

```text
https://github.com/aiyahetun/pongrabbit-PublishKit/releases/latest?utm_source=SOURCE&utm_medium=medium&utm_campaign=launch_2026
```

**Chrome 插件（CWS 不支持自定义 UTM，用官网中转说明）**  
帖内写插件直链 + 官网 UTM；或在官网落地页已有商店按钮。

```text
https://chromewebstore.google.com/detail/publishkit-companion/dijkbipanbpkgndbhonecpfnladaiipj
```

### 2.3 按平台替换 `SOURCE` / `medium`

| 平台 | utm_source | utm_medium | 建议 campaign |
|------|------------|------------|----------------|
| Reddit | `reddit` | `community` | `sideproject_post` 或 `r_SUBREDDIT` |
| Indie Hackers | `indiehackers` | `community` | `launch_2026` |
| Product Hunt | `producthunt` | `launch` | `ph_launch_YYYYMMDD` |
| Hacker News | `hackernews` | `community` | `show_hn_YYYYMMDD` |
| AlternativeTo | `alternativeto` | `directory` | `directory_2026` |
| SaaSHub / BetaList 等 | `saas hub` 等（无空格用 `saashub`） | `directory` | `directory_2026` |
| 知乎 | `zhihu` | `social` | `zh_compare_2026` |
| 掘金 | `juejin` | `social` | `zh_tutorial_2026` |
| 少数派 / Appinn | `sspai` / `appinn` | `pr` | `cn_review_2026` |
| LinkedIn | `linkedin` | `social` | `founder_post` |
| X (Twitter) | `twitter` | `social` | `founder_post` |

---

## 3. 关联词 / 关键词（SEO & 发帖标签）

### 3.1 英文 — 建议使用

**品类（title/H1/首句）**  
`content asset management` · `publish management` · `content operations` · `publish ledger` · `cross-border content`

**差异化（正文必带 1～2 个）**  
`local-first` · `on your device` · `no OAuth` · `no auto-posting` · `manual publish` · `SQLite`

**长尾（对比帖/目录描述）**  
`Buffer alternative without scheduling` · `local content calendar` · `publish tracker` · `not a social media scheduler`

**避免主动优化（仅 FAQ 澄清时用）**  
`social media scheduler` · `auto post` · `post to all platforms` · `Hootsuite alternative`（除非标题明确写「不是 Hootsuite」）

**Reddit / IH 可选 flair 语境词**  
`side project` · `indie` · `desktop app` · `productivity` · `content marketing workflow`

### 3.2 中文 — 建议使用

**品类**  
`内容资产管理` · `发布管理系统` · `发布台账` · `内容运营工具` · `跨境内容`

**差异化**  
`本地优先` · `不做账号托管` · `不做一键分发` · `手动发布` · `浏览器插件回填状态`

**长尾**  
`自媒体发布台账` · `桌面内容管理` · `和蚁小二的区别` · `不做矩阵分发`

**避免**  
`新媒体矩阵` · `一键全网分发` · `社媒调度器`（大词误触达）

### 3.3 统一实体定义句（中英各贴一次）

**中文**  
发稿匣 PublishKit 是本地优先的桌面内容资产与发布管理系统，管理文案、素材、渠道与发布状态；不托管平台账号，不自动发帖。

**English**  
PublishKit is a local-first desktop app for content assets and publish status. You publish manually in the browser; the companion extension syncs links and status over localhost only. No OAuth, no auto-posting.

---

## 4. 通用文案三档（全平台剪贴基础）

### 4.1 一句话（≤60 英文字符 / ≤35 汉字）

**EN**  
Local publish ledger: draft on desktop, post yourself, sync status via Chrome.

**CN**  
本地发稿台账：桌面管内容与渠道，浏览器自己发，插件回填链接与状态。

### 4.2 短描述（约 80～260 字 / 英文 400～600 字符）

**EN**  
PublishKit helps cross-border and multi-channel teams manage copy, media, channels, and **whether each piece is actually published**. Everything stays in a local SQLite database on your PC—no platform passwords in our cloud. Install the Windows app, pair the free Chrome extension (localhost only), copy channel-aware drafts, paste into X/LinkedIn/公众号/任意平台, then mark published with the post URL. Free for 50 items; Pro lifetime $38 (launch). Not Buffer, not a matrix auto-poster.

**CN**  
发稿匣面向跨境、多渠道内容运营：把文案、素材、渠道和「发没发、链接在哪」放在同一套**本地桌面**工作流里。数据在 SQLite，不上传正文到我们的服务器，也不托管微博/公众号/LinkedIn 等账号。配合 Chrome 插件「发稿匣助手」（仅连接本机 127.0.0.1）：复制草稿 → 在网页里自己发帖 → 一键记链接与状态。免费 50 条；Pro 买断首发 ¥139。不是蚁小二式一键分发，也不是 Buffer 式自动调度。

### 4.3 长描述（目录站 / 投稿 / IH 正文骨架，可分段）

**EN — sections**  
- **Problem:** Spreadsheets and Notion break when you need per-channel copy, assets, and proof of publish.  
- **Solution:** PublishKit desktop + Companion extension.  
- **How it works:** (1) Prepare tasks in desktop (2) Copy rich text in side panel (3) Publish natively in browser (4) Attach tab URL & mark done.  
- **Privacy:** Local-first, extension talks only to 127.0.0.1.  
- **Pricing:** Free tier + Pro lifetime.  
- **Links:** Website · GitHub · Chrome Web Store (paste from §1).

**CN — sections**  
- **痛点：** 表格/飞书管不住多渠道版本、配图和发布状态。  
- **方案：** 桌面端 + 浏览器侧栏插件。  
- **流程：** 桌面排期与待发 → 插件复制 → 各平台手动发布 → 回填 URL。  
- **边界：** 不 OAuth、不代发、不矩阵。  
- **价格：** 免费版 + Pro 买断。  
- **链接：** 见 §1 中文站、GitHub、Chrome 商店。

---

## 5. Reddit

**关联词：** `local-first` · `side project` · `content workflow` · `cross-border` · `no auto post`  
**链接：** 英文帖用 [EN 官网 UTM](#22-常用落地链接) `utm_source=reddit`；帖内第二条链可用 GitHub Releases 或 CWS。

### 5.1 r/SideProject（推荐首发）

**Title**  
I built a local-first publish ledger (desktop + Chrome extension)—you post manually, it tracks status

**Body**

```markdown
Hey r/SideProject,

I kept losing track of what went live on which channel—different copy per platform, links scattered in chat history, spreadsheets drifting out of date.

So I shipped **PublishKit**: a Windows desktop app (local SQLite) for content + channels + publish status, plus a **Chrome side panel** that only talks to `127.0.0.1` on your machine.

**Workflow**
1. Queue “ready to publish” in the desktop app  
2. Extension: copy draft → you paste into X / LinkedIn / whatever  
3. Click “this tab” for the URL → mark published  

**What it is NOT**  
No OAuth, no Buffer-style scheduling, no “post to all platforms.”

**Links**  
- Site: https://get.pongrabbit.com/en/?utm_source=reddit&utm_medium=community&utm_campaign=sideproject_post  
- Desktop: https://github.com/aiyahetun/pongrabbit-PublishKit/releases/latest?utm_source=reddit&utm_medium=community&utm_campaign=sideproject_post  
- Extension: https://chromewebstore.google.com/detail/publishkit-companion/dijkbipanbpkgndbhonecpfnladaiipj  

Free for 50 items; Pro is a **lifetime** license ($38 launch). Would love harsh feedback—especially what would make you switch from Notion/sheets.
```

**Comment 备用（有人问隐私）**  
Extension never sends your drafts to our servers—only localhost API. Privacy: https://get.pongrabbit.com/privacy.html

### 5.2 r/Entrepreneur 或 r/content_marketing（偏场景）

**Title**  
How we track multi-channel publishing without giving tools our social passwords

**Body 要点**（自行缩短）  
- 跨境/多账号手动发的团队痛点  
- 为何不用 SaaS 调度器（风控、账号安全）  
- PublishKit 边界 + 三链接（同 UTM，`utm_campaign=content_marketing`）

### 5.3 r/China 或 r/digitalnomad（跨境叙事）

**Title**  
Local desktop tool for cross-border content ops (CN + EN sites, manual publish + status sync)

**Body**  
强调 bilingual site、manual publish、extension on CWS；链接同 EN 官网 UTM，`utm_source=reddit`.

**版规提醒**  
每 sub 发帖前读规则；10% 自推比例；首评可放「我是作者，欢迎喷」。

---

## 6. Indie Hackers

**关联词：** `bootstrapped` · `lifetime deal` · `desktop` · `local-first` · `indie hacker`  
**链接：** EN 官网 UTM `utm_source=indiehackers&utm_medium=community&utm_campaign=launch_2026`

**Post title**  
Launching PublishKit: a local publish ledger for teams who post manually

**Post body**

```markdown
### What is it?
PublishKit (发稿匣) is a Windows desktop app + Chrome extension for **content assets and publish status**—not another social scheduler.

### The problem
If you publish to several channels (especially cross-border), you need different copy, assets, and a reliable record of **what actually went live**. Cloud schedulers want OAuth; matrix tools want your accounts. We wanted: **you publish in the browser, we help you stay organized.**

### What we built
- Local SQLite on your PC  
- Tasks, channels, calendar, duplicate-publish warnings (Pro)  
- Companion extension: copy draft, grab tab URL, mark published—**localhost only**

### Business model
Freemium → Pro **lifetime** ($38 / ¥139 launch). No MRR yet; testing willingness to pay.

### Links
https://get.pongrabbit.com/en/?utm_source=indiehackers&utm_medium=community&utm_campaign=launch_2026  
https://github.com/aiyahetun/pongrabbit-PublishKit  

### Ask
1. Do you still post manually for any platforms?  
2. What would make you pay once vs subscribe?  
```

---

## 7. Product Hunt

**关联词：** `productivity` · `content marketing` · `desktop app` · `chrome extension` · `local-first`  
**链接：** 官网 UTM `utm_source=producthunt&utm_medium=launch&utm_campaign=ph_launch_YYYYMMDD` · Gallery 用 `apps/extension/store/assets/screenshot_*.png`

| 字段 | 限制 | 文案 |
|------|------|------|
| **Name** | — | PublishKit |
| **Tagline** | **60 字符内** | Local publish ledger—desktop + Chrome, you post manually |
| **Description** | 较长，可分段 | 用 §4.3 EN，压缩为 PH 表单段落 |

**Maker 首条评论（发布后立刻贴）**

```markdown
Hi PH 👋 I'm the maker of PublishKit.

We built this because schedulers want your platform passwords, and "matrix" tools auto-post—we don't. PublishKit keeps copy, channels, and publish status on **your PC** (SQLite). The Chrome companion only talks to localhost.

**Try it**  
1) Download Windows app (GitHub release)  
2) Install [Companion on Chrome Web Store]  
3) Settings → paste pairing token → copy draft → publish yourself → mark done  

Launch pricing: **$38 lifetime** Pro (not subscription). Free tier: 50 items.

Honest question: what would you need to replace Notion + spreadsheets for publish tracking?

🔗 https://get.pongrabbit.com/en/?utm_source=producthunt&utm_medium=launch&utm_campaign=ph_launch_YYYYMMDD
```

**Gallery 图说明（可选 caption）**  
1. Side panel + today’s queue  
2. Copy rich text  
3. Mark published with URL  
4. Duplicate warning  
5. Desktop pairing  

---

## 8. Hacker News（Show HN）

**关联词：** `Show HN` · `local` · `sqlite` · `chrome extension`  
**链接：** GitHub 或官网；HN 习惯链 **https://get.pongrabbit.com/en/**（可不加 UTM 或仅 `utm_source=hackernews`）

**Title（≤80 字符为宜）**  
Show HN: PublishKit – local SQLite publish ledger + Chrome extension (no auto-posting)

**First comment**

```markdown
Hi HN — maker here.

PublishKit is a Windows desktop app for multi-channel content prep and **publish status**. Companion Chrome extension uses MV3 + side panel; API is only http://127.0.0.1 on the machine.

Not trying to be Buffer: no OAuth, no scheduled auto-posts. Target users: solo marketers / small cross-border teams who already publish manually.

Open repo: https://github.com/aiyahetun/pongrabbit-PublishKit  
Site: https://get.pongrabbit.com/en/?utm_source=hackernews&utm_medium=community&utm_campaign=show_hn

Happy to answer technical questions (Tauri, local API, extension permissions).
```

---

## 9. 启动目录 / 收录站（英文为主）

**关联词：** `content management` · `marketing tools` · `productivity` · `alternatives` · `local` · `publish tracker`  
**链接：** 官网 UTM `utm_source=alternativeto`（各站替换 source）· **Alternative URL** 字段可填 GitHub

### 9.1 超短（≤80 words，Twitter bio 亦可）

PublishKit is a local-first desktop app for content assets and publish status. Pair the free Chrome extension (localhost only) to copy drafts and mark posts published. No OAuth, no auto-posting. Free tier + lifetime Pro.

### 9.2 标准（AlternativeTo / SaaSHub / Slant）

**Name:** PublishKit  
**Website:** https://get.pongrabbit.com/en/?utm_source=alternativeto&utm_medium=directory&utm_campaign=directory_2026  
**Categories:** Productivity, Marketing, Content Management  
**Platforms:** Windows, Chrome Extension  

**Description:**  
PublishKit helps teams manage copy, media, channels, and whether each item is published—stored locally in SQLite. The PublishKit Companion browser extension connects only to the desktop app on 127.0.0.1: copy channel-aware text, attach the current tab URL, mark tasks published.  

**Alternatives / compares to:** Notion (general notes), Buffer/Later (schedulers—we are not), spreadsheets, Eagle (assets only—we add publish workflow).  

**Pricing:** Free (50 items); Pro lifetime license.

### 9.3 BetaList 风格（problem / solution）

**Problem:** Multi-channel publishing creates version chaos and weak proof of what went live.  
**Solution:** Local desktop ledger + Chrome side panel for manual publish workflow.  
**Link:** 同 §9.2 Website  

### 9.4 提交时勾选标签建议

`desktop` · `chrome-extension` · `content-marketing` · `privacy` · `indie` · `freemium` · `lifetime-deal`

---

## 10. 知乎

**关联词：** `跨境内容` · `自媒体运营` · `内容管理` · `本地软件` · `不做一键分发`  
**链接：** https://get.pongrabbit.cn/zh/?utm_source=zhihu&utm_medium=social&utm_campaign=zh_compare_2026  
**文末：** GitHub + Chrome 商店直链（CWS 无 UTM）

**标题（选一）**  
- 跨境内容运营，我为什么不用「一键分发」而用本地发布台账  
- 发稿匣 PublishKit 和蚁小二、Buffer 分别解决什么问题？  

**正文结构**

```markdown
## 先说结论
如果你要的是**把账号交给云端、自动往各平台发**，PublishKit **不适合**你。
如果你要的是**自己在各平台网页里发**，但想把文案、渠道、发没发、链接在哪**记在本地**，可以了解发稿匣。

## 我遇到的痛点
（此处写 2～3 段个人场景：多渠道、表格乱、链接散落）

## PublishKit 是什么
本地 SQLite 桌面端 + Chrome 插件「发稿匣助手」（只连本机 127.0.0.1）：
复制草稿 → 浏览器发帖 → 回填 URL → 状态回写桌面端。

## 和常见工具的区别
| | 蚁小二/矩阵 | Buffer 类调度 | 发稿匣 |
|--|------------|--------------|--------|
| 账号 | 云端托管 | OAuth | 不托管 |
| 发帖 | 一键/批量 | 定时自动 | **手动** |
| 数据 | 云端 | 云端 | **本地** |

## 价格
免费 50 条；Pro 买断首发 ¥139（非订阅）。

## 链接
官网：https://get.pongrabbit.cn/zh/?utm_source=zhihu&utm_medium=social&utm_campaign=zh_compare_2026  
下载：https://github.com/aiyahetun/pongrabbit-PublishKit/releases/latest  
插件：https://chromewebstore.google.com/detail/publishkit-companion/dijkbipanbpkgndbhonecpfnladaiipj  

（演示视频/GIF 见评论区或文末）
```

**话题：** `#独立开发` `#内容运营` `#效率工具` `#跨境电商`（按知乎实际话题名调整）

---

## 11. 掘金

**关联词：** `独立开发` · `Tauri` · `Chrome扩展` · `SQLite` · `跨境`  
**链接：** 中文官网 UTM `utm_source=juejin&utm_medium=social&utm_campaign=zh_tutorial_2026`

**标题**  
用 Tauri + 本地 API 做发稿台账：桌面端与 Chrome 插件如何只连 localhost

**摘要（≤100 字）**  
发稿匣 PublishKit 的架构简述：桌面 SQLite、本地 HTTP、MV3 Side Panel；强调不代发、不碰平台 OAuth。文末附官网与开源仓库。

**标签**  
`独立开发` `Chrome` `Tauri` `工具`

---

## 12. 少数派 / Appinn / 小众软件（投稿）

**关联词：** `Windows` · `买断` · `本地` · `浏览器插件` · `内容创作`  
**链接：** 中文官网 UTM `utm_source=sspai` 或 `appinn` 或 `iplaysoft`  
**附件：** Press 文件夹（logo + 3 张截图，自行打包）；正文用 §4.2 CN + 表格对比

**邮件/投稿标题**  
【首发】发稿匣 PublishKit：本地内容发布台账 + Chrome 插件（不做一键分发）

**正文**  
直接使用 §4.3 CN sections，补：**系统** Windows 10/11 64 位 · **许可** 免费 + Pro 买断 · **插件** Chrome 网上应用店已上架 · **隐私** 数据本地存储，插件仅 localhost  

**官网栏**  
https://get.pongrabbit.cn/zh/?utm_source=appinn&utm_medium=pr&utm_campaign=cn_review_2026

---

## 13. LinkedIn（创始人帖）

**关联词：** `content operations` · `cross-border marketing` · `local-first` · `build in public`  
**链接：** EN 官网 UTM `utm_source=linkedin&utm_medium=social&utm_campaign=founder_post`

**Post**

```text
We don’t auto-post for you—and that’s the point.

PublishKit is a local-first publish ledger for teams who still publish manually (X, LinkedIn, newsletters, CN platforms…). Desktop app + Chrome extension on localhost only.

If you’re tired of OAuth schedulers and matrix tools holding platform passwords, try the workflow:
draft → copy → publish in your browser → sync status.

Launch: lifetime Pro $38 · Free tier available.

https://get.pongrabbit.com/en/?utm_source=linkedin&utm_medium=social&utm_campaign=founder_post
```

---

## 14. X (Twitter)

**关联词：** `#buildinpublic` `#indiedev` `#contentmarketing` `#localfirst`  
**链接：** 短链可用官网 UTM；单条 ≤280 字符需精简

**Post A**  
Shipped: PublishKit — local SQLite publish ledger + Chrome extension (localhost only). You post manually; we track status. Not Buffer.  
https://get.pongrabbit.com/en/?utm_source=twitter&utm_medium=social&utm_campaign=founder_post

**Post B**  
Cross-border content ops without giving tools your social passwords: desktop drafts + side panel to mark published. Lifetime Pro $38.  
#buildinpublic

---

## 15. GitHub Release（发行说明模板）

**关联词：** 版本号 · `Windows` · `Chrome extension` · changelog  
**链接：** Releases 页自然流量；正文内嵌官网与 CWS

```markdown
## PublishKit vX.Y.Z

### Highlights
- …

### Install
- **Windows:** download `.exe` / `.msi` below (SmartScreen: More info → Run anyway if unsigned)
- **macOS:** `.dmg` on this release page
- **Browser:** [PublishKit Companion on Chrome Web Store](https://chromewebstore.google.com/detail/publishkit-companion/dijkbipanbpkgndbhonecpfnladaiipj)

### Links
- Website: https://get.pongrabbit.com/en/
- 中文: https://get.pongrabbit.cn/zh/
- Pro license: purchase on website → `PKPRO-` key by email

### Full changelog
…
```

---

## 16. Chrome 网上应用店（已上架，备查）

**链接：** §1 CWS · 隐私 https://get.pongrabbit.com/privacy.html  

完整字段见 `apps/extension/store/README.md`。站外帖统一称：**PublishKit Companion / 发稿匣助手**。

---

## 17. 用户证言占位（官网 / PH / 目录）

收集后替换 `【】` 内容，每条 1～2 句。

**EN Template**  
「【Role, region】— 【Before pain】. PublishKit gave us 【outcome】 without handing passwords to another SaaS.」

**CN Template**  
「【身份】— 以前用【表格/工具】总搞不清发没发；现在【具体改善】。」

---

## 18. 发帖检查清单

- [ ] 链接含 UTM（CWS 除外）  
- [ ] 一句边界：「不自动发帖 / no auto-posting」  
- [ ] 定价与官网一致  
- [ ] 演示素材已脱敏（无真实 Token、无客户名）  
- [ ] 英文帖链 `.com`，中文帖链 `.cn`  
- [ ] 需要时附隐私政策链接  

---

## 19. 修订记录

| 日期 | 说明 |
|------|------|
| 2026-10-03 | 初版：多平台文案 + UTM + 关联词 |
