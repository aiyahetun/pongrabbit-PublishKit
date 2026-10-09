# PublishKit · 官网落地页方案

| 属性 | 内容 |
|------|------|
| 文档版本 | v1.0（2026-09-21） |
| 对齐规范 | [docs/设计标准-PublishKit.md](./docs/设计标准-PublishKit.md) §4.3 |
| 设计工具 | Google Stitch（`stitch.withgoogle.com`） |
| 现有参考 | [视觉稿设计/publishkit_landing_page/code.html](./视觉稿设计/publishkit_landing_page/code.html) |
| 配图描述词 | [PublishKit-官网与Chrome商店配图AI描述词.md](./PublishKit-官网与Chrome商店配图AI描述词.md) |
| 部署目标 | `get.pongrabbit.com/en/` · `get.pongrabbit.cn/zh/` |

---

## 1. 设计目标

1. **3 秒内传达定位**：本地优先的内容资产 + 发布台账，不做账号托管、不做一键分发。
2. **完成 AC10 路径**：访客 → 下载 Free → 外链购买 Pro → 激活码解锁。
3. **与 App 视觉一致**：Token、按钮、间距对齐设计标准；落地页信息密度**低于 App**。
4. **静态分语言**：`/zh/` 与 `/en/` 独立 HTML，禁止整页 JS 翻译（PRD §9.2）。

---

## 2. 页面结构（区块顺序）

```text
┌─────────────────────────────────────────────┐
│  TopNav（固定）Logo · 功能 · 定价 · 下载     │
├─────────────────────────────────────────────┤
│  §1 Hero          左文案 + 右产品图 LP-04    │
│  §2 信任条        本地优先 · 无云上传 · 开源 │
│  §3 三步流程      准备 → 发布 → 回填         │
│  §4 功能展示      5 屏轮播/交替排版          │
│  §5 边界说明      我们不做 / 我们做什么      │
│  §6 定价          Free vs Pro 双卡           │
│  §7 FAQ           6～8 条                    │
│  §8 Footer        隐私 · 条款 · 联系 · 备案  │
└─────────────────────────────────────────────┘
```

### 2.1 与现有 Stitch 稿差异（需修正）

现有 `publishkit_landing_page/code.html` 中定价区写 **「Pro ¥299/年 + 立即订阅」**，与 PRD **买断制（¥299 / $79 一次性）** 不一致。正式落地页须改为：

| 项 | 错误（现稿） | 正确（PRD + 调研结论） |
|----|--------------|------------------------|
| Pro 计费 | ¥299/年、订阅 | **¥299 / $79 买断** |
| Free 权益 | 「最多 3 个平台」 | **50 条内容 + 单工作区 + 基础插件** |
| Pro CTA | 立即订阅 | **购买 Pro · 获取激活码** |

---

## 3. 分区块规格

### §0 TopNav

| 元素 | 规格 |
|------|------|
| 高度 | 56px，固定顶栏，`backdrop-blur` + `#F7F6F3` 80% 透明度 |
| 最大宽度 | 1440px 居中 |
| 左 | Logo 锁标（LP-03 + 文字 PublishKit / 发稿匣） |
| 中 | 锚点：功能 · 定价 · FAQ · 文档（可选） |
| 右 | Ghost「查看价格」+ Primary「下载 Windows」 |
| 语言 | 右上角 `EN \| 中文` 链到 `/en/` `/zh/` |

### §1 Hero

| 元素 | 中文 | 英文 |
|------|------|------|
| H1 | 跨境内容发文，不再乱、不再慢、不再忘 | Cross-border content ops — clear, fast, tracked |
| 副标题 | 本地优先的内容资产与发布台账。不做账号托管，不做一键分发。 | Local-first copy, media, channels, and publish status. No OAuth. No auto-posting. |
| Primary CTA | 免费下载 Windows 版 | Download for Windows |
| Secondary | 购买 Pro | Get Pro |
| 信任行 | 🔒 数据默认在本地 | 🔒 Your data stays on your device |
| 配图 | LP-04（桌面）/ LP-05（移动） | 同左 |

**布局**：Desktop 左 45% 文案 / 右 55% 图；Mobile 文案上、图下。

### §2 信任条（Trust Bar）

三枚 icon + 一行说明，横向排列：

| # | 图标 | 中文 | 英文 |
|---|------|------|------|
| 1 | lock | 默认本地 SQLite，不上传文案 | Local SQLite by default |
| 2 | extension | 插件只连 127.0.0.1 | Extension talks to localhost only |
| 3 | block | 不存平台账号密码 | No platform credentials stored |

背景 `#F0EDE6`，上下 padding 24px。

### §3 三步流程

对齐设计标准 §5.1 核心流程：

| 步 | 标题（中） | 说明 | 图标 |
|----|------------|------|------|
| 1 | 准备内容包 | 桌面端选任务，复制文案/导出配图 | topic |
| 2 | 去平台发布 | 在各平台原生编辑器粘贴发布 | send |
| 3 | 插件回填状态 | 浏览器标记已发布，台账即时更新 | sync_saved_locally |

三列卡片：`#FFFFFF` 底、1px `#E4E2DD` 边、圆角 12px。可选底图 LP-13。

### §4 功能展示（5 屏）

交替左右排版（左图右文 ↔ 左文右图）：

| # | 标题 | 配图 ID | 要点 copy |
|---|------|---------|-----------|
| 1 | 今天发什么 | LP-07 | Today 队列：建议先发、排期、最近已发布 |
| 2 | 文案 × 素材同框 | LP-08 | 列表 + Inspector，复制不再切文件夹 |
| 3 | 浏览器现场助手 | LP-09 | Side Panel：复制、标记已发布、URL 回填 |
| 4 | 任务看板 | LP-10 | Kanban：draft → published，阻塞原因可见 |
| 5 | 发布日历 | LP-11 | 周视图复盘：何时发、发过什么 |

截图容器：16:9、`border-radius: 12px`、`box-shadow: var(--pk-shadow-lg)`。

### §5 边界说明（Manifesto）

双列对比表，避免用户误购：

| 我们做什么 | 我们不做 |
|------------|----------|
| 内容条目 × 渠道台账 | 平台账号登录/托管 |
| 复制文案、配图、发布包 | 一键全网自动发帖 |
| 插件回填链接与状态 | 私信/评论客服中台 |
| 本地备份与导出 | 强制上传素材到我们的云 |

背景 `#FFFFFF`，Section 标题「产品边界」。

### §6 定价

双卡布局，Pro 卡 `border: 2px #B8924A` + 「推荐」角标。

#### Free

| 项 | 中文展示 | 英文展示 |
|----|----------|----------|
| 价格 | ¥0 永久 | $0 forever |
| 权益 | 50 条内容 · 单工作区 · 基础任务/插件/导出/备份 | 50 items · 1 workspace · Core tasks & extension |
| CTA | 免费下载 | Download free |

#### Pro（买断）

| 项 | 中文展示 | 英文展示 |
|----|----------|----------|
| 价格 | **¥139** 买断（首发价，原价 ¥299 划线） | **$38** lifetime (was $79) |
| 权益 | 不限条目 · 拆分向导 · FTS 搜索 · 素材反查 · 渠道发布包 · 优先更新 | Unlimited items · Split wizard · FTS · Media usage · Channel packs |
| CTA | 爱发电 / 面包多 购买 | Lemon Squeezy — Get license key |
| 说明 | 付款后邮件发送 `PKPRO-` 激活码 | Key emailed after purchase |

> 首发价见 [PublishKit-竞品定价调研报告.md](./PublishKit-竞品定价调研报告.md) §5。

### §7 FAQ（建议 8 条）

1. 数据真的在本地吗？
2. 为什么不支持一键分发？
3. 支持哪些平台？（答：任意平台手动发；台账内置 19 渠道模板）
4. 浏览器插件支持哪些浏览器？
5. Free 和 Pro 差别？
6. 如何激活 Pro？
7. 有 macOS 版吗？（答：CI 可构建，签名版 roadmap）
8. 与 Buffer / 蚁小二有何不同？

### §8 Footer

| 列 | 内容 |
|----|------|
| 左 | PublishKit · © 2026 · 独立工具，与彭兔子账号无关 |
| 右 | Privacy · Terms · Contact · GitHub · 粤ICP备xxxx（.cn 站） |

链接：`privacy.html` / `terms.html`（待写）/ `mailto:` / Releases URL。

---

## 4. 技术实现要点

| 项 | 要求 |
|----|------|
| 结构 | 静态 HTML + CSS（可 Tailwind CDN 与 Stitch 稿一致） |
| 路径 | `/zh/index.html` · `/en/index.html` · 根路径 302 按 Accept-Language |
| Meta | `og:image` → LP-06；`twitter:card` summary_large_image |
| 下载 | CTA 链 GitHub Releases latest Windows `.exe` / `.msi` |
| 支付 | 外链 API：国内 **微信支付** · 海外 **Paddle**（见 [docs/PublishKit-上线联调收银方案.md](./docs/PublishKit-上线联调收银方案.md)） |
| 性能 | 图片 WebP + lazy-load；Hero ≤800KB |
| 无障碍 | 对比度 ≥4.5:1；按钮 focus ring |

---

## 5. Stitch 生成描述词

> 在 Stitch 新建项目，先导入 [视觉稿设计/publishkit/DESIGN.md](./视觉稿设计/publishkit/DESIGN.md) 作为 Token。

### 5.1 屏幕 S9 · Landing Desktop 1440×900

> Design a **PublishKit marketing landing page** at **1440×900**, light mode only, **Quiet Tool Studio** aesthetic aligned with Linear and Notion — not a flashy marketing site.  
>   
> **Tokens**: background `#F7F6F3`, panels `#FFFFFF`, text `#1B1C19`, secondary `#4E4638`, accent `#B8924A`, borders `#E4E2DD`. Font: Inter. Max content width 1440px, gutter 24px.  
>   
> **Layout top to bottom**:  
> 1. Fixed top nav 56px: logo left, anchor links center (Features, Pricing, FAQ), gold primary button "Download for Windows" right.  
> 2. Hero split 45/55: left headline "Cross-border content ops — clear, fast, tracked", subline about local-first and no auto-posting, two buttons (primary download, secondary get Pro), trust line with lock icon. Right: large macOS app window mock showing Today task queue with channel chips and gold action buttons — placeholder dashed area labeled for screenshot.  
> 3. Trust bar on `#F0EDE6`: three items — local data, localhost extension, no credentials.  
> 4. Three-step flow section: three equal cards with icons — Prepare content pack, Post on native platforms, Track in browser extension.  
> 5. Feature showcase alternating rows (5 sections): Today queue, Content+Inspector, Browser extension panel 360px, Kanban board, Week calendar — each with title, paragraph, 16:9 screenshot placeholder.  
> 6. Manifesto two-column table: what we do vs what we don't.  
> 7. Pricing two cards: Free $0 and Pro lifetime **$38** launch (was $79) highlighted with gold border and "Recommended" badge — **not subscription**.  
> 8. FAQ accordion 8 items.  
> 9. Footer with privacy links.  
>   
> **Constraints**: medium-low density, generous whitespace, no illustration hero wall, no purple AI gradients, no pet/rabbit imagery, rounded corners 8-12px, subtle shadows only.

### 5.2 屏幕 S10 · Landing Mobile 390×844

> Same PublishKit landing page adapted to **390×844** mobile. Single column: hamburger nav, stacked hero (headline then screenshot), trust bar vertical, three-step cards stacked, feature sections single column, pricing cards stacked (Pro first on mobile), FAQ, footer. Maintain identical tokens and tone.

### 5.3 Stitch 迭代 Prompt（第二轮）

> Annotate iteration: reduce hero headline to 2 lines max on mobile; increase Pro pricing card contrast; replace dashed screenshot placeholders with note "use LP-07 asset"; add language switcher EN/ZH in nav; fix Pro card to say "lifetime" not "per year".

### 5.4 Stitch Play 流程（可选）

> Connect screens: Hero "Download" button → external link toast; Pricing "Get Pro" → modal explaining activation key `PKPRO-` in desktop Settings.

---

## 6. 配图映射表

| 落地页位置 | 素材 ID | 文件名 |
|------------|---------|--------|
| Nav Logo | LP-01 + LP-03 | publishkit_logo_lockup_2400x600.png |
| Hero | LP-04 / LP-05 | publishkit_hero_desktop_2880x1620.png |
| og:image | LP-06 | publishkit_og_1200x630.png |
| Showcase ×5 | LP-07～11 | publishkit_feat_*.png |
| Favicon | LP-02 | publishkit_favicon_32.png |

---

## 7. 交付清单

| # | 交付物 | 负责人 | 状态 |
|---|--------|--------|------|
| 1 | Stitch S9/S10 高保真稿 | 设计 | ☐ |
| 2 | AI 配图全套（LP-01～11） | 你 + AI | ☐ |
| 3 | `/zh/` `/en/` 静态页 HTML | 开发 | ☐ |
| 4 | privacy.html / terms.html 中英版 | 产品 | ☐ |
| 5 | 支付 API + Webhook + 激活码邮件 | 开发 | ☐ 见 [docs/PublishKit-上线联调收银方案.md](./docs/PublishKit-上线联调收银方案.md) |
| 6 | `get.pongrabbit.cn` / `.com` 静态部署 | 运维 | ☐ |
| 7 | AC10 验收录屏 | 产品 | ☐ |

---

## 8. 修订记录

| 版本 | 日期 | 说明 |
|------|------|------|
| v1.0 | 2026-09-21 | 首版：结构、文案、Stitch 描述词、修正订阅文案 |
| v1.1 | 2026-09-21 | 首发价调整为 ¥174 / $48 |
| v1.2 | 2026-09-21 | 首发价再降 20%：¥139 / $38 |

---

*对齐 [docs/设计标准-PublishKit.md](./docs/设计标准-PublishKit.md) §4.3 · §7 Stitch 流程*
