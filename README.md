# PublishKit / 发稿匣

跨境营销**内容资产与发布状态**管理工具（桌面端 + 浏览器插件）。  
不做营销平台账号托管，不做一键全网分发。

## 文档

| 文档 | 路径 |
|------|------|
| PRD | [PRD-营销内容资产与发布状态管理工具.md](./PRD-营销内容资产与发布状态管理工具.md) |
| 技术标准 | [docs/技术标准-PublishKit.md](./docs/技术标准-PublishKit.md) |
| 设计标准 | [docs/设计标准-PublishKit.md](./docs/设计标准-PublishKit.md) |
| 产品进度 | [docs/progress/产品进度一览.md](./docs/progress/产品进度一览.md) |
| Bug 记录 | [docs/progress/项目Bug记录汇总.md](./docs/progress/项目Bug记录汇总.md) |

## 当前阶段

**M2 收尾 / F10** — 桌面端 v0.1.9-dev，插件 Side Panel v0.1.9；`npm run tauri dev` 本地运行，`npm run build:desktop` 可打 Windows 包。

## 发布

打 tag 触发 GitHub Actions 构建安装包（见 [开发环境 · GitHub Release](./docs/开发环境.md#github-releaseci)）。

## 本地开发

详见 [docs/开发环境.md](./docs/开发环境.md)。

```powershell
npm install
npm run tauri dev
```

## 仓库

https://github.com/aiyahetun/pongrabbit-PublishKit
