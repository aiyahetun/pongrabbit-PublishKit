# PublishKit / 发稿匣 · Bug 记录汇总

| 项 | 内容 |
|----|------|
| 文档版本 | 1.5 |
| 创建日期 | 2026-08-24 |
| 最后整理 | 2026-08-29（CI Linux 依赖缺失 · BUG-2026-08-29-01） |
| 产品 | PublishKit（发稿匣）— 营销内容资产与发布状态管理工具 |
| 维护约定 | 每次可复现缺陷修复后，在「详细记录」追加一节，并更新「索引」表状态 |
| 检索入口 | 本文为主索引；防回归规则见 [防回归清单-彭兔子Bug教训.md](../防回归清单-彭兔子Bug教训.md) |
| 关联文档 | [PRD](../../PRD-营销内容资产与发布状态管理工具.md) · [技术标准](../技术标准-PublishKit.md) · [设计标准](../设计标准-PublishKit.md) · [产品进度一览](./产品进度一览.md) |

---

## 一、索引（按领域）

> 编号规则：`BUG-YYYY-MM-DD-NN`（同日递增）。文档/流程类用 `DOC-` 前缀，不计入线上缺陷统计。

### 1.1 线上 / 产品缺陷

| 编号 | 领域 | 简述 | 状态 | 首次记录 |
|------|------|------|------|----------|
| BUG-2026-08-24-01 | 桌面端 | 启动崩溃：`duplicate column name: is_custom`（迁移 003 重复执行） | 已修复 · 生产验收通过 | 2026-08-24 |
| BUG-2026-08-24-02 | 桌面端 | 内容页右上角「新建内容」按钮空白（scoped `button {}` 覆盖主按钮样式） | 已修复 · 生产验收通过 | 2026-08-24 |
| BUG-2026-08-24-03 | 任务/台账 | 任务页 / 今天页缺少帖子链接输入区 | 已修复 · 生产验收通过 | 2026-08-24 |
| BUG-2026-08-24-04 | 桌面端 | 来源文件 / 拆分向导 scoped `button {}` 主按钮空白 | 已修复 · 生产验收通过 | 2026-08-24 |
| BUG-2026-08-24-05 | 桌面端 | 素材页「刷新列表」卡死（列表加载时同步为全部图片生成缩略图） | 已修复 · 生产验收通过 | 2026-08-24 |
| BUG-2026-08-26-01 | 内容/扫描 | Word (.docx) 标题样式未识别，拆分无 `#` 段落 | 已修复 · 生产验收通过 | 2026-08-26 |
| BUG-2026-08-26-02 | 插件/任务 | 桌面端已发布后插件再点「标记已发布」无 confirm（重复检测排除当前 task id） | 已修复 · 生产验收通过 | 2026-08-26 |
| BUG-2026-08-29-01 | CI | GitHub Actions `cargo check` 在 ubuntu-latest 失败（缺 glib-2.0 等 Tauri Linux 依赖） | 已修复 · 待验收 | 2026-08-29 |

### 1.2 文档 / 规划类（已关闭）

| 编号 | 领域 | 简述 | 状态 | 首次记录 |
|------|------|------|------|----------|
| DOC-2026-08-24-01 | 文档/PRD | 里程碑 M1 含 F7 插件与 M2 重复；Pro 定价含「日历」与 P0 F6 冲突 | 已修复 · PRD v2.1 | 2026-08-24 |
| DOC-2026-08-24-02 | 文档/PRD | 验收 AC5（防重复）在 P1 F12，却列入 MVP 验收 | 已修复 · PRD v2.1 分层 | 2026-08-24 |
| DOC-2026-08-24-03 | 文档/PRD | 开放问题「Vue vs React」与 §4.2 已冻结 Vue 矛盾 | 已修复 · PRD v2.2 | 2026-08-24 |
| DOC-2026-08-24-04 | 文档/技术 | i18n 缺 pickLang 禁令、locale key diff CI、首次安装 DB 前引导 | 已修复 · 技术 v1.2 | 2026-08-24 |
| DOC-2026-08-24-05 | 文档/设计 | 缺筛选 UI 与数据同步规范；章节 5.4/5.5 顺序错乱 | 已修复 · 设计 v1.2 | 2026-08-24 |
| DOC-2026-08-24-06 | 文档/PRD | 对照彭兔子 Bug 记录，防回归条款不足 | 已修复 · 新增防回归清单 | 2026-08-24 |

### 1.3 待办 / 已知缺口（非 Bug，跟踪用）

| 编号 | 类型 | 简述 | 优先级 | 计划 |
|------|------|------|--------|------|
| GAP-01 | 数据模型 | `bilingual` 条目：拆两条 vs 单条双字段，未写死 | P1 | M0 台账验证时拍板 |
| GAP-02 | 设计 | 首次运行向导 S0 线框未出 | P1 | M1 前补 Stitch / 设计标准 |
| GAP-03 | 设计 | 插件配对屏未单独线框 | P2 | M2 前并入 Settings |
| GAP-04 | 商业 | 激活码离线校验策略未定 | P2 | M3 前 |
| GAP-05 | 资产 | Stitch 逐屏长 prompt 未独立成文件 | P3 | 按需 |

---

## 二、状态说明

| 状态 | 含义 |
|------|------|
| 未修 | 已确认，未开始修复 |
| 修复中 | 已有 PR / 分支，未验收 |
| 已修复 · 待验收 | 代码已合，待自测或用户确认 |
| 已修复 · 生产验收通过 | 对应版本已发布且验证通过 |
| 已缓解 | 有 workaround 或文档约束，未彻底根治 |
| 不修 / 设计如此 | 经评审确认非缺陷 |
| 已修复 · 文档 | 仅文档类条目使用 |

---

## 三、领域标签（索引用）

| 标签 | 范围 |
|------|------|
| 桌面端 | Tauri 主程序、SQLite、扫描、UI |
| 插件 | Chrome/Edge MV3、localhost API、回填 |
| i18n | UI 语言、FOUC、locale key |
| 内容/扫描 | MD 拆分、素材关联、路径索引 |
| 任务/台账 | 发布任务、看板、日历、状态机 |
| 官网 | 落地页、下载、支付外链 |
| 安装/更新 | Windows/macOS 包、签名、自动更新 |
| 文档 | PRD、标准、进度类 |

---

## 四、详细记录模板（复制使用）

```markdown
### BUG-YYYY-MM-DD-NN · 标题

| 项 | 内容 |
|----|------|
| 影响范围 | 例：Windows 桌面 · 内容库 · EN UI |
| 现象 | 用户可见现象，附截图/录屏路径（如有） |
| 复现步骤 | 1. … 2. … 3. … |
| 期望 | 应该怎样 |
| 根因 | 技术原因（确认后填写） |
| 解决方案 | 改动摘要 + 关联文件 |
| 防回归 | 对应 I18N-R* / PATH-R* / 单测 / CI |
| 关联代码 | `path/to/file` |
| 版本 | 修复所在版本号 |
| 状态 | 已修复 · 待验收 / 生产验收通过 |
```

---

## 五、详细记录

> 开发启动后，按时间倒序追加。文档类历史见索引 §1.2。

### BUG-2026-08-29-01 · CI 在 Linux 上 cargo check 失败

| 项 | 内容 |
|----|------|
| 影响范围 | GitHub Actions · CI / check · `ubuntu-latest` |
| 现象 | 每次 push 到 `main` 后 CI 邮件报错；`Check Rust backend` 步骤 exit 101；自仓库启用 CI 起 7 次运行全部失败 |
| 复现步骤 | 1. push 任意提交到 `main` 2. 查看 Actions `CI / check` 3. `cargo check` 报 `Package 'glib-2.0' … not found` |
| 期望 | CI 与本地一致通过：`npm ci`、i18n、前端 build、`cargo check` 均绿 |
| 根因 | `ci.yml` 在 Linux runner 执行 `cargo check`，未安装 Tauri 2 所需的 GTK/WebKit 系统库；Windows 本地无此依赖故可通过 |
| 解决方案 | `ci.yml` 在 `cargo check` 前 `apt-get install` `libwebkit2gtk-4.1-dev` 等 Tauri Linux 依赖 |
| 防回归 | CI 工作流须保留 Linux 依赖安装步骤；或改为 `windows-latest` 执行 `cargo check` |
| 关联代码 | `.github/workflows/ci.yml` |
| 版本 | —（CI 基础设施） |
| 状态 | **已修复 · 待验收** |

### BUG-2026-08-26-02 · 插件重复发布 confirm 未弹出

| 项 | 内容 |
|----|------|
| 影响范围 | 浏览器插件 · 桌面端 API · 任务「标记已发布」 |
| 现象 | 桌面端已标记发布后，插件仍可直接再次「标记已发布」，无浏览器 `confirm` |
| 复现步骤 | 1. 桌面端对某任务标记已发布 2. 插件加载待发列表（或保留旧列表）3. 点「标记已发布」 |
| 期望 | 30 天内同 content×channel 再次发布应弹出重复提醒 |
| 根因 | `publish_tasks` 对 `(content_item_id, channel_id)` 唯一；`duplicate_publish_warning` 用 `id != 当前任务` 排除自身，导致查不到唯一那条已发布记录 |
| 解决方案 | 按 `published_at` 查最近发布记录，不再排除当前 task id；撤销发布时保留 `published_at` 以便撤销后再发仍预警 |
| 防回归 | `cargo test duplicate_publish`（`db.rs` 单测） |
| 关联代码 | `apps/desktop/src-tauri/src/db.rs`、`apps/extension/popup.js` |
| 版本 | v0.2.2 |
| 状态 | **已修复 · 生产验收通过** |

### BUG-2026-08-26-01 · Word docx 标题样式未识别

| 项 | 内容 |
|----|------|
| 影响范围 | Windows 桌面 · 来源文件 · Word / 飞书导出 `.docx` 拆分 |
| 现象 | 导入 `.docx` 后拆分预览无 `#` 标题段落，整篇像纯文本；H1/H2 策略失效 |
| 复现步骤 | 1. 准备含「标题1/Heading1」样式的 Word 文档 2. 来源扫描 → 拆分 3. 预览块无标题层级 |
| 期望 | `Heading1–4` / `标题1–4` 映射为 Markdown `#` … `####`，可走智能/H1 拆分 |
| 根因 | OOXML 中 `w:pStyle` 的 `val` 带 Word 命名空间；`node.attribute("val")` 读不到样式名 |
| 解决方案 | `doc_import.rs` 改用 `node.attribute((W_NS, "val"))`；补单元测试 `reads_heading_style_with_word_namespace` |
| 防回归 | `cargo test --test import_self_check`；docx 样例见 `samples/import/sample.docx` |
| 关联代码 | `apps/desktop/src-tauri/src/doc_import.rs`、`tests/import_self_check.rs` |
| 版本 | v0.2.0 |
| 状态 | **已修复 · 生产验收通过** |

### BUG-2026-08-24-05 · 素材页刷新卡死

| 项 | 内容 |
|----|------|
| 影响范围 | Windows 桌面 · 素材库 |
| 现象 | 点击「刷新列表」后界面长时间无响应；素材量大时尤其明显 |
| 复现步骤 | 1. 扫描大量图片 2. 点击素材页「刷新列表」 |
| 期望 | 列表秒开；预览图后台渐进加载 |
| 根因 | `list_media_assets_cmd` 对每张图同步调用 `ensure_thumbnail` 解码并写 WebP，阻塞主线程 |
| 解决方案 | 列表只读已有缩略图；新增 `generate_media_thumbnails_cmd` 在 `spawn_blocking` 中分批（24 张/批）后台生成 |
| 防回归 | 列表/API 命令禁止同步图像解码；缩略图须异步或分批 |
| 关联代码 | `src/media.rs`、`src/lib.rs`、`views/MediaView.vue` |
| 版本 | v0.1.1 |
| 状态 | **已修复 · 生产验收通过** |

### BUG-2026-08-24-04 · 来源页 / 拆分向导主按钮可能空白

| 项 | 内容 |
|----|------|
| 影响范围 | Windows 桌面 · 来源文件 · 拆分向导 |
| 现象 | 与 BUG-02 同族：`SourcesView.vue`、`SplitWizard.vue` 的 scoped `button {}` 可能把 `.pk-btn--primary` 盖成白底白字，按钮看似空白 |
| 复现步骤 | 1. 进入来源文件或拆分向导 2. 观察带 Primary 样式的主操作按钮 |
| 期望 | 与内容 / 任务页一致，使用全局 `pk-btn` 分级样式 |
| 根因 | 组件内 legacy scoped 按钮样式未清理 |
| 解决方案 | 移除 scoped `button {}`，按钮统一为 `pk-btn` 分级样式 |
| 防回归 | 设计标准 §5.7；禁止页面级 `button {}` 覆盖全局按钮 Token |
| 关联代码 | `apps/desktop/src/views/SourcesView.vue`、`apps/desktop/src/components/SplitWizard.vue` |
| 版本 | v0.1.1 |
| 状态 | **已修复 · 生产验收通过** |

### BUG-2026-08-24-03 · 任务 / 今天页缺少帖子链接输入

| 项 | 内容 |
|----|------|
| 影响范围 | Windows 桌面 · 任务 · 今天 |
| 现象 | 待发 / 已发布任务卡片无「帖子链接（可选）」输入行；无法 inline 填 URL |
| 复现步骤 | 1. 创建任务并标记待发 2. 打开任务或今天页 3. 仅见复制 / 标记按钮，无链接框 |
| 期望 | 待发可填链接后标记已发布；已发布可改链接并保存；有链接时显示可点击 URL |
| 根因 | `TaskActionBar` 布局未单独渲染 URL 行；链接控件被折叠或遗漏 |
| 解决方案 | 重构 `TaskActionBar.vue`：按钮行 + 带标签的 URL 输入行；「重新打开」改为「撤销发布」 |
| 防回归 | 任务页 AC：待发 / 已发布均须可见 URL 控件 |
| 关联代码 | `TaskActionBar.vue`、`TasksView.vue`、`TodayView.vue`、`packages/i18n/locales/*.json` |
| 版本 | v0.1.0 |
| 状态 | **已修复 · 生产验收通过** |

### BUG-2026-08-24-02 · 内容页「新建内容」按钮空白

| 项 | 内容 |
|----|------|
| 影响范围 | Windows 桌面 · 内容库 |
| 现象 | 内容页右上角主按钮可见边框但无文字（白字白底） |
| 复现步骤 | 1. 打开内容页 2. 看右上角「新建内容」位置 |
| 期望 | 金色 Primary 按钮，文案可读 |
| 根因 | `ContentView.vue` scoped `button {}` 覆盖 `.pk-btn--primary` 颜色 |
| 解决方案 | 移除冲突 scoped 样式；`tokens.css` 增加 `button.pk-btn--*` 优先级 |
| 防回归 | 禁止页面 scoped 全局 `button {}`；Primary 按钮 visual QA |
| 关联代码 | `ContentView.vue`、`styles/tokens.css` |
| 版本 | v0.1.0 |
| 状态 | **已修复 · 生产验收通过** |

### BUG-2026-08-24-01 · 启动崩溃 duplicate column is_custom

| 项 | 内容 |
|----|------|
| 影响范围 | Windows 桌面 · SQLite 迁移 |
| 现象 | 第二次启动报错 `duplicate column name: is_custom`，应用无法打开 |
| 复现步骤 | 1. 升级到含迁移 003 的版本 2. 关闭并再次启动 |
| 期望 | 迁移幂等，重复启动不报错 |
| 根因 | 迁移 003 每次启动都执行 `ALTER TABLE … ADD COLUMN is_custom` |
| 解决方案 | `db.rs`：检查 `schema_migrations` + `pragma_table_info` 后再 ALTER；已存在则跳过 |
| 防回归 | 新迁移须登记 version；ALTER 前检查列是否存在 |
| 关联代码 | `apps/desktop/src-tauri/src/db.rs`、`migrations/003_channels.sql` |
| 版本 | v0.1.0 |
| 状态 | **已修复 · 生产验收通过** |

### DOC-2026-08-24-06 · 对照彭兔子 Bug 防回归不足

| 项 | 内容 |
|----|------|
| 影响范围 | PublishKit 全套 PRD / 技术 / 设计标准 |
| 现象 | 对照 [彭兔子项目Bug记录汇总](C:\Users\win\Desktop\彭兔子制版\docs\progress\项目Bug记录汇总.md) 自检时，发现 i18n 漏翻（pickLang）、region 混用、筛选不同步等高频模式未写入门禁 |
| 根因 | 首版文档侧重 FOUC 架构，未映射历史 Bug 模式 |
| 解决方案 | 新增 [防回归清单-彭兔子Bug教训.md](../防回归清单-彭兔子Bug教训.md)；PRD v2.2、技术 v1.2、设计 v1.2 增补 |
| 状态 | **已修复 · 文档** |

---

## 六、与彭兔子项目的隔离说明

PublishKit 为**独立产品**，Bug 编号与彭兔子三端项目**不共用**。  
彭兔子 Bug 仅作**防回归参考**，见 [防回归清单](../防回归清单-彭兔子Bug教训.md)，不在此重复登记。

---

## 七、修订记录

| 版本 | 日期 | 说明 |
|------|------|------|
| 1.5 | 2026-08-29 | CI：BUG-2026-08-29-01 Linux 依赖缺失导致 cargo check 失败 |
| 1.4 | 2026-08-26 | v0.2.2：BUG-2026-08-26-02 插件重复发布 confirm；生产验收通过 |
| 1.3 | 2026-08-26 | v0.2.0：BUG-2026-08-26-01 docx 标题命名空间；多格式导入验收 |
| 1.2 | 2026-08-24 | v0.1.1：BUG-04/05 验收通过；素材刷新卡死根因与分批缩略图方案 |
| 1.1 | 2026-08-24 | M1 v0.1.0：登记 BUG-01~04（03 项已验收，04 待修） |
| 1.0 | 2026-08-24 | 初版：索引结构、文档类 DOC 条目、GAP 跟踪、模板 |
