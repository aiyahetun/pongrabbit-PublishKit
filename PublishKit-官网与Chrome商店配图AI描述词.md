# PublishKit · 官网落地页 & Chrome Web Store 配图 AI 描述词

| 属性 | 内容 |
|------|------|
| 文档版本 | v1.0（2026-09-21） |
| 对齐规范 | [docs/设计标准-PublishKit.md](./docs/设计标准-PublishKit.md) · [PublishKit-营销视觉AI描述词.md](./PublishKit-营销视觉AI描述词.md) |
| 产品版本 | 桌面端 v0.3.4 · 插件 v0.2.3 |
| 用途 | 生成 **官网落地页** 与 **Chrome Web Store** 所需的全部配图 |
| 生成方式 | AI 生图（ChatGPT / Midjourney / Flux 等）；**商店截图优先用真实 UI 截图**，AI 仅作补位 |

> **通则**：气质对齐 **Quiet Tool Studio**（Linear / Notion / Eagle）。暖米底 `#F7F6F3`、面板白 `#FFFFFF`、金色点缀 `#B8924A`。**默认无文字、无水印、无价格**（文案后期叠加）。禁止兔子、宠物、缝纫、AI 炫光渐变。

---

## 0. 素材总览（全覆盖清单）

### 0.1 官网落地页

| ID | 素材名 | 尺寸 | 优先级 | 落位 |
|----|--------|------|--------|------|
| LP-01 | App 图标 | 1024×1024 | P0 | Nav / Footer / 下载页 |
| LP-02 | Favicon | 32×32、180×180 | P0 | 浏览器标签 |
| LP-03 | 横版 Logo 锁标（无字） | 2400×600 | P0 | Nav 左侧 |
| LP-04 | Hero 桌面主视觉 | 2880×1620 | P0 | `/zh/` `/en/` 首屏右侧 |
| LP-05 | Hero 移动主视觉 | 1170×2532 | P1 | 移动首屏 |
| LP-06 | OG / Twitter Card | 1200×630 | P0 | 社交分享 meta |
| LP-07 | 功能图 · Today 队列 | 1920×1080 | P0 | Showcase §1 |
| LP-08 | 功能图 · Content + Inspector | 1920×1080 | P0 | Showcase §2 |
| LP-09 | 功能图 · 浏览器插件 | 1920×1080 | P0 | Showcase §3 |
| LP-10 | 功能图 · 任务看板 Kanban | 1920×1080 | P1 | Showcase §4（v0.3.4） |
| LP-11 | 功能图 · 周日历 | 1920×1080 | P1 | Showcase §5（v0.3.4） |
| LP-12 | 功能图 · 渠道发布包导出 | 1920×1080 | P2 | Showcase 可选 |
| LP-13 | 三步流程条图 | 2400×480 | P2 | 「准备 → 发布 → 回填」区块背景 |
| LP-14 | 「本地优先」信任插图 | 800×600 | P2 | Hero 副文案旁 |
| LP-15 | 定价区轻背景 | 1920×800 | P3 | Pricing 区块（纯纹理即可） |

### 0.2 Chrome Web Store（F21）

| ID | 素材名 | 尺寸 | 优先级 | 说明 |
|----|--------|------|--------|------|
| CWS-01 | 商店图标 | 128×128 | P0 | 96×96 图形 + 16px 透明边；与 LP-01 同源导出 |
| CWS-02 | Small Promo Tile | 440×280 | P0 | **必填**；搜索/分类页缩略图 |
| CWS-03 | Marquee Promo | 1400×560 | P1 | 可选；首页轮播资格 |
| CWS-04 | 截图 1 · 插件侧栏 + 待发 | 1280×800 | P0 | **必填**（≥1 张） |
| CWS-05 | 截图 2 · 复制富文本 | 1280×800 | P1 | 推荐 |
| CWS-06 | 截图 3 · 标记已发布 + URL | 1280×800 | P1 | 推荐 |
| CWS-07 | 截图 4 · 重复发布预警 | 1280×800 | P2 | 可选 |
| CWS-08 | 截图 5 · 桌面端配对设置 | 1280×800 | P2 | 可选 |

> **Chrome 截图建议**：优先从真实 `apps/extension` Side Panel + 桌面端截屏，再统一加 `#F7F6F3` 外框至 1280×800。AI 生成仅在没有可用截图时使用 CWS-04～08 描述词。

### 0.3 导出命名

```text
website/
  publishkit_icon_1024.png
  publishkit_favicon_32.png
  publishkit_apple_touch_180.png
  publishkit_logo_lockup_2400x600.png
  publishkit_hero_desktop_2880x1620.png
  publishkit_hero_mobile_1170x2532.png
  publishkit_og_1200x630.png
  publishkit_feat_today_1920x1080.png
  publishkit_feat_content_inspector_1920x1080.png
  publishkit_feat_extension_1920x1080.png
  publishkit_feat_kanban_1920x1080.png
  publishkit_feat_calendar_week_1920x1080.png
  publishkit_feat_channel_pack_1920x1080.png
  publishkit_flow_3step_2400x480.png
  publishkit_trust_local_800x600.png

apps/extension/store/
  icon_128.png
  promo_small_440x280.png
  promo_marquee_1400x560.png
  screenshot_01_sidepanel_1280x800.png
  screenshot_02_copy_1280x800.png
  screenshot_03_publish_1280x800.png
  screenshot_04_duplicate_warn_1280x800.png
  screenshot_05_pairing_1280x800.png
```

---

## 1. 第 0 步 · 角色设定（每条任务前先发）

> 我要为 **PublishKit（发稿匣）** 生成官网落地页与 Chrome Web Store 配图。产品是**本地优先的跨境内容运营工具**（桌面端 + 浏览器插件），气质像 Linear / Notion / Eagle，**不是**营销站炫光风、**不是**宠物电商、**不是**一键分发 SaaS。  
>   
> **视觉 Token（严格一致）**  
> - 背景 `#F7F6F3` · 面板 `#FFFFFF` · 主文字 `#1B1C19` · 次级 `#4E4638` · 强调 `#B8924A`  
> - 圆角 8–12px · 细线边框 `#E4E2DD` · 轻阴影 `0 8px 28px rgba(27,28,25,.10)`  
> - 默认 **无文字、无水印、无价格**；禁止兔子/宠物/缝纫/霓虹渐变  
>   
> 请确认后我开始指定尺寸与画面。

---

## 2. 品牌基础图（官网 + 商店共用）

### LP-01 · App 图标 1024×1024

> 为 **PublishKit** 设计应用图标，画布 **1024×1024**，**透明背景**，单张输出。  
>   
> **图形**：抽象「发稿匣 / 内容托盘」——圆角矩形容器（12px 感），内 2–3 条水平内容行，右下角 **金色勾选** `#B8924A`；描边 `#1B1C19` 2px，填充 `#FFFFFF` 或 `#F7F6F3`。  
> **风格**：Linear / Notion 克制几何；32×32 仍可辨认。  
> **禁止**：兔子、宠物、文字、水印、满版金渐变、3D 玻璃。  
>   
> 请只输出 1024×1024 透明 PNG。

— EN: PublishKit app icon 1024x1024 transparent, abstract dispatch tray rounded rect, 2-3 list rows, small gold checkmark #B8924A, ink #1B1C19, white fill, quiet SaaS tool, readable at 32px, no text no rabbit

### LP-02 · Favicon

> 从 LP-01 图标裁切/简化，输出 **32×32** 与 **180×180** 两档 PNG（可白底或透明）。图形仅保留容器轮廓 + 一点金色，确保 16px 仍可识别。

### LP-03 · 横版 Logo 锁标 2400×600

> 参考 LP-01，生成横版锁标 **2400×600**，透明底。左侧 22% 放图标（高度 70%），右侧 78% **完全留白**供后期叠字。**禁止生成任何字母或中文。**

---

## 3. 官网 Hero 与分享图

### LP-04 · Hero 桌面 2880×1620

> 参考 LP-01 与桌面端 UI（Today 或 Tasks 看板），生成官网 Hero **2880×1620**，16:9，无文字。  
>   
> **构图**  
> - 全幅背景 `#F7F6F3`，极轻纸纹；左上/左中 **40%** 留白供标题与 CTA  
> - 右侧 **55%**：macOS 风格应用窗口（圆角 12px，轻阴影），窗内展示 **Today 队列** 或 **Tasks 看板**：渠道 chip、ZH/EN、ready/scheduled 状态、金色「准备发布」按钮  
> - 右后方可选第二层窗口（blur 8px）暗示多任务  
> - 左下角可选小图标 LP-01（高度约 6%）  
>   
> **禁止**：大标题、价格、促销、平台 Logo、乱码 UI、插画 Hero 墙。

— EN: landing hero 2880x1620, warm #F7F6F3 whitespace left 40%, macOS app window right showing today task queue kanban, gold accent buttons, quiet Linear-style product shot, no headline text

### LP-05 · Hero 移动 1170×2532

> 竖版 Hero **1170×2532**，背景 `#F7F6F3`，顶部 18% / 底部 15% 留白。  
> 中部：**360px 宽浏览器插件 Side Panel** 全高展示（连接绿点、待发任务、金色「标记已发布」），或窄裁桌面 Today 列表。无文字。

### LP-06 · OG 分享图 1200×630

> OG 图 **1200×630**：左 62% 暖米留白 + LP-01 图标（高约 28%）；右 38% 裁切应用窗口一角（列表 + 金色激活导航）；底部 3px 金线 `#B8924A`。**不写任何标题文字。**

---

## 4. 官网功能展示图（Showcase）

> 以下均为 **1920×1080**，16:9，无文字。UI 须对齐 v0.3.4 真实界面结构（侧栏 220px、Inspector 360px、暖色 Token）。

### LP-07 · Today 队列

> 桌面窗口居中略偏下（占画布 78%），展示 **Today / 今天**：三节——建议先发、已排期、最近已发布；任务行含渠道 chip、语言、状态 pill、行内金色按钮。上方 15% 留白。状态色：ready 绿 `#3A6A4A`、scheduled 金 `#B8924A`、published 蓝 `#2563EB`。

### LP-08 · Content + Inspector

> 窗口 **左 60%** 内容库列表（标题/渠道/语言/状态列）；**右 40%** Inspector：文案预览区 `#FAF8F3` + 素材 3 列缩略图；选中行左侧 3px 金条；顶栏「复制标题 / 复制正文」按钮组。

### LP-09 · 浏览器插件

> **左 65%** 模糊中性浏览器区 `#E4E2DD`（**无**小红书/IG 等平台 Logo）；**右 35%** 360px 白底插件面板：绿点已连接、任务卡、复制按钮组、全宽金色「标记已发布」、URL 输入框。

### LP-10 · 任务看板 Kanban（v0.3.4）

> 窗口展示 **Tasks 看板**：列 draft / ready / scheduled / published / blocked；卡片含标题、渠道、语言；blocked 列卡片带红色原因一行；列头含计数。背景 `#F7F6F3`，窗口外轻投影。

### LP-11 · 周日历（v0.3.4）

> 窗口展示 **Calendar 周视图**：7 列日期格，事件 pill 按渠道色（小红书 `#FF2442`、Instagram `#E1306C` 等）；右侧或底部 drawer 列出选中日任务。气质安静，非 Google Calendar 炫彩。

### LP-12 · 渠道发布包导出（可选）

> 窗口展示任务 Inspector 或操作栏：**「导出渠道发布包」** 按钮、文件夹结构预览（`caption.txt` / `images/` / `checklist.md`），暗示一键导出发帖素材包。无真实平台 Logo。

### LP-13 · 三步流程条 2400×480

> 横条装饰图 **2400×480**，背景 `#F0EDE6`。三等分：左「文档/列表」图标、中「浏览器窗口」图标、右「勾选/同步」图标——均为 **细线几何图标** `#4E4638`，激活步 `#B8924A` 小圆点。极简，无文字，供流程区块底图。

### LP-14 · 本地优先信任插图 800×600

> 小插图 **800×600**：中心笔记本电脑 + 本地 SQLite 象征（小数据库圆柱图标），外圈 **无云上传箭头**（可用打叉的虚线云表示「不上传」）。暖米底，克制信息图，无文字。

### LP-15 · 定价区背景 1920×800（可选）

> 纯背景 **1920×800**：`#F7F6F3` 到 `#F0EDE6` 极 subtle 渐变 + 纸纹，**无任何 UI 元素**，供定价卡片叠在上面。

---

## 5. Chrome Web Store 配图

### CWS-01 · 商店图标 128×128

> 从 LP-01 导出：**有效图形 96×96 居中**，四周各 **16px 透明边**，总画布 **128×128** PNG。浅色/深色背景均可辨认（可加 1px `#E4E2DD` 描边）。

### CWS-02 · Small Promo Tile 440×280（必填）

> Chrome 商店小图 **440×280**，JPEG/PNG。  
>   
> **构图**（Google 建议：少文字、少杂乱、半尺寸仍可读）  
> - 背景 `#F7F6F3` 占 55%  
> - 左下 LP-01 图标（高约 40%）  
> - 右上：**360px 插件面板**裁切（Side Panel + 金色主按钮），占宽约 50%  
> - **不写扩展名文字**（商店标题自带）  
> - 可有一条 3px 金色底边作品牌线  
>   
> **禁止**：Chrome 字样、五星、排名、满版白底、超过 5 个词的文字。

— EN: Chrome Web Store small promo tile 440x280, warm #F7F6F3, app icon bottom left, browser extension side panel top right, gold CTA button, minimal no text, uncluttered, works at half size

### CWS-03 · Marquee Promo 1400×560（推荐）

> 大横幅 **1400×560**：左 35% 留白 + 图标；右 65% **桌面窗口 + 插件面板组合**（双屏暗示桌面+浏览器闭环）。背景 `#F7F6F3`，**无文字**，品牌色一致，半尺寸缩小仍清晰。

— EN: marquee promo 1400x560, desktop app plus extension panel composition, warm neutral, brand gold accent, no text, feature carousel ready, uncluttered high resolution

### CWS-04～08 · 商店截图 1280×800

> **优先真实截图**。若 AI 补位，画布 **1280×800**，**直角满幅 full bleed，无 padding**（Google 要求）。

#### CWS-04 · 截图 1（必填）Side Panel + 待发列表

> 1280×800：右侧 **360px PublishKit Side Panel**（已连接绿点、3 条待发任务、渠道标签）；左侧模糊浏览器发帖区。UI 对齐真实插件。无平台 Logo、无乱码。

#### CWS-05 · 截图 2 · 复制富文本

> 1280×800：插件面板聚焦「复制富文本 / 复制正文」按钮组 + Toast「已复制」；任务卡展示 Markdown 标题一行。

#### CWS-06 · 截图 3 · 标记已发布

> 1280×800：插件面板底部 URL 输入框已填 `https://...`，金色「标记已发布」按钮；任务状态变为 published 的即时反馈（绿色 Toast）。

#### CWS-07 · 截图 4 · 重复发布预警

> 1280×800：浏览器原生 confirm 对话框（或插件内 Modal）：「该内容×渠道 30 天内已发布，是否继续？」——中性文案框，背景仍为插件面板。

#### CWS-08 · 截图 5 · 桌面端配对

> 1280×800：桌面端 **设置 → 插件配对** 区块：API 地址 `127.0.0.1`、Bearer Token、重新生成按钮；右侧小字说明 localhost only。

---

## 6. 后期叠字参考（不在 AI 图内生成）

| 落位 | 中文 | 英文 |
|------|------|------|
| Hero 主标题 | 发稿匣 · 本地内容运营工作台 | PublishKit — Local content ops workspace |
| Hero 副标题 | 文案、素材、渠道、发布状态 — 一套管清 | Copy, media, channels, publish status — in one place |
| Hero CTA | 免费下载 Windows 版 | Download for Windows |
| 三步 | 准备内容包 → 去平台发布 → 插件回填状态 | Prepare → Post natively → Track in browser |
| CWS 商店标题 | （在 Developer Dashboard 填，不进图） | PublishKit Companion |
| CWS 短描述 | （在 Dashboard 填） | Copy, schedule, and track cross-border posts from your desktop library. |

字体：英文 Inter SemiBold；中文 PingFang SC Medium。标题 `#1B1C19`，强调 `#B8924A`。

---

## 7. 出图顺序与参考图

```
1. LP-01 图标 → 参考图 A
2. LP-03 锁标、LP-02 Favicon
3. 真实 UI 截图 → 参考图 B（Today / Content / Extension / Kanban）
4. LP-04 Hero、LP-06 OG（上传 A + B）
5. LP-07～12 功能图（上传 B）
6. CWS-01～03 从 A 导出/衍生
7. CWS-04～08 优先真实截图；缺图再用 AI
8. LP-05 移动 Hero、LP-13～15 可选
```

---

## 8. 自查清单

- [ ] 全部 Token 色正确，无 AI 紫蓝渐变
- [ ] 默认无文字；中文/英文标语仅后期叠加
- [ ] 无彭兔子/宠物/缝纫视觉
- [ ] Chrome 截图 1280×800 直角满幅
- [ ] CWS-02 440×280 已生成（商店必填）
- [ ] LP-04 Hero 左侧留白 ≥40% 供标题
- [ ] 图标 32×32 / 128×128 缩小测试通过
- [ ] Hero ≤800KB；商店图 JPEG quality 85 左右

---

*文档版本 v1.0 · 与 [PublishKit-营销视觉AI描述词.md](./PublishKit-营销视觉AI描述词.md) v2.0 配套使用*
