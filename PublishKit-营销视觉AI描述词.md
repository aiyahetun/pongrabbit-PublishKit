# PublishKit · Logo 与营销视觉 AI 描述词

| 属性 | 内容 |
|------|------|
| 文档版本 | v1.0（2026-08-24） |
| 对齐规范 | [docs/设计标准-PublishKit.md](./docs/设计标准-PublishKit.md) · **Quiet Tool Studio** |
| 用途 | 生成 PublishKit（发稿匣）品牌 Logo、应用图标、官网 Hero、社媒配图、Product Hunt 封面等营销视觉 |
| 品牌关系 | 与彭兔子制版 / PongRabbit Studio（Warm Paper Studio）同属创作者工具生态，**更克制、更工具感** |
| 产品定位 | 本地优先的跨境营销内容资产与发布状态管理工具（桌面端 + 浏览器插件） |

> **AI 生图通则**：PublishKit 是**效率型内容运营工具**，不是营销官网、不是 AI 炫光风、不是宠物服饰电商。画面气质应对齐 Linear / Notion / Eagle：**列表、面板、克制留白、暖金点缀**。全部配图默认**无文字、无水印、无价格标**（文案后期叠加）；若需含产品名，仅在 Logo / 封面类条目中单独说明。

---

## 1. 品牌视觉速查（生成前必读）

### 1.1 对外名称

| 场景 | 中文 | 英文 |
|------|------|------|
| 产品全称 | PublishKit / 发稿匣 | PublishKit |
| 一句话 | 把文案、素材、渠道、发布状态管成一套本地工作台 | Local-first content ops: copy, media, channels, publish status |
| 不做 | 营销账号托管、一键全网分发、AI 写作噱头 | — |

### 1.2 色彩 Token（须严格对齐）

| Token | 色值 | 用途 |
|-------|------|------|
| 应用背景 | `#F7F6F3` | 主背景、大面积留白 |
| 面板白 | `#FFFFFF` | 卡片、窗口、输入框底 |
| 次要底 | `#F0EDE6` | 侧栏 hover、分区底 |
| 预览画布 | `#FAF8F3` | 文案预览区、代码块底 |
| 主文字 | `#1B1C19` | 标题、正文 |
| 次级文字 | `#4E4638` | 说明、标签 |
| 辅助文字 | `#8A8278` | 时间戳、占位 |
| 主强调 | `#B8924A` | 主按钮、激活态、Logo 点缀 |
| 强调字色 | `#795916` | Section label、金色文字 |
| 分割线 | `#E4E2DD` / `#D1C5B3` | 边框、表格线 |

**禁止色倾向**：霓虹渐变、紫蓝 AI 风、冷灰蓝滤镜、彩虹促销、高饱和儿童粉彩。

### 1.3 图形语言

| 是 | 不是 |
|----|------|
| 圆角矩形面板（8–12px） | 大 Hero 插画墙 |
| 细线图标、列表行、看板列 | 3D 玻璃拟态、粒子动效 |
| 暖米纸质感、轻阴影 | 厚重金属、赛博朋克 |
| 金色小面积点缀 | 满版金色渐变 |
| 桌面窗口 / 浏览器插件条 | 手机 App 商城风 |

**推荐隐喻（Logo / 图标）**：发稿匣、内容托盘、清单勾选、文案块 + 渠道节点、日历格 + 勾选——**抽象几何，不画兔子、不画宠物**。

---

## 2. 参考图包与对话节奏

### 2.1 参考图代号

| 代号 | 内容 | 用途 |
|------|------|------|
| **A** | 已生成的 **Logo 主方案**（或 App 图标 1024×1024） | 锁定图形结构、金色比例、圆角风格 |
| **B** | **桌面端 UI 截图**（Today / Content / Tasks 任一真实界面） | 锁定面板布局、色彩、信息密度 |
| **C** | 已生成的 **营销成图**（Hero / 社媒首图） | 从第 2 张起锁定画风、光位、背景 |

### 2.2 推荐出图顺序

```
第 0 步  发送「角色设定」→ 不出图，确认品牌气质
第 1 步  生成 Logo / App 图标主方案 → 得到参考图 A
第 2 步  上传 A（+ 可选 B）→ 生成官网 Hero / OG 图
第 3 步  上传 A + B + 成图 1 → 生成社媒方图 / 竖版海报
第 4 步  上传 A + B → 生成 Product Hunt Gallery / 功能场景图
```

---

## 3. 第 0 步 · 角色设定（所有任务先发）

> 我要为 **PublishKit（发稿匣）** 制作品牌与营销视觉素材。PublishKit 是一款**本地优先的跨境内容运营工具**，管理「文案 × 素材 × 渠道 × 发布状态」，气质像 Linear / Notion / Eagle，**不是**营销官网、**不是** AI 炫光风、**不是**宠物电商。  
>   
> **品牌视觉（后续每一张都必须严格一致）**  
> - 主色：暖米应用底 **#F7F6F3**、面板白 **#FFFFFF**、主文字 **#1B1C19**  
> - 强调色：克制金色 **#B8924A**，仅用于按钮、激活态、Logo 点缀，禁止满版金渐变  
> - 气质：效率工具、列表导向、中等信息密度、安静可信、本地优先  
> - 图形：圆角面板、细线图标、轻阴影；**不出现宠物、兔子、缝纫元素**（那是姊妹品牌彭兔子）  
> - 默认：**无文字、无水印、无价格**（Logo 类除外）  
>   
> 请用一句话确认；下一消息我开始生成 Logo。

---

## 4. Logo 与 App 图标

### 4.1 画布尺寸

| 用途 | 尺寸 | 格式 |
|------|------|------|
| 主 Logo 源文件 | **1024 × 1024 px** | PNG 透明底 |
| macOS / Windows 应用图标 | 1024 → 导出 512/256/128/64/32/16 | PNG / ICO / ICNS |
| 网站 Favicon | 32 × 32、180 × 180 | PNG |
| 社交媒体头像 | 400 × 400 | PNG |

### 4.2 方案 A · 抽象发稿匣（推荐）

> 为 **PublishKit** 设计一款桌面软件品牌 Logo 与应用图标，画布 **1024×1024 像素**，正方形，**透明背景**，单图标输出，不要多方案拼图。  
>   
> **图形要求**  
> - 核心图形：抽象 **「发稿匣 / 内容托盘」**——圆角矩形容器（圆角约 12px 感），内部分为 **2–3 条水平内容行**（暗示文案列表），右侧或右下角有一颗 **金色勾选标记** **#B8924A**  
> - 线条：2px 级别细线描边，颜色 **#1B1C19**；填充为 **#FFFFFF** 或极浅 **#F7F6F3**  
> - 金色点缀：仅占图形面积约 **8–12%**，用于勾选、激活条或左侧 3px 竖条，**禁止**满版金色渐变  
> - 风格：现代 SaaS 工具图标，类似 Linear / Notion / Raycast 的克制几何感；**扁平微立体**，非 3D 玻璃、非拟物皮革  
> - 小尺寸可读：缩至 32×32 时，容器轮廓与勾选仍可辨认  
> - **禁止**：兔子、宠物、缝纫机、纸样、剪刀、彩虹渐变、霓虹 AI 风、复杂文字、水印、阴影过重  
>   
> 请只输出这一张 1024×1024 的透明底图标。

— 英文参考：PublishKit app icon 1024x1024px transparent, abstract dispatch tray with rounded rectangle container 12px radius, 2-3 horizontal content list rows, small gold checkmark accent #B8924A, ink lines #1B1C19, white fill, quiet SaaS tool aesthetic like Linear Notion, flat minimal geometric, readable at 32px, no rabbit no pet no sewing, no text no watermark

### 4.3 方案 B · 渠道节点图（备选）

> 为 **PublishKit** 设计应用图标，画布 **1024×1024**，透明底，单张输出。  
>   
> **图形要求**  
> - 中心：圆角方形节点 **#FFFFFF**，描边 **#D1C5B3**  
> - 四周：3–4 个小圆点或短横线通过细线连接中心（暗示「一条内容 → 多渠道发布」），连接线 **#8A8278**，激活节点 **#B8924A**  
> - 背景透明；整体像轻量 **workflow / 节点图** 图标，信息密度低  
> - 气质：本地工具、冷静、专业；非社交网络气泡、非炫光科技  
> - **禁止**：文字、Logo 字母 P、宠物元素、渐变底、3D 立体球  
>   
> 请只输出这一张 1024×1024 的图。

— 英文参考：minimal workflow hub icon, rounded square center node, 3-4 channel dots connected by thin lines, gold accent one node #B8924A, warm neutral palette, transparent background, local-first content ops tool, no text no pet no gradient

### 4.4 横版 Logo 锁标（官网 / README 用）

> 在已生成的 PublishKit 图标（参考图 A）基础上，生成 **横版品牌锁标**，画布 **2400×600 像素**（4:1），**透明背景**，单张输出。  
>   
> **画面要求**  
> - **左侧（约 22%）**：参考图 A 图标，占画面高度约 **70%**，垂直居中  
> - **右侧（约 78%）**：留空给后期叠字；仅保留极淡的水平对齐参考线，**本图不生成任何文字**  
> - 整体水平呼吸感强，右侧留白 ≥ 55%  
> - 色彩与图标方案严格一致  
> - **禁止**：在本图中生成 PublishKit 字母（避免 AI 乱码字体）、水印、装饰插画  
>   
> 请只输出这一张 2400×600 的透明底图。

— 英文参考：horizontal brand lockup layout 2400x600 transparent, app icon left 22% height 70%, empty right zone for text overlay later, no letters no wordmark generated, warm tool brand spacing

### 4.5 Logo 负向词（每条均可追加）

> 兔子、宠物、狗、猫、缝纫、纸样、剪刀、线轴、彩虹渐变、紫蓝 AI 炫光、霓虹、3D 玻璃拟态、金属质感、廉价促销、卡通吉祥物、复杂衬线字体、乱码文字、水印、价格标、低清、模糊、与参考图 A 不一致

---

## 5. 官网与落地页视觉

### 5.1 画布尺寸

| 用途 | 尺寸 | 比例 |
|------|------|------|
| Hero 主视觉（桌面） | **2880 × 1620 px** | 16:9 |
| Hero 主视觉（移动） | **1170 × 2532 px** | 约 9:19 |
| OG / 社交分享图 | **1200 × 630 px** | 1.91:1 |
| 功能截图展示框 | **2560 × 1600 px** | 16:10 |

### 5.2 Hero · 桌面端（产品窗口 + 克制留白）

> 严格依照参考图 A（PublishKit Logo）与参考图 B（桌面端 UI 截图），生成 **PublishKit 官网 Hero 营销视觉**，画布 **2880×1620 像素**，16:9 横构图，单张输出，**不要文字、不要水印**。  
>   
> **画面要求**  
> - **背景（全幅）**：暖米应用底 **#F7F6F3**，极轻纸张纹理，大面积留白；顶部与左侧各留 **12%** 安全区供后期叠标题与 CTA  
> - **主视觉（偏右 55%）**：一枚 **macOS 风格桌面应用窗口**（圆角 12px，轻阴影 `0 8px 28px rgba(27,28,25,.10)`），窗内 UI **依照参考图 B** 还原：左侧导航（今天/内容/素材/任务…）、右侧列表 + Inspector，色彩严格使用 **#FFFFFF** 面板、**#B8924A** 激活导航、**#1B1C19** 文字  
> - **点缀**：窗口右后方可有一层极淡的第二窗口（模糊 8px），暗示多任务，不抢主窗口  
> - 可选：画面左下角极小位置放置参考图 A 图标（高度约画面 6%），**不生成产品名字**  
> - 气质：安静、专业、本地工具；像 Linear 官网产品截图，**不是**插画 Hero、**不是** AI 渐变背景  
> - **禁止**：大标题文字、价格、促销标签、宠物元素、彩虹渐变、玻璃拟态过度、窗口内乱码中文、低清 UI  
>   
> 请只输出这一张 2880×1620 的图。

— 英文参考：PublishKit landing hero 2880x1620, warm app background #F7F6F3 generous whitespace, macOS desktop app window right side showing content ops UI from reference B, left nav accent gold #B8924A, subtle paper texture, quiet SaaS like Linear, no headline text no watermark no illustration hero no AI gradient

### 5.3 Hero · 移动端（竖版）

> 严格依照参考图 A 与参考图 B，生成 PublishKit **移动端落地页 Hero**，画布 **1170×2532 像素**，竖构图，单张输出，无文字。  
>   
> **画面要求**  
> - 背景 **#F7F6F3**，顶部 **18%** 留白供后期叠标题  
> - 中部：一枚 **略俯角的手机不适用**——改为 **窄版桌面窗口裁切** 或 **浏览器插件竖条面板**（宽 360px 感），展示「今天待发」任务列表与「复制正文 / 标记已发布」按钮，色彩对齐 Token  
> - 底部 **15%** 留白供 CTA  
> - 金色点缀仅用于按钮与激活行；整体克制  
> - **禁止**：文字、水印、满版 UI、宠物、促销风  
>   
> 请只输出这一张 1170×2532 的图。

— 英文参考：mobile landing hero 1170x2532 vertical, warm #F7F6F3, narrow browser extension panel or cropped desktop UI, today queue task list, gold accent buttons, top bottom safe margins, no text no watermark

### 5.4 OG 社交分享图

> 生成 PublishKit **Open Graph 分享图**，画布 **1200×630 像素**，横构图，单张输出。  
>   
> **画面要求**  
> - 背景：左侧 **62%** 暖米留白 **#F7F6F3**；右侧 **38%** 展示应用窗口一角（列表 + 金色激活导航），窗口裁切自然  
> - 左侧留白区仅放参考图 A 图标（高度约画面 28%），**不生成任何英文/中文标题**（避免 AI 字体乱码）  
> - 底部可有一条极细金色线 **#B8924A**（高度 3px）作品牌点缀  
> - 气质：专业工具、可信、安静；非促销海报  
> - **禁止**：长段文字、价格、水印、渐变底、宠物  
>   
> 请只输出这一张 1200×630 的图。

— 英文参考：OG image 1200x630, left whitespace with small app icon, right cropped PublishKit UI window, warm #F7F6F3, thin gold accent line, no generated text no watermark, quiet SaaS tool brand

---

## 6. 功能场景营销图（三款核心流程）

> 以下三张分别对应产品核心流程，可独立出图；均需参考图 B（真实 UI）保持一致。

### 6.1 场景 1 · 今天发什么（Today Queue）

| 项目 | 值 |
|------|-----|
| 画布 | **1920 × 1080 px**（16:9，官网/文档用） |
| 核心信息 | 「今天」视图：建议先发、已排期、最近已发布 |
| 情绪钩子 | 「打开就知道今天发什么」 |

> 严格依照参考图 B，生成 PublishKit 功能场景营销图，画布 **1920×1080**，16:9，单张输出，无文字。  
>   
> **画面要求**  
> - 主画面：桌面应用窗口占画布约 **78%**，居中略偏下，展示 **Today / 今天** 视图：分区标题、任务行（渠道 chip、语言 ZH/EN、状态 ready/scheduled）、行内「准备发布」金色按钮  
> - 背景 **#F7F6F3**，窗口外有柔和投影；窗口上方 **15%** 留白供后期叠一句标语  
> - UI 细节：行高舒适、状态色使用语义色（ready 绿 **#3A6A4A**、scheduled 金 **#B8924A**、published 蓝 **#2563EB**）  
> - **禁止**：假数据乱码、过度装饰、文字标题、水印、宠物、AI 风背景  
>   
> 请只输出这一张 1920×1080 的图。

— 英文参考：feature scene Today queue 1920x1080, desktop app window, content task list with channel chips and status pills, gold prepare button, warm neutral workspace, no headline text, reference B UI fidelity

### 6.2 场景 2 · 文案 × 素材同框（Content + Inspector）

| 项目 | 值 |
|------|-----|
| 画布 | **1920 × 1080 px** |
| 核心信息 | 左侧内容列表 + 右侧 Inspector（文案/素材 Tab） |
| 情绪钩子 | 「文案和配图不再分开找」 |

> 严格依照参考图 B，生成 PublishKit 功能场景图，画布 **1920×1080**，16:9，无文字。  
>   
> **画面要求**  
> - 窗口内 **左右分栏**：左 **60%** 内容库列表（标题、渠道、语言、状态列）；右 **40%** Inspector 展示文案预览区（**#FAF8F3** 画布底）+ 素材缩略图网格 3 列  
> - 选中行左侧 **3px 金色激活条** **#B8924A**；顶栏固定「复制标题 / 复制正文」按钮组  
> - 背景与阴影同 §6.1；上方留白 **12%**  
> - **禁止**：Markdown 正文生成乱码、素材图变成宠物照片、文字标语、水印  
>   
> 请只输出这一张 1920×1080 的图。

— 英文参考：Content library with inspector panel, list plus markdown preview and media thumbnail grid, gold selection bar, copy buttons, warm tool UI, no gibberish text overlay

### 6.3 场景 3 · 浏览器插件现场助手

| 项目 | 值 |
|------|-----|
| 画布 | **1920 × 1080 px** |
| 核心信息 | 浏览器右侧 360px 插件面板：连接状态、复制、标记已发布 |
| 情绪钩子 | 「发帖现场一键回填状态」 |

> 生成 PublishKit **浏览器插件**场景营销图，画布 **1920×1080**，16:9，无文字。  
>   
> **画面要求**  
> - **左侧（约 65%）**：模糊处理的浏览器内容区（中性灰 **#E4E2DD**，**不出现**真实平台 Logo），暗示正在发帖  
> - **右侧（约 35%）**：固定宽 **360px** 插件面板，白底 **#FFFFFF**，圆角左上和左下 12px，轻阴影；面板内：顶部绿色连接点 + 当前任务卡 + 全宽金色主按钮「标记已发布」+ URL 输入框  
> - 整体暖中性；插件与浏览器光位一致，无抠图感  
> - **禁止**：小红书/Instagram 等平台 Logo、真实帖子内容、乱码、水印、宠物  
>   
> 请只输出这一张 1920×1080 的图。

— 英文参考：browser extension panel 360px right docked, blurred neutral browser left, task card copy buttons mark published gold CTA, green connection dot, warm SaaS assistant, no platform logos no text watermark

---

## 7. 社媒营销配图

### 7.1 画布尺寸速查

| 平台 | 尺寸 | 比例 |
|------|------|------|
| 小红书 / 朋友圈竖图 | **1080 × 1920 px** | 9:16 |
| Instagram / 通用方图 | **1080 × 1080 px** | 1:1 |
| Twitter/X 横图 | **1600 × 900 px** | 16:9 |
| Product Hunt Gallery | **1270 × 760 px** | 5:3 |
| 微信公众号头图 | **900 × 383 px** | 2.35:1 |

### 7.2 竖版海报 · 构图 1（上 UI 下留白）

> 严格依照参考图 A 与参考图 B，生成 PublishKit 手机竖版营销海报，画布 **1080×1920 像素**，9:16，单张输出，无文字。  
>   
> **画面要求**  
> - **上半区（约 58%）**：应用窗口展示 Today 或 Tasks 看板，圆角 12px，背景 **#F7F6F3**；窗口内 UI 对齐参考图 B  
> - **下半区（约 42%）**：纯色暖米留白 **#F7F6F3**，仅左下角放小图标（参考图 A，高度约 8%），供后期叠中文标语  
> - 上下自然过渡，无硬分割线  
> - 顶部与底部各留 **8%** 安全边距  
> - **禁止**：促销风、彩虹色、AI 渐变、文字、水印、宠物、缝纫元素  
>   
> 请只输出这一张 1080×1920 的图。

— 英文参考：mobile poster 1080x1920, top app UI window bottom whitespace for copy overlay, warm #F7F3EE tool aesthetic, small icon corner, no text no watermark

### 7.3 竖版海报 · 构图 2（插件场景）

> 严格依照参考图 A，生成 PublishKit 竖版海报，画布 **1080×1920**，9:16，无文字。  
>   
> **画面要求**  
> - **主画面**：竖向浏览器 mock，右侧插件条全高展示；左侧模糊发帖区  
> - 背景 **#F7F6F3**；插件面板内金色主按钮为视觉焦点（面积约画面 5%）  
> - 顶部 **12%** 留白；底部 **20%** 留白供 slogan  
> - 气质：效率、现场感、跨境运营工具  
> - **禁止**：平台 Logo、乱码 UI、文字、水印  
>   
> 请只输出这一张 1080×1920 的图。

### 7.4 方图 · 三栏功能拼图（克制版）

> 严格依照参考图 B，生成 PublishKit 社媒方图，画布 **1080×1080 像素**，1:1，单张输出，无文字。  
>   
> **画面要求**  
> - 画布三等分竖列，列间距 **16px**，背景 **#F0EDE6**  
> - 每列一枚 **小窗口卡片**（圆角 8px）：左列「内容库」、中列「素材网格」、右列「日历/看板」——均为参考图 B 的裁切放大，**不重新设计 UI**  
> - 每列顶部可选 **6px 金色圆点** 作序列暗示（1·2·3）  
> - 四边留 **6%** 边距；整体像产品功能三连图，非促销拼图  
> - **禁止**：大标题、价格、箭头贴纸、宠物、渐变底  
>   
> 请只输出这一张 1080×1080 的图。

— 英文参考：square 1080x1080 three column feature cards, content media calendar UI crops, warm neutral gaps 16px, minimal gold dots, no text no promo stickers

### 7.5 Product Hunt Gallery 主图

> 严格依照参考图 A 与参考图 B，生成 **Product Hunt 产品 Gallery 主图**，画布 **1270×760 像素**，横构图，单张输出。  
>   
> **画面要求**  
> - 背景 **#F7F6F3** 占 **100%**，无渐变  
> - **居中偏右**：桌面窗口展示 Content + Inspector，占画布宽约 **72%**，投影轻柔  
> - **左侧 28%**：大面积留白，仅放参考图 A 图标（高度约画面 22%），**不写产品名**（PH 页面自有标题）  
> - 气质：Indie SaaS、专业、安静；对齐 Product Hunt 上 Notion / Linear 类工具截图风  
> - **禁止**：「Product Hunt」字样、排名、箭头、宠物、AI 炫光、乱码 UI、水印  
>   
> 请只输出这一张 1270×760 的图。

— 英文参考：Product Hunt gallery 1270x760, warm whitespace left with app icon, right desktop UI screenshot content inspector, indie SaaS tool launch image, no Product Hunt text no ranking badges no watermark

---

## 8. 中英文后期叠加文案（生成后 PS / Figma 叠字）

> 文字一律后期叠加，避免 AI 乱码。字体建议：英文 **Inter SemiBold**；中文 **PingFang SC / 微软雅黑 Medium**。

### 8.1 官网 Hero

| 语言 | 主标题 | 副标题 | CTA |
|------|--------|--------|-----|
| 中文 | 发稿匣 · 本地内容运营工作台 | 文案、素材、渠道、发布状态 — 一套管清 | 免费下载 Windows 版 |
| 英文 | PublishKit — Local content ops workspace | Copy, media, channels, publish status — finally in one place | Download for Windows |

### 8.2 社媒竖版海报

| 语言 | 顶部 12% | 中部（慎用） | 底部 18% |
|------|----------|--------------|----------|
| 中文 | PublishKit 发稿匣 | 今天发什么 · 一眼看清 | 选任务 → 复制 → 发布 → 回填 |
| 英文 | PublishKit | Know what to publish today | Pick → Copy → Post → Track |

### 8.3 功能场景图

| 场景 | 中文标语 | 英文标语 |
|------|----------|----------|
| Today | 打开就知道今天发什么 | Open app. See today's queue. |
| Content + 素材 | 文案和配图，不再分开找 | Copy and media, linked in one view. |
| 插件 | 发帖现场，一键记清楚 | Mark published without leaving the browser. |

**叠字色**：标题 **#1B1C19**；强调词 **#B8924A**；正文 **#4E4638**。

---

## 9. 通用负向词（每一张均可追加）

> 兔子、宠物、狗、猫、缝纫机、纸样、剪刀、布料、彩虹渐变、紫蓝 AI 炫光、霓虹、赛博朋克、3D 玻璃拟态、廉价促销风、大红大黄价格标、儿童插画、卡通吉祥物、满版金色渐变、冷灰蓝滤镜、营销官网大 Hero 插画、乱码中英文、水印、低清、模糊、分辨率错误、与参考图 A/B 不一致、窗口内 UI 风格漂移

---

## 10. 素材清单与自查

### 10.1 最低素材包（首发必备）

| # | 素材 | 尺寸 | 状态 |
|---|------|------|------|
| 1 | App 图标主方案 | 1024×1024 | ☐ |
| 2 | 横版 Logo 锁标（无字） | 2400×600 | ☐ |
| 3 | 官网 Hero 桌面 | 2880×1620 | ☐ |
| 4 | OG 分享图 | 1200×630 | ☐ |
| 5 | 功能场景 ×3 | 1920×1080 | ☐ |
| 6 | Product Hunt Gallery | 1270×760 | ☐ |
| 7 | 社媒方图 | 1080×1080 | ☐ |
| 8 | 社媒竖版 ×2 构图 | 1080×1920 | ☐ |

### 10.2 自查清单

- [ ] 色彩对齐 Token（**#F7F6F3** / **#B8924A** / **#1B1C19**），无 AI 渐变风
- [ ] 气质是**效率工具**，不是营销站、不是宠物电商
- [ ] 默认画面**无文字**；文案已规划后期叠加
- [ ] UI 场景与真实产品界面（或 Stitch 原型）一致，无乱码
- [ ] 图标缩至 32×32 仍可辨认
- [ ] 无姊妹品牌（彭兔子）的宠物/缝纫视觉元素
- [ ] 导出体积：Hero ≤ 800KB；图标 PNG 可用 TinyPNG 压一遍

### 10.3 导出命名建议

```text
publishkit_icon_1024.png
publishkit_logo_lockup_2400x600.png
publishkit_hero_desktop_2880x1620.png
publishkit_og_1200x630.png
publishkit_feature_today_1920x1080.png
publishkit_feature_content_inspector_1920x1080.png
publishkit_feature_extension_1920x1080.png
publishkit_ph_gallery_1270x760.png
publishkit_social_square_1080.png
publishkit_social_poster_01_1080x1920.png
publishkit_social_poster_02_1080x1920.png
```

---

## 11. 快速复制：图标完整示例（方案 A）

> 为 **PublishKit** 设计一款桌面软件品牌 Logo 与应用图标，画布 **1024×1024 像素**，正方形，**透明背景**，单图标输出，不要多方案拼图。  
>   
> **图形要求**  
> - 核心图形：抽象 **「发稿匣 / 内容托盘」**——圆角矩形容器（圆角约 12px 感），内部分为 **2–3 条水平内容行**（暗示文案列表），右侧有一颗 **金色勾选标记** **#B8924A**  
> - 线条：2px 级别细线描边，颜色 **#1B1C19**；填充为 **#FFFFFF** 或极浅 **#F7F6F3**  
> - 金色点缀：仅占图形面积约 **8–12%**  
> - 风格：现代 SaaS 工具图标，克制几何感，类似 Linear / Notion  
> - 小尺寸可读：缩至 32×32 时轮廓仍清晰  
> - **禁止**：兔子、宠物、缝纫、彩虹渐变、霓虹 AI 风、文字、水印  
>   
> 请只输出这一张 1024×1024 的透明底图标。

---

*文档版本 v1.0 · 对齐 [docs/设计标准-PublishKit.md](./docs/设计标准-PublishKit.md) · 参考彭兔子运营资产 `图片/营销海报AI描述词.md` 描述结构*
