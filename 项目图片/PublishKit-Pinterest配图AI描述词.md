# PublishKit · Pinterest 配图 AI 描述词

| 属性 | 内容 |
|------|------|
| 文档版本 | v1.6（2026-10-09） |
| 语气 | **独立开发者坦白局**（与 [PublishKit-Pinterest营销文案.md](./PublishKit-Pinterest营销文案.md) §三 一致） |
| 对齐文案 | 图上「文字」块 = Pin 标题 + 正文（去链接/去价）；改文案后运行 `scripts/sync-pinterest-pin-on-image-text.mjs` |
| 对齐视觉 | [PublishKit-营销视觉AI描述词.md](../PublishKit-营销视觉AI描述词.md) · Quiet Tool Studio |
| 用途 | 按每一条 Pin 生成带文字的竖图。二级标题与文案文档的图名一致 |
| 画布 | 全部 **1000 × 1500 px**，竖版 **2:3**。横图参考只取构图，重新排成竖 Pin |

这份文档是生图描述词，不要拿去排期导入。

每一节对应文案里的一条 Pin。中文 Pin、英文 Pin 各出一张，同一张图不要中英混排。出图时上传该节参考图，再贴整段描述词。

---

## 0. 产品与竞品（出图前先读）

### 0.1 PublishKit 是什么、强在哪

| 维度 | 我们卖什么 | 用户立刻能感知的好处 |
|------|------------|----------------------|
| **定位** | Windows 桌面 **内容日历 + 发布台账** + Chrome **发稿匣助手** | 不是「再买一个按月扣的排期 SaaS」，而是 **帖还是你发，电脑帮你记清楚** |
| **闭环** | 文案 × 素材 × 渠道版本 × 发布状态（本机 SQLite） | 翻聊天记录找链接、表格对不上日期、贴错渠道口吻——这三类翻车少很多 |
| **今天** | 「今天」待发清单 | 打开就知道 **还欠几条**，不用对着空表猜 |
| **多渠道** | 同一条内容下：小红书 / Pinterest / YouTube 等各一版正文 | 不用为每个平台开三个 Word；复制的是 **当前标签页那一版** |
| **现场** | 侧栏贴在发帖页旁：复制 → 标记已发布 → 带回链接 | 人不用切回桌面；**127.0.0.1**，不读密码、不上传正文到我们的服务器 |
| **状态** | 看板 / 周历 / 按渠道勾选「已发」 | 「这篇算发过了吗」有依据：是 **链接记上了**，不是「我好像点过」 |
| **商业** | Free 50 条 · Pro **买断**（国内 ¥139 / 海外 $38 首发） | 对标 Buffer/Later **一年订阅费**，我们是一次付清、**不含代发** |

**一句话差异（对内）：** 云端 scheduler 卖「我替你定时发」；Eagle 卖「素材收拢」；PublishKit 卖 **「你自己发，但别再把台账弄丢」**。

### 0.2 和同类比，图上文案该强调什么、别碰什么

| 类型 | 代表 | 他们营销怎么写（2025–2026 公开页抽样） | 我们能借 | 我们不能借 |
|------|------|----------------------------------------|----------|------------|
| **云端排期** | [Buffer](https://buffer.com/features/social-media-calendar) | *See your whole month at once*；整月一眼、拖一拖改时间 | 结果感、**一眼看清** | auto-post、schedule for you、连账号 |
| **视觉排期** | Later | 拖拽日历、Grid 预览、Instagram/TikTok 工作流 | 「发之前能看见」的节奏 | 我们不做 Grid 预览当主卖点 |
| **团队审批** | [Loomly](https://www.loomly.com/why-loomly/loomly-vs-buffer) | *stress-free*、*plan weeks in minutes*、审批流 | 少操心、步骤短 | 团队审批、Canva 一体化 |
| **本地素材** | [Eagle 中文站](https://cn.eagle.cool/) | **散在电脑里** → 收拢、拖曳收藏、0.5 秒找到 | 中文先写 **散在哪**（聊天、文件夹、表格） | 把我们说成纯图库 |
| **国内矩阵** | 蚁小二、融媒宝等 | 一键分发、多账号托管 | 只作对比：**我们不做** | 矩阵、托管、秒发全网 |

**Pinterest 图上的字**要比 Pin 描述更短，但语气必须是 **坦白局**：像 Show HN / 即刻发帖，不像品牌通稿。

### 0.3 独立开发者坦白局（图上文案规则）

与营销文档 **§三** 一致；生图时**逐字**用各节「文字」块，不要改成广告腔。

| 要素 | 中文 | 英文 |
|------|------|------|
| **人设** | 做跨境内容的**一个人**，工具是**自己做给自己用**的 | *I built…* / *I ship…* / *Wrong tool if…* |
| **标题** | 动机或翻车：*我做 X 是因为…* / *我也干过…* / *别指望…* | 问句或坦白句；可夹 1 个搜索词 |
| **副文案** | ① 当时真痛点 ② 边界（手动发、本地、不 OAuth）③ 你怎么用一遍；可说「不适合要自动发的人」 | 第二句出现 PublishKit；避免 *PublishKit is a…* |
| **不代发** | 「我故意不做自动发」 | *I still post manually on purpose* |
| **忌** | 赋能、矩阵、神器、大厂口吻 | virtual assistant、breeze、revolutionary |

| 少用（说明书 / 品牌腔） | 多用（坦白局） |
|------------------------|----------------|
| 内容日历上最容易混的两格 | 我骗过自己：改稿改嗨了，以为已经发了 |
| 本产品不支持自动发布 | 我故意不做自动发，怕风控也怕背锅 |
| PublishKit is a content calendar… | I built PublishKit after losing links in old chats |

**图上结构（Day Master 式「文字」块）：**

1. **主标题** = Pin 标题（最大）。
2. **副文案** = 正文去掉链接与价格后的 2～3 句（定价图保留价；小方图可保留「免费 50 条」）。
3. **关联词** = 最小字号一行。
4. **底部** = `PublishKit · 发稿匣` / `PublishKit`。

**画面气质：** Quiet Tool UI + **开发者自述海报**；不要 Stock 促销、不要「AI 炫光」、不要假团队合影。

改 [PublishKit-Pinterest营销文案.md](./PublishKit-Pinterest营销文案.md) 后，在本目录执行：

```bash
node scripts/sync-pinterest-pin-on-image-text.mjs
```

会把各节「文字」块刷成与营销文档一致（自动去掉网址与大部分价格句）。

---

## 使用说明

### 描述词怎么写（对齐 Day Master 海报式）

每一节描述词按这个顺序写，方便模型把**营销文案直接排进画面**：

1. **画布与气质**：竖版 2:3（1000×1500）、Quiet Tool Studio、参考哪张项目图片。
2. **画面构图**：背景、主视觉（窗口 / 侧栏 / 图标）、可读的界面词（仅限真实功能名）。
3. **文字**（单独列出，要求**逐字出现、不要改写**）：
   - **主标题**：与 [PublishKit-Pinterest营销文案.md](./PublishKit-Pinterest营销文案.md) 同一条 Pin 的**标题**一致（最大字号）。
   - **副文案**：2～4 句，摘自同一条 Pin 的**正文**精华，连着读成一段（营销感、场景感，但不说代发、不说自动发帖）。
   - **关联词行**：与该 Pin 的关联词一致，中间用「 · 」分隔，字号最小。
   - **底部品牌**：中文 Pin 写 `PublishKit · 发稿匣`；英文 Pin 写 `PublishKit`。
4. **收尾**：禁止项 +「请只输出这一张 1000×1500 的图」。

**结构示例（仅示范排版，不是本产品文案）：**

```text
竖版 2:3 … 主题为 …。整体空灵、克制、工具感。
背景为 …。中央 …
色彩 …
文字：
（主标题，最大）
（副文案句 1）
（副文案句 2）
（关联词 · 关联词 · 关联词）
底部统一说明：
PublishKit · 发稿匣
整体像现代编辑设计 + 效率工具海报，信息可读、留白充足。
```

画面上的字按 **独立开发者坦白局** 读完：先「我为啥做 / 我踩过啥坑」，再边界与用法。模型**不要改写**「文字」块，不要画网址。不写自动发帖；「不代发」用开发者口吻（我故意不做），不用冷冰冰免责声明。

不画进图的内容：Pin 链接、Chrome 商店链接、GitHub 地址、UTM、配对 Token、本机文件路径。这些留在 Pin 描述里。价格数字只出现在「定价区背景」那一条。其余图不写 ¥139、不写 $38。免费 50 条只写在小图那一条，因为那条正文就是在补这个额度。

界面里的文章正文用浅灰短线代替，不生成段落。样例稿（狗衣服、宠物测量、缝纫）不要出现。左侧导航只保留已经有的入口：来源文件、内容、素材、任务、今天、日历、设置。不要画「灵感」「报表」。不要画成自动发帖，不要画「发到所有平台」按钮。窗口用 Windows 细标题栏，不要 macOS 红黄绿按钮。

色彩默认：背景 **#F7F6F3**，面板 **#FFFFFF**，主字 **#1B1C19**，次级字 **#4E4638**，辅助字 **#8A8278**，金色点缀 **#B8924A**。黑底图沿用参考图的黑底，不改成暖米。字体：中文苹方或微软雅黑，英文 Inter，标题加粗。不要衬线花体，不要手写体。

每条都可追加的禁止项：兔子、宠物、狗、猫、缝纫、彩虹渐变、紫蓝 AI 风、霓虹、玻璃拟态、廉价促销贴、水印、乱码、网址、与参考图结构无关的装饰。

---

## publishkit_hero_desktop_2880x1620.png

参考图：左侧大片暖米留白，右侧一扇桌面窗口，窗内是发稿匣的任务列表。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。参考 `publishkit_hero_desktop_2880x1620.png` 的留白和窗口比例，改成竖图。
>
> 背景暖米 #F7F6F3，轻纸纹。上半约 42% 留白放标题。下半是一扇 Windows 桌面窗口，圆角 12px，轻阴影。窗内左侧窄导航，当前项是「任务」。右侧是稿件列表，正文用浅灰短线，不生成文章。窗口内可读的词只有：发稿匣、任务、今天、日历、内容、素材。
>
> 文字（必须逐字出现在画面上，不要改写，不要画网址）：
> 我做发稿匣，就因为总找不到「发过的那条链接」
>
> 跨境发帖这几年，我翻最多的是聊天记录，不是数据看板。
> 于是做了 PublishKit：Windows 上的内容日历 + Chrome 侧栏，复制、发帖、把链接记回来——**仍然是你自己点发送**，我不接平台密码。
> 自用顺了才上架给同路人。
>
> 内容日历 · 多平台发布 · 自媒体工具 · 小红书运营 · 内容排期 · 内容营销
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 标题 #1B1C19，副文 #4E4638，底部品牌名小一号。金色只用于导航当前项。不要价格，不要宠物，不要自动发布按钮。
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。构图与中文版相同：上半留白放英文标题，下半 Windows 窗口。界面语言改成英文，可读的界面词只有：PublishKit、Tasks、Today、Calendar、Content、Media。列表正文用浅灰短线。
>
> 背景 #F7F6F3，面板 #FFFFFF，字色 #1B1C19 / #4E4638，金色点缀 #B8924A。
>
> 文字（必须逐字出现在画面上，不要改写，不要画网址）：
> I built PublishKit after losing links in old chats
>
> I publish on multiple channels—I won’t hand passwords to another scheduler. PublishKit: local content calendar on Windows, Chrome side panel beside your tab, save the URL when it’s live. I still post manually on purpose. Not Buffer—honest.
>
> content calendar · social media content calendar · content planner · social media tracker · pinterest planner
>
> 底部统一说明：
> PublishKit
>
> 不要价格，不要宠物，不要 macOS 按钮，不要自动发布。
>
> 请只输出这一张 1000×1500 的图。

## publishkit_hero_mobile_1170x2532.png

参考图：暖米底正中一条窄侧栏，里面是待发卡片和一颗金色按钮。四周留白很大。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。参考 `publishkit_hero_mobile_1170x2532.png`：正中一条窄面板，像 Chrome 侧栏，圆角 12px，背景暖米 #F7F6F3。
>
> 顶部约 28% 放标题。面板里 3 条待发，每条只有一行短标题和浅灰副线。面板底部一颗金色按钮。可读的界面词只有：今天、待发、复制、标记已发布。三条短标题写：Pinterest 标题、小红书正文、YouTube 简介。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 一个人运营没有排期同事，只有「今天」清单
>
> 「今天发什么」是我给自己做的第一屏——还没发的都列在这，不花哨。
> 复制 → 贴进正在开的小红书 / Pinterest 编辑页 → 侧栏把链接写回。没有团队审批，也没有自动发。
> 你要是也手动多平台发稿，这套可能对你有用。
>
> 内容日历 · 今天发什么 · 内容排期 · 小红书运营 · 自媒体工具
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 不要价格，不要宠物，不要密码框，不要灵感或报表图标。
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一构图，界面改英文。面板顶部写 Today。三条短标题：Pinterest title、YouTube description、Instagram caption。按钮写 Copy 与 Mark published。背景 #F7F6F3，金色按钮 #B8924A。
>
> 文字（必须逐字出现在画面上，不要改写）：
> Solo maker’s Today list (no team scheduler)
>
> I built PublishKit’s Today view for mornings with no ops team—what’s still unposted, one screen. Copy from the side panel, post yourself, save the link. Manual multi-channel publishing only.
>
> social media tracker · content calendar · content planner · daily content plan
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## publishkit_icon_1024.png / publishkit_icon_1024新.png

参考图：白底圆角稿纸，三行短线，右下角一颗金色勾。黑描边。透明底。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景暖米 #F7F6F3。画面上半居中放大参考图 `publishkit_icon_1024新.png` 的图标：圆角稿纸、三行短线、右下金色勾 #B8924A、黑描边。图标上不要再写字。
>
> 图标下方留白。文字（必须逐字出现）：
>
> 发稿匣｜你发帖，电脑帮你记台账
>
> Windows 上的内容日历，旁边一条 Chrome 侧栏。稿、图、哪天发、发过没有，都在这台电脑上。
> 你还是自己写、自己发。它负责记住。插件在 Chrome 商店搜 PublishKit Companion。
>
> 发稿匣 · PublishKit · 自媒体工具 · 内容日历 · 内容营销
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 不要兔子，不要宠物，不要价格，不要把图标画成文件夹照片。
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。暖米底 #F7F6F3。上半同一枚图标：圆角稿纸、三行短线、金色勾。图标上无字。
>
> 文字（必须逐字出现）：
> Side project note: I don’t want your OAuth tokens
>
> I built PublishKit local-first—drafts and publish history on your PC. You still post in the browser; the Chrome panel hands you the right channel text. Not a matrix tool. PublishKit Companion on the Chrome Web Store.
>
> PublishKit · content calendar · content planner · social media tracker
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## publishkit_logo_lockup_2400x600.png

参考图：黑底。左侧发稿匣图标（白匣、稿纸、金色勾），右侧一条很细的白线，线上没有字。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。这一张沿用参考图的黑底，不要改成暖米。
>
> 上半：参考 `publishkit_logo_lockup_2400x600.png` 的图标，白匣、稿纸、金色勾 #B8924A，居中，图标本身不写字。细白线可以留在图标下方，很短。
>
> 下半用暖白米白字 #F5F3EE。文字（必须逐字出现）：
>
> 多平台发布｜一篇稿，各渠道各备一版
>
> 多平台发布最烦的是同一篇稿开了三个文档。小红书一套说法，Pinterest 一个短标题，YouTube 一段简介，放在同一条下面。
> 哪一版已经发出去，回来改一下状态。
>
> 多平台发布 · 内容日历 · 自媒体工具 · 小红书运营 · Pinterest
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 不要把右侧细线画成一串乱码字母。不要宠物。
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500，黑底。上半同一枚发稿匣图标。下半米白字 #F5F3EE。
>
> 文字（必须逐字出现）：
> I got tired of three docs per idea—built per-channel versions
>
> PublishKit’s content planner keeps Pinterest, YouTube, and Xiaohongshu copy on one item—tick what’s live. I still publish manually. Wrong tool if you want one paragraph blasted everywhere.
>
> content planner · social media content calendar · pinterest planner · youtube planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## publishkit_og_1200x630.png

参考图：左侧暖纸上一个发稿匣图标，右侧桌面窗口打开「内容」列表，底部一条金色细线。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。
>
> 上方标题。中部左侧是发稿匣图标（白匣、稿纸、金色勾）。右侧裁切一扇 Windows 窗口的一角，导航停在「内容」，列表行用浅灰短线。底部一条 4px 金色线 #B8924A。
>
> 文字（必须逐字出现）：
> 我骗过自己：改稿改嗨了，以为已经发了
>
> 做 content calendar 时我把「草稿」和「已发布+链接」硬拆开——渠道、日期、URL 各记各的。
> 你去平台亲手点发送，发完贴回链接。我故意不做自动发，怕风控也怕背锅。
>
> 内容日历 · 内容排期 · 多平台发布 · 发布记录 · 内容营销
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 窗口内不要生成邮件正文，不要文件路径，不要宠物。
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一构图。窗口导航写 Content。列表用灰线。底部金色细线。
>
> 文字（必须逐字出现）：
> I fooled myself editing—thought I’d already posted
>
> PublishKit splits drafts from proof (channel, date, URL). I post in the browser on purpose—no auto-publish. Save the link on the item so future-me stops guessing. Social media tracker for manual publishers.
>
> social media tracker · content calendar · content planner · publish checklist
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## publishkit_feat_today_1920x1080.png

参考图：桌面窗口，「任务」页分成建议先发、已排期、最近已发布。行内有语言标记和状态。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。顶部放标题。中下部一扇 Windows 窗口，占宽度约 88%。导航当前项「今天」。主区三个小标题：建议先发、已排期、最近已发布。每区两行，行内状态字用：待发、已排期、已发布。渠道只写小红书或 Pinterest。语言只写 中文 或 EN。行标题用灰线，不生成文章。金色只给一个「复制」按钮。
>
> 文字（必须逐字出现）：
> 同一条内容，Pinterest 发了小红书还没有——正常
>
> 我做「今天」时坚持：每个渠道单独一行状态，中英文也分开，不绑成一个假进度。
> 复制、网页里自己发、链接贴回来。数据在你电脑上——我懒得做云端账号托管那套。
>
> 内容日历 · 今天发什么 · 多平台发布 · 小红书运营 · 内容排期
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一窗口构图，界面改英文。三个分区：Suggested、Scheduled、Recently published。状态字：Ready、Scheduled、Published。渠道：Instagram、Pinterest。语言：ZH、EN。一个金色按钮写 Copy。
>
> 文字（必须逐字出现）：
> Pinterest live, Xiaohongshu not yet—I built rows for that
>
> PublishKit Today: one row per channel, ZH and EN separate. I publish manually in the browser, attach the URL. Local on your PC—I didn’t build another OAuth cloud.
>
> social media tracker · content calendar · social media content calendar · instagram planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## publishkit_feat_kanban_1920x1080.png

参考图：任务页打开看板，列是草稿、待发、阻塞、已发布。卡片可以拖。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。顶部标题。下方 Windows 窗口，导航停在「任务」，页内标题「看板」。三列竖排更适合竖图：草稿、待发、已发布。每列两张白卡片，卡片里只有两行灰线，不生成文章。其中一张卡片略微抬起，表示可以拖。不要画第四列功能说明。
>
> 文字（必须逐字出现）：
> 看板是我给自己的诚实反馈：还在草稿就是没发
>
> 三列：草稿、待发、已发布——拖过去才算数，我用来治「脑补已发」。
> 和周历、今天共用同一批任务。一个人做跨境内容，够用就行，不堆企业功能。
>
> 内容排期 · 内容日历 · 草稿 · 多平台发布 · 自媒体工具
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一看板，列名改成 Draft、Ready、Published。卡片内只有灰线。一张卡片略微抬起。
>
> 文字（必须逐字出现）：
> My kanban lie detector: still in Draft = didn’t ship
>
> PublishKit board won’t flatter you—Published only after you really posted. Same cards in Today and week view. Content planner for solo makers, not agency theater.
>
> content planner · social media content calendar · social media tracker · content calendar
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## publishkit_feat_calendar_week_1920x1080.png

参考图：日历页，按日期排的行，右侧是渠道和「待发」。导航停在「日历」。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。顶部标题。下方 Windows 窗口，导航停在「日历」，页内标题「日历」，旁边两个小开关写「月」「周」，「周」为当前。列表 6 行，每行左侧日期用 10-20 这种短日期，中间灰线，右侧只写：Pinterest · 待发、小红书 · 待发、YouTube · 待发。不要生成文章标题。
>
> 文字（必须逐字出现）：
> 我弃用长表排期，改成周历：只问哪天发哪条
>
> 表格拉长后我对不上 Pin 和小红书哪天发——周历只记「计划日」。
> 到那天从「今天」复制，平台网页里自己点发送，再标已发。我**故意不做**定时代发，账号风控自己扛。
>
> 内容日历 · 内容排期 · 多平台发布 · 小红书运营 · Pinterest排期
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 不要画定时发送，不要画登录平台的按钮。
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一周历。页标题 Calendar。开关 Month / Week，Week 为当前。行尾只写：Pinterest · to post、YouTube · to post。
>
> 文字（必须逐字出现）：
> I quit long spreadsheets for a week view—one question only
>
> Which day is this Pin meant to go out? PublishKit content calendar stores that. I copy from Today, post myself, mark done. No OAuth, no midnight auto-post—by design.
>
> content calendar · social media content calendar · content planner · youtube planner · pinterest planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## publishkit_feat_content_inspector_1920x1080.png

参考图：内容库。中间是一篇稿的标题和正文，右侧是配图网格，上面有复制按钮。导航停在「内容」。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。顶部标题。下方 Windows 窗口，导航停在「内容」。中间一篇稿：一行标题灰条，下面数行长短灰线，像 Markdown 的小标题和段落，不生成真实句子。右侧 4 个空的暖灰图片格，不要照片里的动物。上方一颗按钮写「复制正文」。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 粘贴翻车太多，才在桌面加了「先看一遍」
>
> 长文 Markdown 在桌面过一眼再贴——Pin 标题和笔记正文不是同一段，我吃过亏。
> 结构改好，侧栏按渠道复制。小功能，但独立开发就爱修这种具体痛点。
>
> 内容排期 · 小红书运营 · Pinterest标题 · 多平台发布 · 内容日历
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一内容页，界面改英文。导航 Content。按钮 Copy text。正文区只有灰线，表示标题层级和段落长短。右侧 4 个空图片格。
>
> 文字（必须逐字出现在画面上，不要改写）：
> Too many paste regrets—I read on desktop first now
>
> Pinterest planner line ≠ YouTube wall of text. I fix structure on the PC, then PublishKit’s side panel copies what matches your open tab. Small feature, real scar tissue.
>
> content planner · pinterest planner · youtube planner · social media content calendar
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## publishkit_feat_content_inspector_1920x1080英文.png

参考图：同一内容页，稿子区域是英文结构。这条中文 Pin 也用英文稿做画面，因为卖点是「英文稿先看结构」。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。海报标题用中文，窗口里的稿子区域保持英文结构：几行灰线，其中两行稍重，表示英文字幕标题。导航停在「内容」。右上角两个小标签写 ZH、EN，EN 为当前。不要生成英文段落，不要宠物照片。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 跨境发帖：英文简介和 Pin 标题我分开存
>
> 中英各一条，发没发各记各的——这是我做跨境时的硬需求，不是锦上添花。
> 桌面先看结构，侧栏再复制。同路人应该懂。
> 英文站：https://get.pongrabbit.com/en/?utm_source=pinterest&utm_medium=social&utm_campaign=pin_2026
> **关联词（中文）：** 内容日历、多平台发布、YouTube简介、Pinterest、内容排期
> **Title:** Cross-border posting: I store EN and ZH separately
> **Description:**
> YouTube blurb ≠ Pinterest title—I learned that the hard way. PublishKit keeps languages on separate items with their own posted-or-not. Desktop first, side panel copy second. Built for people like me.
>
> 内容日历 · 多平台发布 · YouTube简介 · Pinterest · 内容排期
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。窗口界面全英文。导航 Content。标签 ZH 与 EN，EN 为当前。正文区灰线表示 heading 和 paragraph。按钮 Copy text。
>
> 文字（必须逐字出现在画面上，不要改写）：
> Cross-border posting: I store EN and ZH separately
>
> YouTube blurb ≠ Pinterest title—I learned that the hard way. PublishKit keeps languages on separate items with their own posted-or-not. Desktop first, side panel copy second. Built for people like me.
>
> youtube planner · pinterest planner · content planner · content calendar
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## publishkit_feat_extension_1920x1080.png

参考图：浏览器右侧一条发稿匣侧栏，左侧网页被虚化。侧栏里有复制和标记已发布。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。左侧大半是虚化的发帖页，不要出现任何平台商标，不要可读正文。右侧贴边一条白侧栏，圆角。侧栏顶部写 PublishKit。下面两条任务，渠道分别写 Pinterest、小红书。每条有按钮「复制富文本」和金色按钮「标记已发布」。不要画端口号，不要画 Token，不要画配对码。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 插件是我后来加的：发帖页旁就得有稿子
>
> 桌面写排期，浏览器发帖——中间少不了一个侧栏。发稿匣助手复制**这一渠道**那段，不碰密码，只连本机 127.0.0.1。
> 先 Windows 桌面，再装插件配对。独立开发常见组合：本地核心 + 浏览器现场。
>
> 小红书运营工具 · 自媒体工具 · Chrome插件 · 多平台发布 · 内容日历
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一侧栏构图。侧栏顶部 PublishKit。两条渠道：Pinterest、YouTube。按钮：Copy rich text、Mark published。左侧发帖页虚化，无商标。不要端口，不要 Token。
>
> 文字（必须逐字出现在画面上，不要改写）：
> The extension came second—I needed copy beside the composer
>
> PublishKit Companion: this channel’s text beside your post box, localhost only, no passwords. Desktop app first, pair once, save the tab URL when live. Typical indie stack: local core + browser glue.
>
> content planner · social media tracker · pinterest planner · content calendar
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## publishkit_feat_channel_pack_1920x1080.png

参考图：一条任务打开后，有复制按钮，以及「导出渠道发布包」，旁边一个小卡片列出文件夹里的文件名。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。顶部标题。下方 Windows 窗口，导航停在「任务」。主区一条内容，下面三个并排的小卡片，卡片标题只写：Pinterest、小红书、YouTube。每张卡片里两行灰线。卡片上方一行按钮，只写：复制富文本、导出渠道发布包。导出按钮用金色描边。不要生成脚本或文章。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 渠道发布包：我给每个平台单独备文件夹
>
> 同一段糊全场我试过，Pin 太长、笔记太像标题——所以在同一条下按渠道拆开，还能导出发布包。
> 发完记哪版上线；发送仍在你自己的平台页面。要一键矩阵的请绕道。
>
> 多平台发布 · 小红书运营 · Pinterest标题 · YouTube简介 · 内容日历
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一窗口。三张卡片标题：Pinterest、YouTube、Instagram。按钮：Copy rich text、Export channel pack。卡片内只有灰线。
>
> 文字（必须逐字出现在画面上，不要改写）：
> I stopped one-paragraph blast—built channel packs instead
>
> PublishKit: Pinterest line, YouTube blurb, Reddit tone on one item—copy or export a channel pack. I still publish manually. Wrong tool for matrix auto-blast.
>
> pinterest planner · youtube planner · social media content calendar · content planner · instagram planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## publishkit_flow_3step_2400x480.png

参考图：暖米底上三个细线图标横排。左：一页清单。中：一个浏览器窗口。右：一个圆圈里的勾。图标之间没有字。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3，轻纸纹。参考 `publishkit_flow_3step_2400x480.png` 的三个细线图标，改成竖向三行，线色 #1B1C19，线很细。不要填色，不要 3D。
>
> 顶部标题。三行各自是：
> 1. 清单页图标，图标下写「先把这一周排进内容日历」
> 2. 浏览器窗口图标，图标下写「再到网页里贴上，自己点发送」
> 3. 圆圈勾图标，勾用金色 #B8924A，图标下写「发出去之后，把链接写回来」
>
> 文字（必须逐字出现在画面上，不要改写）：
> 我的发文闭环就三步（故意没有第四步代发）
>
> 排进内容日历 → 平台网页自己发 → 侧栏记链接。没有「授权我们发帖」——这是我产品边界，不打算改。
> 怕风控、不想交 OAuth 的独立开发者，可能是同路人。
>
> 内容日历 · 多平台发布 · 内容排期 · 自媒体工具 · 小红书运营
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 不要加第四步。不要画代发。
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一套三个细线图标，竖向排列。勾是金色 #B8924A。
>
> 文字（必须逐字出现在画面上，不要改写）：
> My publish loop: plan, post yourself, save the link—no step four
>
> PublishKit: calendar → browser post → attach URL. I won’t add auto-publish—that’s the product boundary. Built for manual publishers who fear OAuth more than extra clicks.
>
> content calendar · content planner · social media tracker · social media content calendar
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## publishkit_trust_local_800x600.png

参考图：暖纸上的线稿笔记本电脑，屏幕里一个数据库圆筒。电脑后方一朵虚线云，云上一个叉。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。参考 `publishkit_trust_local_800x600.png` 的线稿：一台打开的笔记本，屏幕中央一个数据库圆筒，细线 #1B1C19。笔记本后方一朵虚线云，云中间一个叉，叉旁一颗小金点 #B8924A。不要画叶子，不要画数据中心机房。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 坦白：数据在 SQLite，因为我也不信第三方云
>
> 稿和发布记录在本机；平台登录不接。插件只连 127.0.0.1——换机自己备份。
> 做工具的人先说服自己，再上架给别人。GitHub 可下安装包，自行判断要不要用。
>
> 自媒体工具 · 多平台发布 · 内容日历 · 小红书运营 · 内容排期
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一线稿：笔记本、屏幕里的数据库圆筒、后方打叉的虚线云、一颗金点。
>
> 文字（必须逐字出现在画面上，不要改写）：
> Honest architecture: SQLite on your PC, localhost extension
>
> I built PublishKit local-first because I don’t trust handing every channel to a SaaS. No OAuth. Extension → 127.0.0.1 only. Back up when you switch machines. Judge from GitHub if you want.
>
> content calendar · social media tracker · content planner · pinterest planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## screenshot_01_sidepanel_1280x800.png / cws04_sidepanel_1280x800.png

参考图：浏览器右侧侧栏，列出今天待发。左侧网页虚化。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。左侧虚化发帖页，无商标、无可读正文。右侧白侧栏。侧栏标题「今天」。下面 4 行，每行一个渠道字：Pinterest、小红书、YouTube、Instagram，后面跟一条灰线。不要端口，不要 Token。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 侧栏和桌面「今天」同源——少一次 Alt+Tab
>
> 写插件就是为了发帖时不切窗口。列表跟桌面同步，复制、标记都在浏览器里完成。
> 桌面 → 插件 → 配对，三步。小团队就我一个人，流程也按一个人设计。
>
> 小红书运营工具 · 自媒体工具 · 内容日历 · 今天发什么 · Chrome插件
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一侧栏。侧栏标题 Today。四行渠道：Pinterest、YouTube、Instagram、Reddit，各跟一条灰线。
>
> 文字（必须逐字出现在画面上，不要改写）：
> I hate alt-tab while posting—built the side panel for that
>
> PublishKit Companion mirrors desktop Today beside your composer. Desktop → extension → pair. Designed for a team of one (me).
>
> social media tracker · content calendar · content planner · pinterest planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## cws05_copy_toast_1280x800.png

参考图：侧栏里一条任务，金色按钮是「复制富文本」，右上角有一颗黑胶囊写「已复制」。旁边还有「复制正文」「当前页」「标记已发布」。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。右侧白侧栏，左侧虚化页面。侧栏顶部一颗黑胶囊，白字「已复制」。下面一条任务，渠道写 Pinterest。按钮逐字写：复制富文本、复制正文、当前页、标记已发布。复制富文本用金色 #B8924A。任务标题用一条灰线，不生成句子。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 复制错渠道是我自己的锅，所以做了「按渠道复制」
>
> 小红书口吻贴进 Pin 我干过，丢人但真实。侧栏点复制就是这一版，不是隔壁渠道。
>
> 多平台发布 · 小红书运营 · Pinterest · 内容排期 · 自媒体工具
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一侧栏。黑胶囊白字 Copied。渠道 Pinterest。按钮：Copy rich text、Copy text、Current page、Mark published。Copy rich text 为金色。
>
> 文字（必须逐字出现在画面上，不要改写）：
> I pasted the wrong channel once—shipped per-channel copy
>
> PublishKit puts the Pinterest (or Instagram) variant on your clipboard—not the Xiaohongshu one. My bug, my fix.
>
> pinterest planner · instagram planner · social media content calendar · content planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## cws06_publish_1280x800.png

参考图用途：侧栏里标记已发布，并带上当前页链接。画面重点是「标记已发布」和链接一栏，不是复制提示。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。右侧白侧栏。一条任务，渠道写 Pinterest。有一个空的输入行，标签写「发布链接」。一颗小按钮写「当前页」。最下面一颗宽的金色按钮写「标记已发布」。不要画成已经打开的平台后台。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 你点发送，我（工具）只帮你留住 URL
>
> 发布记录我不做云端同步——标记已发布时抓当前标签链接，以后搜任务就能找到。
> 发送不经我服务器，这是架构选择，不是临时限制。
>
> 发布记录 · 内容日历 · Pinterest · 多平台发布 · 自媒体工具
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 输入行里不要生成真实网址，用一条灰线表示已经填上。
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一侧栏。标签 Publish link。小按钮 Current page。金色宽按钮 Mark published。链接行是一条灰线，不要真实网址。渠道 Pinterest。
>
> 文字（必须逐字出现在画面上，不要改写）：
> You hit Post—I only persist the URL (on your machine)
>
> PublishKit saves the tab link when you mark done—publish on the platform, not through me. Social media tracker for people who don’t trust cloud history search.
>
> social media tracker · content calendar · pinterest planner · content planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## cws07_duplicate_warn_1280x800.png

参考图用途：同一条链接已经在记录里时，侧栏先提醒，不另起一行。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。右侧白侧栏。侧栏中部一张浅金底提示条 #F0EDE6，描边 #D1C5B3。提示条里只有这一句：「这个链接已经记过」。提示条下方两个按钮：「留下这一条」「取消」。不要画红色警报，不要画叉叉爆炸。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 重复记链接是我会犯的蠢错，于是做了提醒
>
> 连点「已发布」日历就脏——插件发现 URL 已存在会先拦一下。小功能，来自真实手滑。
>
> 内容日历 · 发布记录 · 多平台发布 · 内容排期 · 自媒体工具
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一提示条，浅金底。提示条内只写：This link is already saved。按钮：Keep this one、Cancel。
>
> 文字（必须逐字出现在画面上，不要改写）：
> I double-clicked “published” too often—added a warning
>
> PublishKit warns if the URL’s already on your content calendar. One post, one row—scar tissue from my own messy ledger.
>
> social media tracker · content calendar · content planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## cws08_pairing_1280x800.png

参考图用途：桌面和插件配对。真实截图里有端口和一长串 Token，新图不要把那些画出来。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。画面中部一张白卡片，圆角 12px。卡片标题「配对」。一个输入框，框内用圆点占位，标签写「配对码」。两个按钮：「保存」「测试连接」，测试连接为金色。卡片下方四个短步骤横排小字：装桌面版、装插件、配对、再复制。不要画端口数字，不要画长串 Token，不要画二维码。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 配对码看着土，但稿子真的不出这台电脑
>
> 侧栏连桌面走 127.0.0.1，是我能接受的简单方案。装桌面、装插件、配对、复制——不优雅，但透明。
>
> 自媒体工具 · Chrome插件 · 小红书运营工具 · 内容日历 · 多平台发布
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一白卡片。标题 Pair。输入框标签 Pairing code，框内圆点。按钮 Save、Test connection，后者金色。卡片下四步：Desktop app、Extension、Pair、Copy。不要端口，不要 Token。
>
> 文字（必须逐字出现在画面上，不要改写）：
> Ugly pairing code, honest localhost sync
>
> PublishKit pairs extension to desktop over 127.0.0.1—your drafts don’t hit my servers. Not elegant; transparent. Desktop, extension, pair, copy.
>
> content planner · content calendar · social media tracker
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## cws03_marquee_1400x560.png / Marquee Promo 1400×560.png / Marquee Promo 1400×560

参考图：左侧发稿匣图标，中间桌面日历窗口，右侧侧栏贴着「标记已发布」。横幅很宽。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。把参考横幅收成竖向三段：上段标题；中段一枚发稿匣图标（白匣、金色勾）；下段一条窄侧栏，上面一条金色按钮写「标记已发布」，再一条白按钮写「复制富文本」。不要把整张 1400×560 横图硬塞进竖图。窗口里的稿件标题用灰线，不要宠物句子。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 助手插件：我只做复制和回填，不碰「发布」按钮
>
> 发帖页旁边递稿子，发完同步状态——**不代你点发布**。独立开发边界写死在产品里。Chrome：PublishKit Companion。
>
> Chrome插件 · 自媒体工具 · 小红书运营工具 · 内容日历 · 多平台发布
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一竖向三段。侧栏按钮：Mark published（金色）、Copy rich text。
>
> 文字（必须逐字出现在画面上，不要改写）：
> Companion plugin: copy + mark posted, never hits Post
>
> PublishKit Companion beside your composer—I refused to add auto-publish. Copy prepared text, mark posted when the link exists. Hard boundary for a side project.
>
> social media tracker · content calendar · pinterest planner · content planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## promo_small_440x280.png / Small Promo Tile 440×280.png

参考图很小：左边图标，右边侧栏一角。正文要补上「免费 50 条」。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。上半标题和三行说明。下半左侧发稿匣图标，右侧一张缩小的侧栏卡片，卡片上只写「复制富文本」「标记已发布」，其余用灰线。这一条可以写免费额度，不要写买断价格。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 我仍在各平台官网发帖——工具只负责别乱
>
> Windows 里排 content calendar，发仍去 Pinterest / 小红书官方页。
>
> 内容日历 · 自媒体工具 · 多平台发布 · 小红书运营 · 内容排期
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一构图。侧栏按钮 Copy rich text、Mark published。可以写 50 items，不要写美元价格。
>
> 文字（必须逐字出现在画面上，不要改写）：
> I still post on the official sites—tool only kills chaos
>
> Plan on PublishKit’s content calendar; publish on Pinterest or YouTube like always. 50 free items. Skip this if you want a scheduler—I won’t build that.
>
> content calendar · content planner · social media content calendar · youtube planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## 定价区背景 1920×800.jpg

参考图几乎是一张空白暖纸，没有界面。这一条只做文字海报，数字以文案为准。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。参考 `定价区背景 1920×800.jpg` 的空白暖纸纹理，铺满整张，颜色 #F7F6F3。不要放软件窗口，不要放图标墙。
>
> 上方两句说明，字号中等，先讲为什么是一次买断。中间两个大数字块上下排列，数字用 #1B1C19，单位用 #4E4638。两个块之间一条短的金色线 #B8924A。数字下面再接两句小字。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 定价坦白：我发得不多，不想月月给 Buffer 交钱
>
> Pro 买断 ¥139 / 
>
> 底部统一说明：8 首发——Free 50 条先走完闭环。邮件里 `PKPRO-` 激活。不含代发；我要的是台账，不是替平台打工。
> 价格以官网为准。
>
> 自媒体工具 · 内容日历 · 多平台发布 · 内容排期 · 小红书运营工具
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 不要画划掉的月费，不要画别的价格，不要画优惠券。
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一张空白暖纸。上方两句说明，中间两个数字块，中间一条短金线，数字下面再接两句小字。不要软件窗口。
>
> 文字（必须逐字出现在画面上，不要改写）：
> Pricing honesty: I post manually, so I ship lifetime Pro
>
> PublishKit—50 free items, Pro one-time (
>
> 底部统一说明：8). Buffer alternative only if you don’t need auto-post (I don’t). `PKPRO-` after checkout. Check site for today’s price.
>
> content planner · content calendar · social media tracker · buffer alternative
>
> 底部统一说明：8 once.
>
> PublishKit: 50 items free, Pro one-time (
>
> 底部统一说明：8 launch)—a buffer alternative if you post manually anyway. No auto-post. Enter `PKPRO-` after checkout. Check the site for today’s price.
>
> content planner · content calendar · social media tracker · buffer alternative
>
> 底部统一说明：8 once.
>
> PublishKit: 50 items free, Pro one-time (
>
> 底部统一说明：8 launch)—a buffer alternative if you post manually anyway. No auto-post. Enter `PKPRO-` after checkout. Check the site for today’s price.
>
> content planner · content calendar · social media tracker · buffer alternative
>
> 底部统一说明：8 once.
>
> PublishKit: 50 items free, Pro one-time (
>
> 底部统一说明：8 launch)—a buffer alternative if you post manually anyway. No auto-post. Enter `PKPRO-` after checkout. Check the site for today’s price.
>
> content planner · content calendar · social media tracker · buffer alternative
>
> 底部统一说明：8 once—when monthly schedulers stack up
>
> PublishKit vs Buffer-style bills: 50 items free on your content planner, Pro one-time (
>
> 底部统一说明：8 launch). You still post yourself—enter `PKPRO-` under Settings → License after checkout. Check the site for today’s price.
>
> content planner · content calendar · social media tracker · buffer alternative

>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## 11.jpg

参考图：真实的内容库。左边导航，中间一篇稿，右边配图。新图保留这个版式，稿子正文改成灰线。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。参考 `11.jpg` 的三栏：左导航、中正文、右配图。收成竖图后，导航改成窗口顶部的一行小字：内容。中间是标题区和数行灰线。右下 4 个空的暖灰图格，不要动物、不要衣服、不要真人。上方按钮写「复制富文本」。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 内容库是我给自己做的「别在文件夹里考古」
>
> 字和图同屏，复制在上，发没发记在任务/日历——不做花里胡哨的协作，就解决我每天发稿那 90 秒。
> Windows 桌面，GitHub 有源码，自行判断。
>
> 内容日历 · 自媒体工具 · 多平台发布 · 内容排期 · 内容营销
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一三栏收成竖图。顶部小字 Content。按钮 Copy rich text。正文灰线，4 个空图格。
>
> 文字（必须逐字出现在画面上，不要改写）：
> I built the library so I stop digging through folders
>
> PublishKit: text + images one screen, status on tasks/calendar. No agency features—just the 90 seconds I save daily. Windows desktop, code on GitHub.
>
> content planner · content calendar · social media content calendar · social media tracker
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## 33.jpg

参考图：真实侧栏，同一屏上能看到 Pinterest、小红书、Instagram 三条，每条有复制富文本和标记已发布。新图保留这三行，不要保留样例标题。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。左侧虚化网页。右侧白侧栏占画面约 46%。侧栏里正好三条，渠道名逐字写：Pinterest、小红书、Instagram。每条下面两个按钮：「复制富文本」「标记已发布」。标题位置用灰线。不要端口，不要 Token，不要样例稿标题。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 侧栏按渠道排队——我做多平台时的笨办法
>
> 这条 Pin、下条笔记、再下 Instagram，各复制各标记。「当前页」填 URL，少复制几次链接。笨但稳，适合手动发稿党。
>
> Pinterest · 小红书运营 · 多平台发布 · Instagram · 内容日历
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一侧栏三条。渠道名：Pinterest、Instagram、YouTube。按钮：Copy rich text、Mark published。标题用灰线。
>
> 文字（必须逐字出现在画面上，不要改写）：
> Channel queue in the side panel—my dumb reliable hack
>
> PublishKit lists today by channel beside your tab. “Current page” for the URL—low tech, works for manual multi-platform posting like mine.
>
> pinterest planner · instagram planner · social media tracker · social media content calendar
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## ChatGPT Image 2026年8月24日 17_55_33.png

参考图：纯黑底，中央一个白色发稿匣，匣里叠着稿纸，右下金色圆勾。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。这一张沿用参考图的纯黑底。上半居中放大那个白色发稿匣：叠放的白稿纸、一条金色竖条、右下金色圆勾、白勾。图标上不要写字。
>
> 下半用 #F5F3EE。文字（必须逐字出现）：
>
> 待发的稿收一处，发完打勾
>
> 内容日历如果散在聊天记录里，这一周就没法看。发稿匣把还没发的条目收在电脑上。发出去之后打个勾，这条才离开待发。
>
> 内容日历 · 内容排期 · 自媒体工具 · 多平台发布
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 不要改成暖米底，不要加第二套图标。
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500，纯黑底。上半同一发稿匣图标。下半米白字 #F5F3EE。
>
> 文字（必须逐字出现在画面上，不要改写）：
> My drafts used to live in chats—now one tray
>
> PublishKit: unposted in one place, check = really live. No vanity metrics—solo maker content calendar.
>
> content calendar · social media tracker · content planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## ChatGPT Image 2026年8月24日 18_03_16.png

参考图是概念界面，左侧有参考图里不存在于产品的菜单。新图不要把那些菜单画出来。只保留「今天的数量」这一层意思。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。一扇 Windows 窗口。顶部三个并排数字块，白底，圆角 8px。数字用大号 #1B1C19，标签在数字下方。三个标签只写：已发、待发、草稿。数字可以是 4、7、3。数字块下面是 5 行列表，每行一条灰线和一个状态小字，状态只在这三词里挑。左侧不要任何图标导航。不要灯泡，不要柱状图，不要 macOS 按钮。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 早上先数三个数：已发、在等、还在写
>
> 不靠感觉排今天——点进去看渠道和状态。我一个人运营就靠这张晨间快照。
>
> 内容日历 · 今天发什么 · 内容排期 · 多平台发布 · 自媒体工具
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一窗口和三个数字块。标签改成：Published、Waiting、Draft。列表状态只用这三个英文词。无图标侧栏，无 macOS 按钮。
>
> 文字（必须逐字出现在画面上，不要改写）：
> Morning ritual: count live, waiting, drafting
>
> PublishKit morning numbers before I panic-post. Channel + status per row—built for a team of one.
>
> social media tracker · content calendar · content planner · social media content calendar
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## ChatGPT Image 2026年8月26日 12_49_52.png

参考图：中间一块白卡片，四角各有一个圆点用细线连到中心，其中一角是金色勾。原图偏冷灰，新图改回暖米，结构保持。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景改成暖米 #F7F6F3，不要冷灰，不要渐变。画面中部一块白圆角卡片，四角四个圆，细线连到卡片，线色 #8A8278。三个圆是白底空心。右上角那个圆是金色 #B8924A，里面一个白勾。四个圆旁边各写一个渠道名：小红书、Pinterest、YouTube、Instagram。金色勾旁边写 Pinterest。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 「发过了」——我对自己说过谎，其实只发了一个渠道
>
> 四个平台四个勾，勾上才算。同一条内容别假装全渠道已发——这是我做状态拆分的理由。
>
> 多平台发布 · 内容日历 · 小红书运营 · Pinterest · YouTube
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。暖米底。中心白卡片，四条细线，三个空心圆，一个金色勾。四个名字：Xiaohongshu、Pinterest、YouTube、Instagram。金色勾旁写 Pinterest。
>
> 文字（必须逐字出现在画面上，不要改写）：
> I lied: “it’s published” (on one channel only)
>
> PublishKit ticks per channel—four boxes, four truths. Why I split status instead of one fake “done.”
>
> social media content calendar · pinterest planner · youtube planner · instagram planner · content planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## ChatGPT Image 2026年8月26日 12_51_39.png

参考图：黑底。左边发稿匣图标，右边三条没有字的金色细横线。横线上不要编功能名，只写三个渠道。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。沿用参考图的黑底。上方标题用 #F5F3EE。中部左侧发稿匣图标（白匣、金色勾）。右侧三条细横线，金色 #B8924A，线很细。每条线的左端写一个词，不要写日期，不要写功能名：
>
> Pinterest
> 小红书
> YouTube
>
> 文字（必须逐字出现）。三条渠道名留在横线左端：
> 周历右边我只写三件真能发的活
>
> 左边稿库，右边今天 Pin / 明天笔记 / 周五 YouTube——不写占位符。独立开发排期就图别骗自己。
>
> 内容日历 · 内容排期 · Pinterest · 小红书运营 · YouTube
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500，黑底。左发稿匣图标，右三条金色细线。线端三个词：Pinterest、Instagram、YouTube。不要日期。
>
> 文字（必须逐字出现在画面上，不要改写）：
> Left: drafts. Right: three posts I’ll actually ship
>
> No placeholder calendar theater—PublishKit week view with real Pin, caption, YouTube dates. Indie scheduling: don’t lie to yourself.
>
> content calendar · content planner · pinterest planner · youtube planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## ChatGPT Image 2026年8月26日 12_54_10.png

参考图：竖长列表，有的行是金色勾，有的行是空心圆，底部有复制和完成。原图左侧有一排概念图标，新图不要那些图标。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。画面中部一条竖向白面板，圆角 12px。面板里 7 行。每行左侧一个小灰块当缩略图，中间一条灰线，右侧要么是金色勾 #B8924A，要么是空心圆。大约一半有勾。不要照片内容，不要动物。面板底部两个按钮：「复制」「标记完成」，标记完成是金色。面板左侧不要图标栏，不要灯泡，不要柱状图，不要 macOS 按钮。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 金勾 = 我真的在平台上点过发送了
>
> 空圈还在等；金勾才是 live。复制在底部，发送仍去官网亲手点——我不替你按按钮。
>
> 内容日历 · 今天发什么 · 内容排期 · 多平台发布 · 自媒体工具
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一竖列表面板。行尾是金色勾或空心圆。底部按钮 Copy、Mark done。无图标侧栏，无 macOS 按钮。
>
> 文字（必须逐字出现在画面上，不要改写）：
> Gold check = I actually clicked Post on the site
>
> Empty circle = still waiting. Check = live. PublishKit won’t ghost-post—I publish on Pinterest/XHS myself.
>
> social media tracker · content calendar · content planner · social media planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## ChatGPT Image 2026年8月26日 12_55_53.png

参考图：左边是图标，右边是列表。和纯黑底那张图标 Pin 分开，这张用暖米底，让两张封面不要长得一样。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。左上是发稿匣图标，白匣、稿纸、金色勾，高度约占画面 22%。图标右侧和下面是一张白列表面板，6 行。每行一条灰线，行尾小字只在「这周要发」「已经过去」里交替。不要登录框，不要密码，不要平台商标。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 我不要你的登录框，只要你的稿子
>
> 工具打开是待发列表，不是绑号向导。密码留在浏览器——我做的是发稿台账，不是矩阵 SaaS。
>
> 自媒体工具 · 内容日历 · 内容排期 · 多平台发布
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。暖米底。左上同一图标。右侧白列表 6 行，行尾只交替写 This week 与 Posted。无密码框。
>
> 文字（必须逐字出现在画面上，不要改写）：
> Inbox for copy—not a login carnival
>
> PublishKit opens to drafts due this week, not eight OAuth screens. I build publish ledgers, not matrix SaaS.
>
> content planner · content calendar · social media tracker
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

## ChatGPT Image 2026年8月29日 09_55_47.png

参考图：一张概念列表，行上有 ready、scheduled、published。原图左侧有概念图标，渠道图标也较多。新图只保留三种状态，渠道写成文字，不声称支持图上每一个图标。

### 中文 Pin

> 竖版 2:3 Pinterest Pin，画布 1000×1500 像素。背景 #F7F6F3。一扇 Windows 窗口里只有一张列表，没有左侧图标栏。8 行。每行：一条灰线、一个渠道字、一个状态胶囊。渠道只在这四个词里换：小红书、Pinterest、YouTube、Instagram。状态胶囊只写这三个词，各用各的颜色：待发用 #3A6A4A，已排期用 #B8924A，已发布用 #2563EB。胶囊字是白的。不要灯泡，不要柱状图，不要 macOS 按钮，不要一排认不出的商标。
>
> 文字（必须逐字出现在画面上，不要改写）：
> 排期日到了≠已发布——我写在产品里防自欺
>
> Scheduled 只是你选的日期；Published 才是你点过发送且链接记上了。手动发稿的人都懂这差别。
>
> 内容日历 · 内容排期 · 多平台发布 · 发布记录 · 自媒体工具
>
> 底部统一说明：
> PublishKit · 发稿匣
>
> 请只输出这一张 1000×1500 的图。

### 英文 Pin

> 竖版 2:3，1000×1500。同一列表，无图标侧栏。状态胶囊只写：Ready、Scheduled、Published。颜色同上：绿 #3A6A4A、金 #B8924A、蓝 #2563EB。渠道词：Pinterest、YouTube、Instagram。
>
> 文字（必须逐字出现在画面上，不要改写）：
> Scheduled date ≠ published—I coded that distinction on purpose
>
> PublishKit: published = you posted + link stored. Manual publishers need honest content calendar states—not scheduler fantasy.
>
> content calendar · social media content calendar · social media tracker · content planner
>
> 底部统一说明：
> PublishKit
>
> 请只输出这一张 1000×1500 的图。

---

## 出图时对着看

- **语气** = 独立开发者坦白局（§0.3）；「文字」块与 [PublishKit-Pinterest营销文案.md](./PublishKit-Pinterest营销文案.md) v1.6 **逐字一致**（改文案后跑同步脚本）。
- 画面上没有网址，没有 Token，没有 ¥139 / $38，**除了「定价区背景」那一张**。
- **免费 50 条**只出现在 `promo_small` / Small Promo Tile 那一节。
- 导航里没有灵感，没有报表；窗口用 **Windows** 细标题栏，没有红黄绿按钮。
- 界面正文是灰线，不是样例文章，也不是狗或衣服（`11.jpg` 样例稿勿复述）。
- 黑底只留给锁标、`publishkit_logo_lockup`、`ChatGPT Image … 17_55_33`、`… 12_51_39`。其余背景 **#F7F6F3**。
- 出图顺序建议：先发 §3 角色设定（见 [PublishKit-营销视觉AI描述词.md](../PublishKit-营销视觉AI描述词.md)）→ 上传该节**参考图** → 粘贴对应 **中文 Pin** 或 **英文 Pin** 整段描述词。

---

## 修订记录

| 版本 | 日期 | 说明 |
|------|------|------|
| v1.6 | 2026-10-09 | 语气锁定「独立开发者坦白局」；§0.3 与营销 v1.6 §三 对齐；同步脚本保留坦白句、仅剥价格 |
| v1.4 | 2026-10-09 | 新增 §0 产品/竞品；「文字」块同步脚本 |
| v1.2 | 2026-10-09 | 统一「文字」块与 Day Master 式排版说明 |
| v1.1 | 2026-10-08 | 首版：与 Pinterest 营销文案逐图对应，1000×1500 竖 Pin |
