# PublishKit / 发稿匣 — 设计标准

| 字段 | 内容 |
|------|------|
| 文档版本 | v1.2 |
| 编写日期 | 2026-08-24 |
| 状态 | 待评审 |
| 关联文档 | [PRD](../PRD-营销内容资产与发布状态管理工具.md)、[技术标准](./技术标准-PublishKit.md) |
| 设计工具 | Google Stitch（`stitch.withgoogle.com`）用于高保真原型；开发实现以本文为准 |

---

## 1. 设计定位

### 1.1 产品气质

PublishKit 是**效率型内容运营工具**，不是营销官网，也不是数据大屏。

| 是 | 不是 |
|----|------|
| 列表、表格、看板、日历 | 大 Hero、插画墙、渐变 AI 风 |
| 中等信息密度、快速扫读 | 卡片瀑布流、过度留白 |
| 本地优先、可信、安静 | 炫酷动效、强品牌表演 |
| 操作路径短：复制 → 发布 → 回填 | 多层向导、功能藏深 |

**参考气质（非抄袭）：** Linear、Notion、Eagle、Billfish、Obsidian 的「工具感」。

**Sibling 品牌关系：** 与彭兔子制版（Warm Paper Studio）同属创作者工具生态，可共享暖色与金色点缀，但 PublishKit **更克制、更列表导向**，避免营销站的大标题衬线排版。

### 1.2 设计原则

1. **任务优先**：默认视图回答「今天发什么」，不是「资产有多少」。
2. **状态可见**：发布状态、渠道、语言在列表行内一眼可见，不藏进详情。
3. **高频操作外露**：复制正文、复制标题、打开文件夹、标记已发布——不进二级菜单。
4. **详情不跳页**：桌面端主路径采用「列表 + 右侧 Inspector」，减少上下文丢失。
5. **系统一致**：跟随 OS 深浅色；不强制自带炫目主题。
6. **零语言闪烁**：UI 语言在首帧渲染前就绪（见技术标准 i18n 章节）；设计稿按 locale 分别出稿，禁止「中文稿 + 标注换成英文」的单稿糊弄。

---

## 2. 设计 Token

### 2.1 色彩

#### 浅色模式（默认）

| Token | 值 | 用途 |
|-------|-----|------|
| `--pk-bg-app` | `#F7F6F3` | 应用背景 |
| `--pk-bg-panel` | `#FFFFFF` | 面板、卡片、输入框底 |
| `--pk-bg-alt` | `#F0EDE6` | 次要区域、侧栏 hover |
| `--pk-bg-canvas` | `#FAF8F3` | 预览区、代码/文案预览底 |
| `--pk-ink` | `#1B1C19` | 主文字 |
| `--pk-ink-secondary` | `#4E4638` | 次级文字 |
| `--pk-ink-muted` | `#8A8278` | 辅助、占位、时间戳 |
| `--pk-border` | `#E4E2DD` | 分割线、表格线 |
| `--pk-border-strong` | `#D1C5B3` | 输入框、面板描边 |
| `--pk-accent` | `#B8924A` | 主按钮、激活导航、关键强调 |
| `--pk-accent-text` | `#795916` | 标签、Section label |
| `--pk-accent-soft` | `rgba(184,146,74,0.12)` | 选中行、激活 chip 底 |
| `--pk-inverse` | `#FDFCFA` | 深色按钮上的字 |

#### 深色模式

| Token | 值 | 用途 |
|-------|-----|------|
| `--pk-bg-app` | `#1A1917` | 应用背景 |
| `--pk-bg-panel` | `#242320` | 面板 |
| `--pk-bg-alt` | `#2C2A26` | 侧栏、hover |
| `--pk-ink` | `#F5F3EE` | 主文字 |
| `--pk-ink-secondary` | `#C9C2B8` | 次级 |
| `--pk-ink-muted` | `#8A8278` | 辅助 |
| `--pk-border` | `#3A3834` | 分割 |
| `--pk-accent` | `#C9A85A` | 主强调（略提亮） |
| `--pk-accent-soft` | `rgba(201,168,90,0.16)` | 选中底 |

#### 语义色（深浅模式共用，深色下略降饱和）

| 语义 | 色值 | 用途 |
|------|------|------|
| `--pk-status-draft` | `#8A8278` | 草稿 |
| `--pk-status-ready` | `#3A6A4A` | 就绪 |
| `--pk-status-scheduled` | `#B8924A` | 已排期 |
| `--pk-status-published` | `#2563EB` | 已发布 |
| `--pk-status-blocked` | `#B42318` | 阻塞 |
| `--pk-status-archived` | `#6B7280` | 已归档 |

#### 渠道色（日历 / 看板卡片左边线，可选）

| 渠道 | 色值 |
|------|------|
| 小红书 | `#FF2442` |
| 抖音 | `#000000` / 深色模式下 `#FFFFFF` 描边 |
| Instagram | `#E1306C` |
| Pinterest | `#E60023` |
| Reddit | `#FF4500` |
| Product Hunt | `#DA552F` |
| 其他 | `#8A8278` 默认 |

### 2.2 字体

| 用途 | 中文 | 英文 | 备注 |
|------|------|------|------|
| UI 正文 | PingFang SC, Microsoft YaHei | Inter, Source Sans 3 | **不用** 营销站 Noto Serif / Playfair 作主 UI 字体 |
| 等宽（路径、ID） | Consolas, monospace | Consolas, monospace | 文件路径、配对码 |
| 数字/表格 | 与 UI 正文相同 | tabular-nums | 日期、计数对齐 |

| 层级 | 字号 | 字重 | 行高 |
|------|------|------|------|
| Page title | 20px | 600 | 1.3 |
| Section title | 16px | 600 | 1.35 |
| Body | 14px | 400 | 1.5 |
| Meta / label | 12px | 500 | 1.4 |
| Table cell | 13px | 400 | 1.45 |
| Button | 13–14px | 500–600 | 1 |

### 2.3 间距与圆角

| Token | 值 |
|-------|-----|
| `--pk-space-1` | 4px |
| `--pk-space-2` | 8px |
| `--pk-space-3` | 12px |
| `--pk-space-4` | 16px |
| `--pk-space-5` | 24px |
| `--pk-space-6` | 32px |
| `--pk-radius-sm` | 6px |
| `--pk-radius-md` | 8px |
| `--pk-radius-lg` | 12px |
| `--pk-radius-pill` | 999px |

**布局常量：**

| 区域 | 尺寸 |
|------|------|
| 窗口默认 | 1280 × 800（最小 1024 × 640） |
| 侧栏（展开） | 220px |
| 侧栏（图标模式，P2） | 72px |
| Inspector 右栏 | 360px（可拖拽 320–480） |
| 插件面板 | 360 × 640（max-height: 90vh） |

### 2.4 阴影与层级

| Token | 值 | 用途 |
|-------|-----|------|
| `--pk-shadow-sm` | `0 1px 3px rgba(27,28,25,.06)` | 输入框、chip |
| `--pk-shadow-md` | `0 4px 12px rgba(27,28,25,.08)` | 下拉、浮层 |
| `--pk-shadow-lg` | `0 8px 28px rgba(27,28,25,.10)` | Modal |

层级：`base(0)` → `sticky-header(10)` → `dropdown(100)` → `modal(200)` → `toast(300)`

---

## 3. 组件规范

### 3.1 按钮

| 类型 | 样式 | 用途 |
|------|------|------|
| Primary | 填充 `--pk-accent`，字 `--pk-inverse` | 准备发布、标记已发布、保存 |
| Secondary | 白底 + `--pk-border-strong` 边框 | 导出、取消 |
| Ghost | 无边框，hover 底 `--pk-bg-alt` | 复制、次要操作 |
| Danger | 字/边 `--pk-status-blocked` | 删除链接、归档 |

- 高度：32px（紧凑）/ 36px（默认）
- 圆角：`--pk-radius-md`
- 图标按钮：32×32，必须带 tooltip 与 aria-label
- **同一视觉区域 Primary 最多 1 个**

### 3.2 输入与搜索

- 搜索框：左侧放大镜图标；占位符用 i18n；支持 `Ctrl/Cmd+K` 聚焦
- 路径输入：等宽字体 + 「浏览…」按钮
- 筛选：chip 可多选；激活态 `--pk-accent-soft` 底 + accent 边

### 3.3 标签 Chip

| 类型 | 样式 |
|------|------|
| 语言 ZH / EN | 小 pill，outline |
| 渠道 | 可选渠道色左边点 |
| 状态 | 实心 pill + 6px 圆点 |
| 标签 | 灰 outline，可关闭（编辑态） |

### 3.4 表格 / 列表

- 表头 sticky；行高 44–48px
- Hover：`--pk-bg-alt`
- 选中：`--pk-accent-soft` 底 + 左侧 3px accent 条
- 列建议：标题（flex）| 渠道 | 语言 | 状态 | 更新时间 | 操作（图标）
- 空状态：一行说明 + 一个 CTA，**不用** 大插画

### 3.5 看板

- 列宽最小 280px；列头：状态名 + 计数
- 卡片：标题 1 行 ellipsis；次行渠道 + 语言；blocked 显示红色原因一行
- 拖拽：卡片左侧 subtle handle；拖拽中阴影 `--pk-shadow-md`

### 3.6 日历

- 月视图为主；周视图为 P1
- 事件 pill 高度 18–20px；按渠道色
- 点击日期 → 右侧 drawer 列出当日任务

### 3.7 条目 Inspector（右栏）

Tab 顺序：**文案 | 素材 | 任务 | 备注**

- **文案**：分段预览；顶栏固定 `[复制标题] [复制正文] [复制标签]`
- **素材**：缩略图网格 3 列；每项 `[显示] [复制路径]`
- **任务**：按渠道一行，inline 状态切换
- **备注**：合规提示、UTM 备注

### 3.8 浏览器插件

- 结构：Header（连接状态）→ 当前任务卡 → 复制按钮组 → 分隔线 → URL 回填区 → Footer
- 宽度固定 360px；按钮全宽 stacked
- 连接成功：绿色 dot + 文案；失败：红色 + 「打开桌面端」链接
- Toast：底部 compact，2s 自动消失

### 3.9 反馈

| 类型 | 规范 |
|------|------|
| Toast | 成功/失败/信息；不阻断 |
| Inline error | 输入框下 12px 红字 |
| Confirm | 破坏性操作 Modal；标题 + 一句后果 |
| Loading | 列表 skeleton 或顶部细 progress；不全屏 spinner |
| Empty | 见 3.4 |

---

## 4. 信息架构与页面规范

### 4.1 桌面端导航

| 顺序 | 导航项 | 路由 | 核心问题 |
|------|--------|------|----------|
| 1 | 今天 | `/today` | 现在发什么 |
| 2 | 内容 | `/content` | 有哪些条目 |
| 3 | 素材 | `/media` | 有哪些图/视频 |
| 4 | 任务 | `/tasks` | 各渠道进度 |
| 5 | 日历 | `/calendar` | 何时发/发过 |
| 6 | 来源文件 | `/sources` | 从哪拆条目 |
| 7 | 设置 | `/settings` | 目录、语言、插件、许可 |

顶栏全局：项目切换 | 搜索 | 插件连接状态 | 窗口控制

### 4.2 关键页面线框要求

#### 今天 `/today`

- Section 1：**建议先发**（ready + 逾期 scheduled）
- Section 2：**已排期**
- Section 3：**最近已发布**（7 天内，可折叠）
- 行内主操作：`[准备发布]`

#### 内容 `/content`

- 左 60% 列表 + 右 40% Inspector（选中时）
- 筛选：市场、语言、类型、标签、来源文件
- 无选中时 Inspector 显示快捷统计或引导

#### 素材 `/media`

- 文件夹面包屑 + 网格/列表切换
- 选中素材：底部或侧 drawer 显示「被 N 条内容引用」

#### 任务 `/tasks`

- 默认看板；可切换列表
- 筛选：项目、渠道、状态、负责人（Team）

#### 设置 `/settings`

- 分组：工作区目录 | UI 语言 | 插件配对 | 备份 | 许可证 | 关于
- **不放** 复杂营销式定价墙；升级入口链到官网

### 4.3 官网落地页（与 App 区分）

- 更宽留白；可保留一句 serif 标题作品牌
- 区块：Hero → 三步流程 → 截图 → 定价 → FAQ → Footer
- **静态分语言**：`/zh/` 与 `/en/`，禁止整页 JS 翻译
- 视觉 Token 与 App 一致，但密度更低

---

## 5. 交互规范

### 5.1 核心用户流程（设计必须串联）

```text
选任务 → 准备发布（复制/导出）→ 浏览器粘贴发布 → 插件标记已发布 → 台账更新
```

| 步骤 | 用户动作 | 界面反馈 |
|------|----------|----------|
| 1 | 在「今天」或「任务」选中 ready 任务 | Inspector 展示文案+素材 |
| 2 | 点「准备发布」 | Toast「正文已复制」；可选导出文件夹 |
| 3 | 切换到浏览器平台发帖 | — |
| 4 | 插件点「标记已发布」 | URL 自动填入；确认后状态变 published |
| 5 | 回桌面端（可选） | 日历/列表即时反映 |

### 5.2 快捷键（MVP）

| 快捷键 | 动作 |
|--------|------|
| `Ctrl/Cmd + K` | 全局搜索 |
| `Ctrl/Cmd + Shift + C` | 复制正文（Inspector 聚焦时） |
| `Ctrl/Cmd + Enter` | 标记当前任务 ready → 打开准备发布 |
| `Esc` | 关闭 Modal / 清空搜索 |

### 5.3 状态变更

- 列表/看板内可改状态；`published` 需 URL（插件回填或手填）
- `blocked` 必须显示原因（缺图 / 合规 / 待审核）
- 重复发布（P1 / F12）：Modal 警告「该 content×channel 30 天内已发布」

### 5.4 动效

| 场景 | 时长 | 曲线 |
|------|------|------|
| Hover | 120ms | ease |
| Panel 展开 | 180ms | ease-out |
| Toast | 200ms in / 150ms out | ease |
| 禁止 | 全屏过渡、弹跳、粒子 | — |

### 5.5 无障碍（基础）

- 对比度：正文 ≥ 4.5:1
- 焦点环：2px accent outline，不禁用
- 图标按钮必须有 accessible name
- 状态不只靠颜色：带文字或 icon

### 5.6 筛选与状态同步（防回归）

> 彭兔子 BUG-2026-08-03-01：下拉显示「国内」但 URL/数据仍是海外。

- 渠道、市场、语言、任务状态等筛选：**单一 store 为真源**
- 改筛选 → 同步 URL query（可选）→ 列表请求用同一组参数
- 刷新/深链进入 → 从 query 或持久化 settings 恢复筛选，**不得 UI 与数据不一致**
- 禁止「显示标签」与「实际 filter 参数」两套 state

### 5.7 控件约束

- 避免未样式化的原生 `<select>`（彭兔子 BUG-2026-08-24-02）；用 styled combobox 或 chip 组
- 浏览器/Electron 环境：**禁止**在 frontend 使用 Node `global`（彭兔子 BUG-2026-08-05-02）；统一 `window`

---

## 6. 内容语言 vs UI 语言（设计侧）

| 维度 | UI 语言 | 内容语言 |
|------|---------|----------|
| 控制 | 设置 → UI Language | 条目字段 `zh` / `en` chip |
| 展示 | 导航、按钮、空状态 | 文案预览区原文，不翻译 |
| 筛选 | 设置切换后立即生效 | 筛选项「内容语言」 |
| 设计稿 | 分别出 zh-CN / en 两版 App 屏 | 预览区可展示中文 MD 原文 |

**设计验收：** en UI 界面中不得出现未 i18n 的中文（内容预览区除外）。

---

## 7. Stitch 原型工作流

### 7.1 推荐顺序

1. 导入本文档 §2 Token 作为 DESIGN.md
2. 生成 Screen：Today → Content+Inspector → Tasks Kanban → Extension → Settings → Landing
3. Stitch Play 串联：Today → Extension 标记已发布
4. Annotate 迭代密度与按钮层级
5. 导出截图进 PRD / 开发任务

### 7.2 屏幕清单

| ID | 名称 | 尺寸 |
|----|------|------|
| S1 | Today Queue | 1280×800 Desktop |
| S2 | Content Library + Inspector | 1280×800 Desktop |
| S3 | Media Library | 1280×800 Desktop |
| S4 | Tasks Kanban | 1280×800 Desktop |
| S5 | Publish Calendar | 1280×800 Desktop |
| S6 | Split Markdown Wizard | 1280×800 Desktop |
| S7 | Settings Workspace | 1280×800 Desktop |
| S8 | Browser Extension Panel | 360×640 |
| S9 | Landing Page | 1440×900 Desktop |
| S10 | Landing Page Mobile | 390×844 |

### 7.3 样例数据（所有稿统一）

- 项目：PongRabbit Studio
- 条目：「01 三角口水巾 · 小红书成稿」「How to measure a dog · Instagram」
- 渠道：小红书、Instagram、Pinterest、Product Hunt
- 状态：ready、scheduled、published、blocked

---

## 8. 设计交付物

| 交付物 | 格式 | 负责人 |
|--------|------|--------|
| Stitch 高保真原型 | Stitch 项目链接 | 设计 |
| Token CSS 变量 | `tokens.css` | 设计 → 开发 |
| 组件 Storybook / 样例页 | 与实现同步 | 开发 |
| 中英 UI 文案表 | `locales/*.json` | 产品 + 开发 |
| 图标集 | Lucide 或 Phosphor，16/20/24 | 设计 |

**DoD：** 开发实现与 Token 偏差需有记录；新增组件先补本文档再写代码。

---

## 9. 修订记录

| 版本 | 日期 | 说明 |
|------|------|------|
| v1.0 | 2026-08-24 | 首版：Token、组件、IA、交互、Stitch 流程、i18n 设计约束 |
| v1.1 | 2026-08-24 | 自检：导航命名与 PRD 对齐；重复发布标注 P1 |
| v1.2 | 2026-08-24 | 对照彭兔子 Bug：§5.6 筛选同步、§5.7 原生控件/global |
