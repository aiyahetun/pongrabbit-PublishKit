# PublishKit · 彭兔子 Bug 教训与防回归清单

| 字段 | 内容 |
|------|------|
| 文档版本 | v1.0 |
| 编写日期 | 2026-08-24 |
| 来源 | [彭兔子制版/docs/progress/项目Bug记录汇总.md](C:\Users\win\Desktop\彭兔子制版\docs\progress\项目Bug记录汇总.md)（v1.31） |
| 关联 | [PRD](../PRD-营销内容资产与发布状态管理工具.md)、[技术标准](./技术标准-PublishKit.md)、[设计标准](./设计标准-PublishKit.md) |

> 本文档从彭兔子三端项目 **200+ 条** Bug 中抽取与 PublishKit **可复现同族** 的模式，用于自检与发布门禁。制版引擎、支付、Paddle、小程序等域内问题不照搬，只提取**方法论**。

---

## 1. 教训总览（按 PublishKit 相关度）

| 等级 | 彭兔子 Bug 模式 | PublishKit 是否已覆盖 | 动作 |
|------|-----------------|----------------------|------|
| 🔴 P0 | i18n 漏翻 / pickLang 恒回中文 / 首屏闪旧 UI | 部分（FOUC 已写，细节不足） | **补技术标准 §3.6 + CI** |
| 🔴 P0 | region/平台 与 UI 语言/数据范围 混用 | 部分（UI vs content 已分） | **禁止 region 推断 locale** |
| 🟠 P1 | 筛选 UI 与内部状态/URL 不一致 | 未写 | **补设计 §5.6** |
| 🟠 P1 | 路径/ID 字符串 split 取错段 | 部分（结构化 ID） | **API 与路由规范** |
| 🟠 P1 | watch ignore 相对路径解析错误 | 部分 | **扫描 ignore 单测** |
| 🟠 P1 | UTF-8/BOM 破坏文案与清单 | 未写 | **补编码规范** |
| 🟡 P2 | 双 vendor/双路径 热更漏文件 | 不适用（桌面安装包） | **版本号 + 完整性校验** |
| 🟡 P2 | 原生控件与设计系统不一致 | 部分 | **禁裸 select** |
| 🟡 P2 | 错误信息/Toast 硬编码中文 | 部分 | **错误码 i18n** |
| ⚪ N/A | 跨子域 localStorage | 桌面单进程，风险低 | 插件 storage 独立即可 |
| ⚪ N/A | CSP / Paddle iframe | 无 Web 支付 iframe | — |

---

## 2. i18n 同族 Bug（最高优先级）

### 2.1 彭兔子典型条目

| Bug ID | 现象 | 根因摘要 |
|--------|------|----------|
| BUG-2026-07-24-12 | `lang=en` 步骤7仍中文 | `pickLang` 恒返回中文；`en.json` 缺 key |
| BUG-2026-08-04-01 | 约束 Status、Grid 等仍中文 | 硬编码 + 动态 DOM 未走 i18n |
| BUG-2026-07-21-01 | 热更后仍中文 | i18n 文件路径/清单错误，生产未加载 en |
| BUG-2026-07-19-05 | 首屏闪旧版三 Tab | 先 render 旧壳再 async 换 UI |
| BUG-2026-07-17-09 | 官方版型空白 | 两套 `t()` 互相回调，栈溢出 |
| BUG-2026-07-28-27 | batch 页全中文 | 子页面未调 `applyBatchShell` |
| BUG-2026-08-14-05 | 海外页串中文口号 | 内容语言未按 market 隔离 |
| BUG-2026-07-19-14 | API 离线错误仍中文 | 错误文案未 i18n |

### 2.2 PublishKit 已有防护

- UI Locale 与 Content Language 分离（PRD §9.2、技术标准 §3.1）
- 主窗口 `visible: false` 直至 i18n ready（技术标准 §3.2）
- bundled locale，禁止首屏 async fetch（技术标准 §3.3）
- CI grep 硬编码中文（技术标准 §3.3）

### 2.3 本次自检**新增**要求（写入技术标准 v1.2）

| ID | 规则 |
|----|------|
| I18N-R1 | **禁止** `pickLang(zh, en)` 类函数默认回退中文；缺失 key 时显示 `[missing:key]` 或 en fallback，并打 dev 日志 |
| I18N-R2 | **单一 i18n 入口**（如 `vue-i18n`）；禁止第二套 `t()` 互调 |
| I18N-R3 | 动态插入 DOM（Toast、表格 cell、插件 popup）**必须**走 i18n，禁止模板字符串写中文 |
| I18N-R4 | `zh-CN.json` 与 `en.json` **key 集合必须完全一致**；CI 跑 key diff |
| I18N-R5 | 错误码 → message 走 locale 表；离线/API 失败不得硬编码中文 |
| I18N-R6 | 新页面/新路由合并前：该路由下 **0 硬编码 UI 字符串**（内容预览区除外） |
| I18N-R7 | 插件与桌面 **共用** `packages/i18n` 包或 generated 同步，禁止第三份文案 |

### 2.4 验收补充

| ID | 场景 |
|----|------|
| I18N-R8 | 故意删除 en.json 某 key → 英文 UI 不出现中文，出现 missing 标记或英文 fallback |
| I18N-R9 | 切换路由（今天→内容→任务）全程无语言闪变 |
| I18N-R10 | 插件在 desktop 未启动时，offline 文案与上次 locale 一致 |

---

## 3. region / market / 平台 混用

### 3.1 彭兔子典型条目

| Bug ID | 现象 | 根因 |
|--------|------|------|
| BUG-2026-07-14-01 | `.cn` 被当 intl | `region.js` Host 含 pongrabbit → intl |
| BUG-2026-08-13-06 | 国内订单变 USD | 库内 `region=intl` |
| BUG-2026-08-03-01 | 下拉「PC 国内」但数据是海外 | UI 选中项与 hash `platform=pc_intl` 不同步 |

### 3.2 PublishKit 映射

| 彭兔子概念 | PublishKit 对应 | 规则 |
|------------|-----------------|------|
| `region=intl/cn` | **不存在** | 禁止用域名、query、Host 推断 UI 语言 |
| 国内/海外订单 | `content_item.market` / `channel.market` | 仅用于**内容筛选**，不是 UI locale |
| 运营台下拉 | 任务/内容筛选项 | 选中项必须与 URL query 或持久化 filter **双向同步** |

### 3.3 新增要求

- 筛选状态单一数据源（Pinia/store）；改 chip 即改 query，刷新后从 query 恢复
- 禁止 `if (hostname.includes('cn')) locale = zh` 类逻辑

---

## 4. 路径、文件、编码

### 4.1 彭兔子典型条目

| Bug ID | 现象 | 根因 |
|--------|------|------|
| BUG-2026-08-15-04 | exportId 解析 404 | `pathname.split('/')[5]` 取下标错误 |
| BUG-2026-08-13-04 | 快照名与 export_id 不一致 | 文件名与 ID 双轨未校验 |
| BUG-2026-08-15-03 | ignore 未生效打进 node_modules | ignore 相对错误根目录 |
| BUG-2026-08-05-03 | 中文变 `?` | PowerShell 非 UTF-8 写 JS |
| BUG-2026-07-17-13 | 热更 SKIP | manifest UTF-8 BOM |
| BUG-2026-07-18-05 | 热更路径 vendor/draftsman 错误 | 路径别名未统一 |

### 4.2 PublishKit 新增要求

| ID | 规则 |
|----|------|
| PATH-R1 | 业务 ID 只用 UUID/自增 ID；**禁止**从 `path.split('/')` 取 ID |
| PATH-R2 | `media_assets.path` 存绝对路径 + 可选 `watch_root_id` 相对路径；盘符变更走「修复断链」 |
| PATH-R3 | `ignore_globs` **相对 watch_root** 解析；单测：嵌套根目录不误扫 node_modules |
| PATH-R4 | MD/ locale JSON / 导出 txt 统一 **UTF-8 无 BOM**；读写显式 UTF-8 |
| PATH-R5 | 发布包 manifest（zip 内 filelist）无 BOM；Windows 路径用 `/` 或规范化再存 |
| PATH-R6 | 素材 hash 变更时保留 `linked_items`，提示用户而非静默断链 |

---

## 5. UI / 交互同族

### 5.1 彭兔子典型条目

| Bug ID | 现象 |
|--------|------|
| BUG-2026-08-24-02 | 原生 160px select 不符合 Warm Paper |
| BUG-2026-08-15-02 | 次要操作做成 Primary 按钮 |
| BUG-2026-07-19-05 | 首屏 UI 结构闪变 |

### 5.2 PublishKit 要求（设计标准已有，强调）

- 筛选用 styled combobox 或 chip，避免未样式化原生 `<select>`
- 每屏 Primary 按钮 ≤1（设计标准 §3.1 已有）
- 路由切换不卸载整壳再挂载另一套布局（避免「闪旧 Tab」）

---

## 6. 插件与桌面通信

### 6.1 可借鉴

| 彭兔子 | PublishKit |
|--------|------------|
| localStorage 隔离/失败 | 插件 `chrome.storage.local` + 配对 token；desktop 关则 offline |
| URL token 跨域握手 | **不需要**（localhost API） |
| 路径权限过宽 | 已限制 `127.0.0.1` + Bearer |

### 6.2 新增

- 插件版本与 desktop API `api_version` 不兼容时，明确提示升级，禁止半连通状态 silently fail

---

## 7. 发布门禁（合并 PRD + 技术 + 本清单）

### M1 末（桌面）

- [ ] I18N-1 ~ I18N-4 + I18N-R1 ~ R4
- [ ] AC1 ~ AC4、AC7、AC8
- [ ] PATH-R3 ignore 单测
- [ ] locale key diff CI 绿

### M2 末（+ 插件）

- [ ] AC6、I18N-5、I18N-R10
- [ ] 插件 offline/online 状态清晰

### M3 末（可售卖）

- [ ] AC5、AC9、AC10
- [ ] 安装包完整性校验（PATH-R4 等价物：签名或 hash manifest）

---

## 8. 自检结论（2026-08-24）

| 文档 | 对照 Bug 记录前 | 对照后 |
|------|----------------|--------|
| PRD v2.1 | i18n/边界/里程碑已较好 | 风险表需补 region/筛选/i18n 漏翻 |
| 技术 v1.1 | FOUC 架构正确 | 缺 pickLang 禁令、key diff、编码、路径解析 |
| 设计 v1.1 | Token/组件齐全 | 缺筛选同步、原生控件约束 |

**结论：** 彭兔子 Bug 的 **i18n 同族** 与 PublishKit 文档方向一致，但原文档**防细节回归不足**。本清单 + 技术标准 v1.2 补丁后，可视为与历史教训对齐。

---

## 9. 修订记录

| 版本 | 日期 | 说明 |
|------|------|------|
| v1.0 | 2026-08-24 | 基于项目Bug记录汇总 v1.31 首版 |
