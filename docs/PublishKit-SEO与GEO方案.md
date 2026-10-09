# PublishKit · SEO 与 GEO 方案

| 属性 | 内容 |
|------|------|
| 文档版本 | **v1.1**（2026-09-22） |
| 状态 | **TDK v2 已写入 HTML**（2026-09-22）；工程项 E1–E11 已完成 |
| 站点 | `https://get.pongrabbit.cn` · `https://get.pongrabbit.com` |
| 参考 | 彭兔子 [国内SEO与数据统计执行手册](../../彭兔子制版/docs/七月上线/国内SEO与数据统计执行手册-小白版.md) · `seoHreflang.js` 双站 alternate 模式 |

---

## 1. 术语

| 词 | 含义 | PublishKit 要做什么 |
|----|------|---------------------|
| **SEO** | 传统搜索引擎（Google、Bing、百度、神马） | title/description、sitemap、收录、结构化数据 |
| **GEO** | Generative Engine Optimization（ChatGPT / Perplexity / 豆包等 AI 检索） | 清晰实体定义、FAQ 可引用段落、`llms.txt`、避免 AI 误读为「自动发帖 SaaS」 |

---

## 2. 竞品 SEO / GEO 调研（2026-09）

> 数据来源：竞品官网 title/description、公开落地页、行业 GEO 实践（Answer-first、llms.txt、对比页）。定价背景见 [竞品定价调研报告](../PublishKit-竞品定价调研报告.md)。

### 2.1 市场分三类（PublishKit 只打其中一类）

```text
                    自动发帖 / 账号托管 / 一键分发
                                    ↑
              Buffer · Later · Hootsuite · 蚁小二 · 融媒宝
                                    │
    ────────────────────────────────┼────────────────────────→ 搜索量
                                    │
         Eagle · Billfish · Typora  │  PublishKit ★
         ContentMK · Postwinds      │  （本地台账 + 插件回填）
                                    │
                         纯本地 / 买断 / 不做 OAuth
```

**SEO 结论**：不要抢「社媒调度器」「一键分发」大词 — 流量大但**误购率高**、AI 也会把 PublishKit 归类错误。应抢「本地内容管理」「发布台账」「跨境内容运营」等**意图匹配**词。

### 2.2 海外竞品：他们怎么写 title / 怎么被 AI 引用

| 产品 | 典型 title / H1 关键词 | SEO 打法 | 对 PublishKit 的启示 |
|------|------------------------|----------|----------------------|
| **Buffer** | social media scheduler, plan & publish | 品牌词 +「scheduler」类目词；大量对比文（vs Hootsuite） | 可做 **反向对比**：「不是 Buffer — 不做 auto-post」 |
| **Later** | visual social media planner, Instagram | 视觉排期 + 平台名 | 我们不强调 IG，强调 **ledger / asset** |
| **Typefully** | write & schedule threads for X | 垂直平台 + writing | 英文可蹭 **cross-border / multi-channel prep** |
| **ContentMK** | desktop content management, local-first, SQLite | **最接近**：replace spreadsheets, privacy-first, multi-site | 对标其 **local-first + desktop** 叙事，加 **publish status + extension** |
| **Postwinds / trypost** | open-source Buffer alternative, self-hosted | GitHub +「alternative」长尾 | 英文长尾：`buffer alternative without auto posting` |
| **Ferryman / WPSeoHub** | local SEO calendar, meta title 50–60 chars | 教程型博客 + 工具页 | FAQ 写清 **title/desc 字数建议** 可引 GEO |
| **Voltis Content Calendar** | local-first planner, **NOT** auto-posting | 明确边界句放在首屏 | **与 PublishKit 同策略**，可互链/对比 |

**海外竞品共性**

- Title 50–60 字符；description 150–160 字符；H1 含 **1 个类目词 + 1 个差异化词**。
- 高转化长尾：**「X alternative」**、**「local-first」**、**「self-hosted / on your device」**。
- GEO：SoftwareApplication + FAQPage；博客用 **Answer-first**（首段直接回答「What is X?」）。
- 第三方背书：Product Hunt、G2、Reddit、Indie Hackers — AI 检索常引用这些来源（P2 内容）。

### 2.3 国内竞品：他们怎么占词 / 我们怎么错开

| 产品 | 典型占词 | 模式 | PublishKit 策略 |
|------|----------|------|------------------|
| **蚁小二** | 自媒体矩阵、多平台一键分发、多账号管理 | 云端 SaaS + 分发 | **对立定位**：页内首屏 + FAQ 写「不做一键分发」 |
| **融媒宝 / 新媒体管家** | 批量发布、矩阵运营、短视频分发 | 订阅制 | 不竞价「矩阵」「批量发」；改打 **台账、资产** |
| **即推 GEO / GEOFlow** | AI 搜索优化、GEO 内容生产 | 卖 GEO 服务/系统 | 品类不同；避免用户搜「GEO 工具」时进错站 — title 用 **发稿匣 / 内容资产** |
| **SoPilot 等 AI 营销台** | AI 写稿 + 自动发布 + SEO 文章 | 云端 AI | 强调 **本地 SQLite、无 OAuth、手动发布** |
| **飞书多维表格 / Notion** | 内容日历模板 | 通用协作 | 对比句：「专为发布状态设计的桌面工具，不是表格模板」 |

**国内 SEO 特点**

- 百度仍看 **备案 + 域名年龄 + 内链**；新子域 `get.*` 需 **独立提交**，短期配额可能为 0。
- 中文 title 建议 **25–35 字**（含品牌）；description **70–120 字**，前 40 字放核心卖点。
- 占词优先：**内容资产管理、发布台账、自媒体台账、跨境内容运营、本地内容管理工具**。
- 慎用大词：**新媒体运营工具、社媒管理**（与蚁小二同质，CTR 差）。

### 2.4 竞品 GEO（生成式搜索）观察

| 平台 | 用户常问 | 竞品怎么被描述 | PublishKit 目标回答 |
|------|----------|----------------|---------------------|
| ChatGPT / Perplexity | best Buffer alternative local | 推 Postwinds、Typefully 等 | **local ledger, no auto-post, Windows desktop** |
| 豆包 / DeepSeek / 通义 | 自媒体管理工具推荐 | 蚁小二、融媒宝 | **本地台账、不做账号托管、买断制** |
| Google AI Overview | content calendar tool | SaaS 调度器为主 | FAQ 首句定义品类，避免只写 calendar |

**GEO 实操（行业共识 + 竞品验证）**

1. **实体定义句**（全站一致，写入 `llms.txt` + FAQ 第一条）：  
   「PublishKit / 发稿匣是本地优先的桌面内容资产与发布管理系统，管理文案、素材、渠道与发布状态；不托管平台账号，不自动发帖。」
2. **对比表可见 HTML**（非纯图片）：我们是什么 / 不是什么 — AI 易抽取表格。
3. **`llms.txt`**：产品边界、定价事实、下载 URL、隐私页 — 减少模型幻觉。
4. **FAQ 与 JSON-LD 逐字一致** — 不只在 Schema 里写 AI 看不到的字段。
5. **P1 静态对比页**（见 §12）：`/compare/vs-buffer`、`/compare/vs-yixiaoer` — 专门吃「alternative / 对比」GEO 查询。

### 2.5 关键词矩阵（首发落地页 + 后续内容）

#### 英文 — 优先（P0 写入 meta / H1 / FAQ）

| 意图 | 目标词 / 短语 | 放置 |
|------|---------------|------|
| 品类 | content asset management, publish management, content operations | title, H1, llms.txt |
| 差异化 | local-first, on your device, no OAuth, no auto-posting | hero, FAQ, manifesto |
| 场景 | cross-border content team, multi-channel prep | EN 副标题、features |
| 长尾 | publish tracker, content ledger, buffer alternative without scheduling | P1 对比页、博客 |

#### 英文 — 避免主动优化（可 FAQ 澄清）

`social media scheduler`, `auto post`, `post to all platforms`, `OAuth`, `Hootsuite alternative`（除非对比页标题）

#### 中文 — 优先（P0）

| 意图 | 目标词 / 短语 | 放置 |
|------|---------------|------|
| 品类 | 内容资产管理、发布管理系统、内容运营工具 | title, H1 |
| 差异化 | 本地优先、不做账号托管、不做一键分发 | hero（已对齐）、FAQ |
| 场景 | 跨境内容、多渠道内容准备、发布台账 | features、定价区 |
| 长尾 | 自媒体发布台账、桌面内容管理、发稿管理 | P1 帮助页 |

#### 中文 — 避免

`一键分发`, `矩阵发布`, `多账号托管`, `自动发帖工具`

### 2.6 与竞品定价叙事的 SEO 配合

| 竞品类型 | 用户搜索动机 | 落地页应出现的对比句 |
|----------|--------------|----------------------|
| Buffer/Later（$60–300/年） | 找便宜调度器 | 「买断 ¥139 / $38，管资产与台账，不包自动发帖」 |
| 蚁小二（¥360+/年） | 找国内矩阵工具 | 「不做云端账号矩阵；数据在本地，手动发布 + 插件记状态」 |
| Eagle（$35 买断） | 找素材库 | 「含渠道包、发布状态、Chrome 插件，不仅是素材库」 |

---

## 3. URL 与语言策略

### 3.1  canonical 主 URL（每个语言一条）

| 语言 | 规范 URL | 说明 |
|------|----------|------|
| 中文 | `https://get.pongrabbit.cn/zh/` | 国内主域，默认语言 |
| 英文 | `https://get.pongrabbit.com/en/` | 海外主域 |

**不要用** `get.pongrabbit.cn/en/` 作为英文 canonical（跨域语言分流：cn 站中文、com 站英文）。

### 3.2 hreflang（双站互指）

在 **zh 页** `<head>` 增加：

```html
<link rel="alternate" hreflang="zh-CN" href="https://get.pongrabbit.cn/zh/" />
<link rel="alternate" hreflang="en" href="https://get.pongrabbit.com/en/" />
<link rel="alternate" hreflang="x-default" href="https://get.pongrabbit.com/en/" />
```

在 **en 页** 对称添加。对齐彭兔子 `seoHreflang.js` 思路。

### 3.3 根路径

| 域名 | 行为 |
|------|------|
| `get.pongrabbit.cn/` | 302 → `/zh/` |
| `get.pongrabbit.com/` | 302 → `/en/`（或 Accept-Language，与现 `index.html` 一致） |

---

## 4. 页面级 Meta（待写入 HTML）

### 4.1 中文 `/zh/`

| 标签 | 文案 | 策略说明 |
|------|------|----------|
| `<title>` | 发稿匣 — 本地内容与发布管理工具 | **SEO**：品牌 + 用户能理解的品类词 |
| `meta description` | 发稿匣是本地桌面内容与发布管理工具，管理文案、素材、渠道与发布状态。不做账号托管，不做一键分发。Windows 免费，Pro 买断 ¥139。 | **GEO**：首句 Answer-first 定义实体 |
| `meta keywords` | 发稿匣,PublishKit,内容管理,发布管理,跨境内容,本地内容管理 | **百度**：辅助占词（不用「台账」） |
| `og:title` | 发稿匣 — 本地内容与发布管理工具 | 与 title 一致 |
| `og:description` | 本地桌面工具，管文案、素材、渠道与发布状态。不做账号托管，不做一键分发。 | 分享卡片摘要 |
| `og:url` | `https://get.pongrabbit.cn/zh/` |
| `og:locale` | `zh_CN` |
| `og:image` | 绝对 URL：`https://get.pongrabbit.cn/assets/images/publishkit_og_1200x630.png` |

> **注意**：当前 HTML 里 `og:image` 为相对路径 `../assets/...`，微信/Slack 分享会裂图 — **上线前必须改绝对 URL**。

### 4.2 英文 `/en/`

| 标签 | 文案 | 策略说明 |
|------|------|----------|
| `<title>` | PublishKit — Local content & publish management tool | **SEO**：management 比 ledger 更易理解 |
| `meta description` | PublishKit is a local-first desktop app for content and publish management. …—not a Buffer-style scheduler. Free on Windows; Pro lifetime $38. | **GEO**：首句实体定义 + 对比 Buffer |
| `og:title` | PublishKit — Local content & publish management tool | 与 title 一致 |
| `og:description` | Local-first desktop app for content assets and publish tracking. No account hosting. No auto-posting. | 分享摘要 |

### 4.3 其他页面

| 页面 | title 要点 |
|------|------------|
| `privacy.html` | Privacy Policy — PublishKit |
| 未来 `terms.html` | Terms of Service — PublishKit |

每页独立 `canonical` + `description`。

---

## 5. 结构化数据（JSON-LD）

在 `zh/index.html` 与 `en/index.html` 底部（`</body>` 前）插入两段：

### 5.1 SoftwareApplication

```json
{
  "@context": "https://schema.org",
  "@type": "SoftwareApplication",
  "name": "PublishKit",
  "alternateName": "发稿匣",
  "applicationCategory": "BusinessApplication",
  "operatingSystem": "Windows 10, Windows 11",
  "offers": {
    "@type": "Offer",
    "price": "0",
    "priceCurrency": "CNY"
  },
  "description": "Local-first content asset and publish ledger. No OAuth. No auto-posting."
}
```

英文页 `priceCurrency` 用 USD；Pro 价可另加第二个 `Offer` 或只在正文展示（避免 Schema 与首发价频繁改）。

### 5.2 FAQPage

从页面 FAQ 区抽取 6～8 问，与可见文案 **完全一致**（GEO 关键：AI 只引用页内真实文字）。

---

## 6. 站点文件（待新增到 `website/`）

| 文件 | 用途 |
|------|------|
| `robots.txt` | 允许抓取；指向 sitemap |
| `sitemap.xml` | 列出 canonical URL（zh/en/privacy，后续 terms） |
| `llms.txt` | GEO：产品一句话定义、边界、下载链接、定价事实 |
| `assets/images/manifest.json` | 已有；可选 `WebSite` 图标引用 |

### 6.1 `robots.txt` 示例

```text
User-agent: *
Allow: /

Sitemap: https://get.pongrabbit.cn/sitemap.xml
Sitemap: https://get.pongrabbit.com/sitemap.xml
```

（若 cn/com 各部署一份，可只写本域 sitemap。）

### 6.2 `sitemap.xml` 最小集

```xml
<!-- get.pongrabbit.cn 版 -->
<url><loc>https://get.pongrabbit.cn/zh/</loc><priority>1.0</priority></url>
<url><loc>https://get.pongrabbit.cn/privacy.html</loc><priority>0.3</priority></url>
```

com 站对称 `/en/`。

### 6.3 `llms.txt` 完整版（GEO，对齐竞品 ContentMK / llms.txt 规范）

```text
# PublishKit / 发稿匣

> Local-first desktop app for content asset and publish management.
> Manages copy, media, channels, and publish status on your device.
> Does NOT auto-post, store platform passwords, or offer one-click multi-platform distribution.

## What we are
- Content asset library + publish ledger for cross-border / multi-channel teams
- Windows desktop + Chrome extension (manual publish, status sync back)
- Freemium: Free 50 items / Pro lifetime ¥139 or $38

## What we are NOT
- Not a Buffer/Later-style social media scheduler
- Not 蚁小二-style cloud matrix auto-distribution
- Not OAuth / account hosting SaaS

## Canonical URLs
- English: https://get.pongrabbit.com/en/
- 中文: https://get.pongrabbit.cn/zh/
- Privacy: https://get.pongrabbit.cn/privacy.html

## Download
- Latest release: https://github.com/aiyahetun/pongrabbit-PublishKit/releases/latest

## Compare (for AI citation)
- vs schedulers: we track status; you publish manually in the browser
- vs 蚁小二: local data, no cloud account matrix
- vs Eagle: includes channels, publish states, and extension workflow
```

---

## 7. 搜索引擎登记（上线后人工）

### 7.1 海外

| 平台 | 动作 | 备注 |
|------|------|------|
| **Google Search Console** | 添加资源 `https://get.pongrabbit.com/`，提交 sitemap | 彭兔子 www 已有 GSC 经验 |
| **Bing Webmaster** | 同上 | 可选 |
| **IndexNow** | 可选 | 新页更新时 ping |

### 7.2 国内

| 平台 | 动作 | 备注 |
|------|------|------|
| **百度站长** | 验证 `get.pongrabbit.cn`，提交 sitemap | 新站配额常为 0，靠备案 + 外链 |
| **神马搜索** | HTML 标签验证（彭兔子 www 已做过） | 复制验证 meta 到 zh 页 head |
| **360 站长** | 可选 | 验证 + sitemap |
| **搜狗站长** | 可选 | 验证 + sitemap |

| **备案号关联** | 百度站长绑定 ICP | 与 Phase 0 页脚 ICP 一致 |

> 国内新子域前 2–4 周「索引为 0」正常；配合 www 主站 1–2 条内链 + 知乎/少数派等外链（见 §12 P2）。

### 7.3 统计（可选 P1）

| 工具 | 域 |
|------|-----|
| Google Analytics 4 | `.com` |
| 百度统计 / 友盟 | `.cn` |

静态站可插脚本到 `site.js` 或页脚，**注意隐私政策披露**。

---

## 8. GEO 内容原则（防 AI 误读）

PublishKit 最容易被 AI 说成「社交媒体调度器 / Buffer 替代品自动发帖」。页内必须反复明确：

| 我们是什么 | 我们不是什么 |
|------------|--------------|
| 本地内容资产 + 发布**台账** | 一键全网分发 |
| 浏览器原生手动发布 + 插件回填状态 | 平台 OAuth 托管 |
| 买断制桌面工具 | 订阅制 SaaS 调度器 |

**GEO 写法**：FAQ、Manifesto、首屏副标题用 **短句 + 对比表**；避免空洞形容词。

**Answer-first 模板**（每条 FAQ 答案第一句可直接被 AI 摘抄）：

| 问题 | 中文答案首句 | 英文答案首句 |
|------|--------------|--------------|
| PublishKit 是什么？ | 发稿匣是运行在您电脑上的内容资产与发布管理系统。 | PublishKit is a local-first desktop app for content assets and publish tracking. |
| 和蚁小二有什么区别？ | 我们不提供云端多账号一键分发，数据在本地，您在浏览器手动发布。 | We do not offer cloud matrix posting; you publish manually and log status locally. |
| 能自动发帖吗？ | 不能；我们不做 OAuth，也不代替您在平台点击发布。 | No — we do not auto-post or store platform credentials. |

---

## 9. 与 www 彭兔子主站的关系

| 项 | 建议 |
|----|------|
| 品牌 | 页脚注明「独立工具，与彭兔子制版账号无关」✅ 已有 |
| 互链 | `www.pongrabbit.cn` / `.com` 页脚可加「PublishKit 发稿匣 → get.*」 |
|  sitemap | **独立** sitemap，不并入 www（子域分开提交） |
| 权重 | 新子域需单独积累；可从 www Help/Blog 发 1～2 条内链 |

---

## 10. 实施清单（工程 P0）

| # | 任务 | 文件 | 状态 |
|---|------|------|------|
| E1 | og:image / canonical 改绝对 URL | `zh/en/index.html` | ✅ |
| E2 | 更新 meta description 对齐新文案 | 同上 | ✅ |
| E3 | hreflang 四链 | 同上 | ✅ |
| E4 | twitter:card | 同上 | ✅ |
| E5 | JSON-LD SoftwareApplication + FAQPage | 同上 | ✅ |
| E6 | `robots.txt` | `website/robots.txt` | ✅ |
| E7 | `sitemap.xml`（cn/com 各一或文档说明） | `website/sitemap.xml` | ✅ |
| E8 | `llms.txt` | `website/llms.txt` | ✅ |
| E9 | 验收脚本 `website/scripts/verify-seo.mjs` | 新建 | ✅ |
| E10 | 更新 [官网验收清单](./官网验收清单.md) AC-SEO | docs | ✅ |
| E11 | 首页可见 **对比表** HTML（我们/不是我们） | manifesto 区已有 | ✅ |
| E12 | FAQ 答案改为 Answer-first 首句 | 保持原意，仅去过时支付文案 | ✅ |
| E13 | `llms.txt` 链到 footer | 不链出（不对用户透传） | ⏭ |

---

## 11. 内容路线图（SEO + GEO P1/P2）

| 阶段 | 页面 / 动作 | 目的 | 优先级 |
|------|-------------|------|--------|
| **P0** | 落地页 meta / JSON-LD / llms.txt / sitemap | 收录 + 实体定义 | 上线前 |
| **P1** | `compare/vs-buffer-en.html` · `compare/vs-yixiaoer-zh.html` 静态对比页 | 吃 alternative / 对比 GEO 查询 | 上线后 2 周内 |
| **P1** | `help/getting-started-zh.html` · `help/getting-started-en.html` | 长尾 + 教程引用 | 上线后 1 月 |
| **P2** | Product Hunt / Indie Hackers 首发帖（含 canonical URL） | 海外 AI 第三方引用 | 上线当周 |
| **P2** | 知乎 / 少数派「本地内容台账」一文 + 链 get.cn | 百度权重 + 品牌词 | 上线后 |
| **P2** | Chrome Web Store 描述与官网 **同一实体定义句** | 跨源 GEO 一致 | 商店上架时 |

### P1 对比页 title 建议

| 页面 | title |
|------|-------|
| vs Buffer | PublishKit vs Buffer — local publish ledger, not a scheduler |
| vs 蚁小二 | 发稿匣 vs 蚁小二 — 本地台账，不做一键分发 |

每页：独立 canonical、300–500 字对比表、链回 `/zh/` 或 `/en/` 下载。

---

## 12. 度量指标（上线后 30 / 90 天）

| 指标 | 工具 | 目标（90 天） |
|------|------|----------------|
| 索引页数 | GSC / 百度站长 | ≥3（zh/en/privacy） |
| 品牌词展现 | GSC「PublishKit」「发稿匣」 | 查询出现且无错误摘要 |
| 分享卡片 | 微信 / Slack / LinkedIn | og 图 + title 正确 |
| GEO 抽测 | ChatGPT / Perplexity / 豆包 | 10 次提问中 ≥7 次含 **local / 不自动发帖** |
| 误购信号 | 客服 / FAQ 点击 | 「能自动发吗」FAQ 点击率可接受即可 |

---

## 13. 验收标准

**本地**

```powershell
node website/scripts/verify-seo.mjs
```

检查：canonical、hreflang、og 绝对 URL、sitemap 可达、JSON-LD 可解析。

**线上**

- [ ] Rich Results Test（Google）无致命错误
- [ ] 分享 `get.pongrabbit.cn/zh/` 到微信，卡片图/标题正常
- [ ] GSC「网址检查」→ 已编入索引（可能需数天）
- [ ] 向 ChatGPT / Perplexity / 豆包 问「PublishKit 是什么 / 和蚁小二区别」— 回答应含 **local / no auto-posting / 不做一键分发**（GEO 长期项）
- [ ] 竞品词抽测：搜「buffer alternative local」不应只出现 SaaS 调度器而无 PublishKit（P1 对比页后复测）

---

## 14. 变更记录

| 版本 | 日期 | 变更 |
|------|------|------|
| v1.1 | 2026-09-22 | 竞品 SEO/GEO 调研：海内外占词、关键词矩阵、llms.txt 扩写、P1/P2 内容路线、度量指标 |
| v1.0 | 2026-09-22 | 初版：SEO+GEO 策略、双域 hreflang、结构化数据、站长登记 |
