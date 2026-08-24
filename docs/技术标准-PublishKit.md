# PublishKit / 发稿匣 — 技术标准

| 字段 | 内容 |
|------|------|
| 文档版本 | v1.2 |
| 编写日期 | 2026-08-24 |
| 状态 | 待评审 |
| 关联文档 | [PRD](../PRD-营销内容资产与发布状态管理工具.md)、[设计标准](./设计标准-PublishKit.md) |

---

## 1. 架构总览

### 1.1 系统边界

```text
┌──────────────────────────────────────────────────────────────┐
│ PublishKit Desktop (Tauri 2)                                  │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────────┐  │
│  │ WebView UI  │  │ Rust Core    │  │ Local Services      │  │
│  │ Vue 3/React │◄─┤ FS / SQLite  │◄─┤ File Watcher        │  │
│  │ + i18n      │  │ Index / API  │  │ Thumbnail / Export  │  │
│  └─────────────┘  └──────────────┘  └─────────────────────┘  │
│         ▲                    ▲                                 │
│         │    localhost HTTP  │ 127.0.0.1:<port>               │
└─────────┼────────────────────┼─────────────────────────────────┘
          │                    │
┌─────────▼────────────────────▼─────────────────────────────────┐
│ Browser Extension (MV3)                                         │
│  Service Worker + Side Panel / Popup                            │
│  - Pairing token    - Copy helpers    - Publish backfill       │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│ Static Landing (optional)                                       │
│  get.pongrabbit.com / get.pongrabbit.cn — 下载、定价、隐私政策    │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 技术选型（M1 冻结）

| 层级 | 选型 | 理由 |
|------|------|------|
| 桌面壳 | **Tauri 2** | 体积小、本地 FS 友好、Rust 侧做索引与 SQLite |
| 前端 | **Vue 3 + TypeScript + Vite** | 已冻结（PRD §4.2）；组件化、i18n 生态成熟 |
| 样式 | CSS Variables + 组件库自建 | Token 见设计标准；不引入重型 UI 框架 |
| 本地库 | **SQLite**（`rusqlite` 或 `sqlx`） | 单文件、易备份、可加密同步 |
| 文件监听 | `notify`（Rust） | 跨平台 directory watch |
| 缩略图 | `image` crate + 本地 cache 目录 | 不污染用户素材目录 |
| 插件 | Chrome Extension **MV3** | Edge 同源包可复用 |
| 落地页 | 静态 HTML 或 Astro | 分 `/zh/` `/en/`，无运行时翻译 |
| CI | GitHub Actions | Windows build；macOS build 用 `macos-latest` |

**备选：** Electron 仅当 Tauri 在特定 Windows 环境 blocked 时启用；M1 不双轨。

### 1.3 仓库结构（建议）

```text
publishkit/
├── apps/
│   ├── desktop/          # Tauri + frontend
│   └── extension/        # MV3 插件
├── packages/
│   ├── shared/           # 类型、常量、API schema
│   └── i18n/             # locales/zh-CN.json, en.json
├── docs/                 # PRD、标准（可 submodule 或同仓）
├── landing/              # 静态官网
└── .github/workflows/    # build-win, build-mac, release
```

---

## 2. 平台与发布策略

### 2.1 阶段规划

| 阶段 | 平台 | 说明 |
|------|------|------|
| M1 MVP | **Windows 10/11 x64** | 自用验证；主开发环境 |
| M3 对外售卖 | Windows 安装包 + 自动更新 | NSIS / WiX 或 Tauri updater |
| M3+ | **macOS 12+**（Apple Silicon + Intel 或 universal） | 海外转化需要 |
| P2 | Linux | 非优先 |

### 2.2 macOS 构建要求

- **日常开发可在 Windows 完成**；业务代码跨平台
- **正式 macOS 安装包必须在 macOS 环境构建**（Xcode 工具链、codesign、notarization）
- 推荐：**GitHub Actions `macos-latest`** 自动打 `.dmg` / `.app`
- 需 **Apple Developer Program**（$99/年）签名与公证
- 无 Mac 硬件也可发布 Mac 版，但 **发布前需在真机做一轮验收**

### 2.3 Windows 构建

- 本地 + GitHub Actions `windows-latest`
- 代码签名：初期可选；对外售卖建议 Authenticode 证书
- 安装包：`PublishKit-setup-x64.exe`

### 2.4 自动更新

- Tauri Updater 或自建（检查 GitHub Releases / 官网 JSON）
- 更新通道：`stable`；Beta 可选
- 更新包 HTTPS；校验签名

---

## 3. 国际化（i18n）——硬性标准

> **背景：** 前项目「中文底 + 异步翻译英文」导致 FOUC。PublishKit 从架构层禁止。

### 3.1 两层语言模型

| 层 | 字段/配置 | 说明 |
|----|-----------|------|
| **UI Locale** | `settings.ui_locale`：`zh-CN` \| `en` | 控制界面字符串 |
| **Content Language** | `content_item.language`：`zh` \| `en` \| `bilingual` | 用户营销文案元数据，**不经过 UI 翻译** |

二者独立：用户可用 **英文 UI** 管理 **中文小红书稿**。

### 3.2 UI 语言解析顺序（启动时同步）

```text
0. 首次安装（DB 尚未存在）：跳过 SQLite，直接读 OS locale → 映射 → bundled locale
1. 读取 SQLite settings.ui_locale（用户曾手动设置 → 最高优先级）
2. 若空：读取 OS locale
   - Windows: GetUserDefaultLocaleName → zh-CN / en-US 等
   - macOS: NSLocale preferredLanguages
3. 映射到支持集：zh-* → zh-CN；其余 → en（M1 仅双语 UI）
4. 同步加载对应 locale JSON（bundled 进主包，禁止首屏 async fetch）
5. i18n ready → 创建并 show 主窗口；首次 locale 写入 settings
```

**主窗口创建：** `visible: false` 直至 step 5；可显示轻量 splash（splash 文案也需 i18n 或中性 logo）。

### 3.3 实现规范

| 规则 | 要求 |
|------|------|
| 组件内文案 | **禁止**硬编码中文/英文字符串；只用 `t('key')` |
| Locale 文件 | `packages/i18n/locales/zh-CN.json`、`en.json` 平行维护 |
| 默认语言 | 不存在「代码默认中文」；默认由 §3.2 决定 |
| 运行时切换 | 设置切换 UI 语言 → 内存换 locale → 即时重渲染；**不 reload 整页 FOUC** |
| 插件 | 打开 popup 前从 desktop API 取 `ui_locale`，或读 extension storage 缓存 |
| 官网 | 静态分路由 `/zh/`、`/en/`；Accept-Language 仅做 302 建议，不 JS 替换 DOM |
| CI 门禁 | `grep` 检测 `apps/desktop/src` 内 Unicode 中文（排除 i18n 目录与测试 fixture） |

### 3.4 验收用例（i18n）

| ID | 场景 | 通过标准 |
|----|------|----------|
| I18N-1 | OS=英文，首次安装 | 首帧无任何中文 UI（内容预览除外） |
| I18N-2 | OS=中文，首次安装 | 首帧无英文 UI 闪屏 |
| I18N-3 | 设置切 EN → 重启 | 全程无中文 UI 闪屏 |
| I18N-4 | EN UI 打开中文内容条目 | 导航/按钮英文；正文区中文原文 |
| I18N-5 | 插件 popup | 与 desktop UI locale 一致 |

### 3.5 反模式（禁止）

- HTML/模板写死中文，JS 再 `replace`
- 默认 `locale=zh-CN`，再读 region/intl 切换
- **`pickLang(zh, en)` 或任意「缺 key 就回中文」的 helper**
- **两套 `t()` 互调**（彭兔子 BUG-2026-07-17-09 栈溢出同族）
- 首屏 `fetch('/locales/en.json')` 后再渲染
- 用 Google Translate API 翻译 UI
- 把用户 MD 正文自动翻译后展示（除非用户显式请求，P2+）
- **用域名、Host、`region=` query 推断 UI 语言**（彭兔子 BUG-2026-07-14-01 同族）

### 3.6 彭兔子 Bug 教训 · 额外硬性规则

> 详见 [防回归清单-彭兔子Bug教训.md](./防回归清单-彭兔子Bug教训.md)

| ID | 规则 |
|----|------|
| I18N-R1 | 缺 i18n key 时不得 silent 回中文；dev 显示 `[missing:key]` |
| I18N-R2 | 全应用单一 i18n 实例；插件与桌面共享 `packages/i18n` |
| I18N-R3 | Toast、表格 cell、错误弹窗等动态 UI 必须 `t()` |
| I18N-R4 | CI：`zh-CN.json` 与 `en.json` key 集合 diff，不一致则 fail |
| I18N-R5 | API/离线/扫描错误信息走 error code + locale message |
| I18N-R6 | 新路由合并前：该路由 0 硬编码 UI 字符串（内容预览除外） |

**筛选状态（防 BUG-2026-08-03-01）：** 列表/看板筛选项与 URL query 或持久化 store **双向同步**；刷新后不得 UI 与数据范围不一致。

---

## 4. 数据层

### 4.1 数据库

- 引擎：SQLite 3
- 文件位置：
  - Windows: `%APPDATA%/PublishKit/publishkit.db`
  - macOS: `~/Library/Application Support/PublishKit/publishkit.db`
- 迁移：`migrations/001_init.sql` 递增版本号；启动时自动 migrate
- WAL 模式开启；崩溃恢复

### 4.2 核心表（逻辑模型）

```sql
-- 工作区
workspaces (id, name, created_at)

-- 项目
projects (id, workspace_id, name, brand_domestic, brand_overseas, default_site, ...)

-- 监视目录
watch_roots (id, workspace_id, path, kind: copy|media|video, ignore_globs_json)

-- 素材索引
media_assets (id, path, hash, kind, width, height, thumb_path, mtime, ...)

-- 源文档
source_documents (id, path, hash, title, last_scanned_at)

-- 内容条目
content_items (
  id, project_id, source_document_id, source_anchor,
  title, pattern_key, market, language, content_type,
  fields_json, tags_json, notes, compliance_note,
  created_at, updated_at
)

-- 条目-素材关联
content_media (content_item_id, media_asset_id, sort_order)

-- 渠道
channels (id, project_id, name, market, default_language, color, ...)

-- 发布任务
publish_tasks (
  id, content_item_id, channel_id,
  status, scheduled_at, published_at, publish_url,
  blocked_reason, checklist_json, note,
  created_at, updated_at
)

-- 设置
settings (key, value_json)  -- ui_locale, license, pairing_token, ...

-- 全文搜索（可选 FTS5）
content_items_fts(content_item_id, title, body)
```

### 4.3 文件与索引

- **不复制**用户大图进 DB；只存 path + hash + metadata
- Hash：文件 size + mtime 快速判变；变更时 SHA256 可选
- 缩略图：`%APPDATA%/PublishKit/cache/thumbs/<hash>.webp`
- 路径变更：通过 hash + 相对路径 + 「修复断链」工具重关联

### 4.4 备份与恢复

- 导出：`publishkit-backup-YYYYMMDD.zip` = db + settings + 可选 thumb cache
- 导入：校验 schema 版本；合并或覆盖可选
- 云同步（P1）：AES-256 加密整个 db 文件 → 用户云盘目录；不做自建服务器

---

## 5. 本地 HTTP API（桌面 ↔ 插件）

### 5.1 通则

- 绑定：`127.0.0.1` **only**（禁止 `0.0.0.0` 默认暴露）
- 端口：启动时选可用端口（如 17345–17365）；写入 settings
- 认证：Header `Authorization: Bearer <pairing_token>`
- Pairing：桌面端生成 8 位码；插件输入一次；交换 token；存 `chrome.storage.local`
- CORS：仅允许 `chrome-extension://<extension-id>`
- **业务 ID 不得从 URL path 下标 parse**（彭兔子 BUG-2026-08-15-04 同族）；只用 `/tasks/:id` 路由参数

### 5.2 端点（MVP）

| Method | Path | 说明 |
|--------|------|------|
| GET | `/health` | `{ ok, version, ui_locale }` |
| GET | `/tasks/today` | 今日待发任务列表 |
| GET | `/tasks/:id` | 任务详情 + 文案 fields + media paths |
| POST | `/tasks/:id/prepare` | 记录「准备发布」；返回 copy payload |
| POST | `/tasks/:id/publish` | body: `{ url, published_at? }` 标记已发布 |
| GET | `/settings/ui-locale` | 插件同步 UI 语言 |

响应 JSON UTF-8；错误 `{ error: { code, message } }`。

### 5.3 安全

- Token 可轮换；设置页「重新配对」使旧 token 失效
- 速率限制：本地 60 req/min 防插件 bug 死循环
- 不暴露任意文件读；media path 仅返回用户已索引路径

---

## 6. 浏览器扩展

### 6.1 结构（MV3）

```text
extension/
├── manifest.json
├── service_worker.js      # 与 desktop 通信
├── sidepanel.html         # 主 UI（优先 side panel）
├── popup.html             # 备选入口
├── content_scripts/       # 可选：检测平台 URL 模式
└── _locales/              # 插件 UI 字符串（en, zh_CN）
```

### 6.2 manifest 关键项

```json
{
  "manifest_version": 3,
  "name": "__MSG_extName__",
  "default_locale": "en",
  "permissions": ["storage", "activeTab", "sidePanel"],
  "host_permissions": ["http://127.0.0.1/*"],
  "optional_permissions": ["clipboardWrite"]
}
```

- **最小权限**；不过度申请 `<all_urls>` 读写
- `default_locale: en`；中文走 `_locales/zh_CN/messages.json`

### 6.3 功能实现要点

| 功能 | 实现 |
|------|------|
| 连接检测 | 轮询 `/health` 每 30s + 点击刷新 |
| 复制正文 | `navigator.clipboard.writeText`；失败 fallback execCommand |
| 复制图片 | MVP 调 desktop 复制到系统剪贴板（API 触发）；插件不直接读本地文件 |
| 回填 URL | `chrome.tabs.query({ active: true })` → `tab.url` 填入表单 |
| UI 语言 | 启动读 desktop `ui_locale` 写 extension storage；渲染前就绪 |

### 6.4 上架要求

#### Chrome Web Store

| 项 | 要求 |
|----|------|
| 开发者账号 | 一次性 $5 注册 |
| 隐私政策 | 公开 URL；声明只连 localhost、不上传内容 |
| 权限说明 | 商店表单解释每项 permission |
| 审核 | 通常数天；MV3 合规、无远程代码 |
| 包 | zip 上传；版本 `manifest.version` |

#### Microsoft Edge Add-ons

- Largely 复用 Chrome 包
- 微软开发者账号注册后提交

#### 不上架分发（内测）

- 「加载已解压的扩展程序」+ 文档说明
- 正式获客仍建议上架

### 6.5 验收

- 与 desktop 配对成功后，复制 + 回填全链路 < 30s
- desktop 关闭时插件显示明确离线态
- 中英文插件 UI 无闪屏（与 §3 一致）

---

## 7. 文件扫描与 MD 拆分

### 7.1 扫描

- 支持扩展名：`.md`, `.txt`, `.png`, `.jpg`, `.jpeg`, `.webp`, `.gif`, `.mp4`, `.mov`, `.pdf`
- Ignore globs 默认：`**/node_modules/**`, `**/.git/**`, `**/.cursor/**`
- **`ignore_globs` 必须相对 `watch_root` 解析**（彭兔子 BUG-2026-08-15-03：相对错根会扫进 node_modules）
- 并发：Rust 线程池；UI 显示进度（已扫描数/总数）
- 冷启动 1 万文件 < 30s（SSD 参考）
- 文本与 locale 文件读写：**UTF-8 无 BOM**（彭兔子 BUG-2026-07-17-13 / 08-05-03）

### 7.2 MD 拆分策略

| 策略 | 规则 |
|------|------|
| **智能识别（默认）** | 优先按 H3 识别渠道稿（小红书/Instagram/口播等）；否则按 H2/H1 正文块；自动跳过说明类标题 |
| 整篇 | 单文件 → 单条目 |
| 按 H1 / H2 / H3 | 对应 `#` / `##` / `###` 标题切块（通用，不绑定行业字段） |
| 手动 | UI 框选行号范围（P1） |

**段落分类（预览时）：**

| 类型 | 说明 | 默认导入 |
|------|------|----------|
| `content` | 正文块 | ✅ |
| `platform` | 渠道成稿（标题含平台/口播/Copy 等） | ✅ |
| `meta` | 操作说明、速查、发布前须知、调研对照等 | ❌（可手动勾选） |

说明类关键词示例：怎么用、使用说明、速查、发布前、调研、对照、分工…

**复制行为：** 存储仍保留 Markdown 真源；复制到剪贴板时转换为 **HTML 富文本 + 纯文本** 双格式，供发帖平台粘贴。

**非 MD 来源：**

| 格式 | M1 | 说明 |
|------|-----|------|
| `.md` / `.txt` | ✅ | 扫描 + 拆分 |
| 手动新建 | ✅ | 内容页「新建内容」 |
| `.docx` / `.doc` | P1 | 导入转换 |
| 富文本/HTML | P2 | 粘贴导入 |

拆分结果写 `content_items`；`source_anchor` 存 `{ startLine, endLine, heading }` JSON。

### 7.3 素材关联启发（MVP 规则）

1. 与 MD 同目录文件
2. 文件名含 `pattern_key` 或条目标题关键词
3. MD 内表格/Front matter 明示文件名
4. 人工 override 优先

---

## 8. 性能与可靠性

| 指标 | 目标 |
|------|------|
| 应用冷启动（已索引） | < 3s 到可交互 |
| 全量重扫 1 万文件 | < 30s |
| 搜索响应 | < 300ms（FTS 或 LIKE 优化） |
| Inspector 打开 | < 100ms |
| 缩略图生成 | 异步；列表先 placeholder |
| DB 写入 | 任务状态变更 fsync 安全 |
| 崩溃 | 重启后数据不丢；扫描可续 |

---

## 9. 安全与隐私

| 项 | 标准 |
|----|------|
| 数据驻留 | 默认全部本地 |
| 网络 | 桌面端仅更新检查/激活验证（可离线激活待定）；插件仅 localhost |
| 日志 | 不含用户正文；可开关诊断日志 |
| 许可证 | 激活码本地校验 + 可选在线验证 |
| 卸载 | 可选删除 cache；不删用户 MD/图片 |

---

## 10. 官网与域名

### 10.1 推荐部署

| 区域 | 域名 | 托管 |
|------|------|------|
| 海外 | `get.pongrabbit.com` | Cloudflare Pages |
| 国内 | `get.pongrabbit.cn` | 现有 ECS Nginx 静态目录 |

- **独立子域**，不挂在 `www` / `app` 路径下
- 页脚声明：PublishKit 为独立工具，与彭兔子制版账号无关
- 国内页脚保留 ICP / 公安备案号（主域已有备案主体）

### 10.2 DNS

```text
get.pongrabbit.com  CNAME  → xxx.pages.dev
get.pongrabbit.cn   A      → 国内 ECS IP
```

- **勿**将 `pongrabbit.cn` NS 改到 Cloudflare（备案解析风险，见彭兔子运维文档）

### 10.3 落地页技术

- 静态 HTML / Astro；`/zh/`、`/en/` 独立构建
- 下载链接指向 GitHub Releases 或国内网盘
- 支付：Lemon Squeezy（海外）/ 面包多或爱发电（国内）外链

---

## 11. 许可证与商业化（技术）

| 版本 | 限制实现 |
|------|----------|
| Free | `projects.count <= 1` AND `content_items.count <= 50` |
| Pro | 本地 license 文件或激活码写入 settings |
| Team | 同上 + 协作功能 flag |

- 激活：输入激活码 → 可选在线校验 → 写入 `settings.license`
- 免费版超限：只读或禁止新建，不删用户数据

---

## 12. 测试策略

| 层级 | 范围 |
|------|------|
| 单元 | MD 拆分、状态机、i18n key 完整性 |
| 集成 | SQLite 迁移、API 端点、file watcher |
| E2E | Playwright：选任务 → 复制 → mock 插件 publish |
| i18n | 自动化截图对比首帧 locale |
| 手动 | 真机 Windows；Mac CI 产物真机一轮 |

**发布门禁：**

- [ ] I18N-1 ~ I18N-5 通过
- [ ] I18N-R1 ~ I18N-R4（防回归清单）通过
- [ ] AC1 ~ AC8（PRD）通过
- [ ] 插件与 desktop 版本兼容矩阵 documented
- [ ] 隐私政策 URL 可访问

---

## 13. 版本与兼容

| 组件 | 版本策略 |
|------|----------|
| Desktop | SemVer `1.0.0` |
| Extension | 独立 SemVer；major 与 desktop API breaking 对齐 |
| DB schema | migration 递增；向后兼容 1 major |
| API | `/health.version` + `api_version` 字段 |

---

## 14. 修订记录

| 版本 | 日期 | 说明 |
|------|------|------|
| v1.0 | 2026-08-24 | 首版：架构、平台、i18n、API、插件、域名、数据层 |
| v1.1 | 2026-08-24 | 自检：Vue 冻结、首次安装 i18n 引导、与 PRD v2.1 验收对齐 |
| v1.2 | 2026-08-24 | 对照彭兔子 Bug：§3.6 防 i18n 漏翻、筛选同步、PATH/UTF-8、API 路由 ID |
