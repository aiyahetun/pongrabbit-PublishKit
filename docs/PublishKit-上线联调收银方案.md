# PublishKit · 上线 / 联调 / 收银方案（执行手册）

| 属性 | 内容 |
|------|------|
| 文档版本 | **v1.2**（2026-09-22） |
| 状态 | **Phase 0-1 本地验收通过**（2026-09-22）· 待 Phase 0 部署 |
| 关联 | [域名与部署](../../彭兔子制版/docs/域名与部署.md) · [官网验收清单](./官网验收清单.md) · [SEO与GEO方案](./PublishKit-SEO与GEO方案.md) · [官网落地页方案](../PublishKit-官网落地页方案.md) |
| 代码位置 | 静态站 `website/` · 桌面端 `apps/desktop/` · **收银 API 待建**（见 §4） |

> **用法**：按 Phase 顺序操作；每完成一步在 §8「进度记录」打勾；遇到问题把现象写进 §8，我会据此更新本文档。

---

## 1. 目标与边界

### 1.1 要交付什么

| 交付物 | 说明 |
|--------|------|
| 国内官网 | `https://get.pongrabbit.cn/zh/` |
| 海外官网 | `https://get.pongrabbit.com/en/` |
| **SEO / GEO** | sitemap、hreflang、结构化数据、站长登记（见 [SEO与GEO方案](./PublishKit-SEO与GEO方案.md)） |
| 国内收银 | **微信支付 Native**（扫码），¥139 买断 |
| 海外收银 | **Paddle** 一次性付款，$38 买断 |
| 履约 | 支付成功后邮件发送 `PKPRO-` 激活码；用户在桌面端「设置 → 许可证」粘贴 |

### 1.2 不在本期范围

- 彭兔子制版账号体系打通（PublishKit 独立工具，页脚已声明）
- 在线验码 / 防盗版服务端（桌面端当前仅本地校验 `PKPRO-` 前缀）
- macOS App Store 签名分发（仍走 GitHub Releases）

### 1.3 已确认决策（来自讨论）

| # | 决策 |
|---|------|
| D1 | 域名：国内 `.cn`、海外 `.com`，挂在 **pongrabbit 子域 `get`** |
| D2 | 国内支付：**微信支付**（非爱发电/面包多） |
| D3 | 海外支付：**Paddle**（非 Lemon Squeezy） |
| D4 | 落地页保持 **静态 HTML**；支付必须走 **HTTPS API + Webhook** |
| D5 | 联系邮箱：**国内 `support@pongrabbit.cn`** · **海外 `support@pongrabbit.com`**（与彭兔子 www 一致；**不用** `hi@` 占位） |

### 1.4 待你确认（操作前填 §8）

| # | 问题 | 建议默认 |
|---|------|----------|
| Q1 | 收银 API 放在哪？ | 扩展现有 `api.pongrabbit.cn/com`（复用 Resend/Paddle/微信配置） |
| Q2 | 微信商户用哪个？ | PC 网站 Native 商户（与小程序虚拟支付 **分开**，见彭兔子域名文档 §生产部署） |
| Q3 | Phase 0 是否先上静态站（支付后开）？ | **是** — 先 `get.*` 可访问 + 下载，Pro 按钮暂提示「即将开放」或邮件购码 |

---

## 2. 架构

### 2.1 域名与流量

```text
                    ┌─────────────────────────────────────┐
                    │  get.pongrabbit.cn  (国内 ECS Nginx) │
                    │  静态目录 /var/www/publishkit        │
                    │  /zh/  /en/  /privacy.html           │
                    └──────────────┬──────────────────────┘
                                   │ 购买 Pro（Phase 2）
                                   ▼
                    ┌─────────────────────────────────────┐
                    │  api.pongrabbit.cn                     │
                    │  POST /api/v1/publishkit/orders        │
                    │  POST /api/v1/publishkit/webhooks/wx   │
                    └─────────────────────────────────────┘

                    ┌─────────────────────────────────────┐
                    │  get.pongrabbit.com (CF Pages 或 Nginx)│
                    └──────────────┬──────────────────────┘
                                   │ 购买 Pro（Phase 1）
                                   ▼
                    ┌─────────────────────────────────────┐
                    │  api.pongrabbit.com                    │
                    │  POST /api/v1/publishkit/checkout       │
                    │  POST /api/v1/publishkit/webhooks/paddle│
                    └─────────────────────────────────────┘
```

对齐 [彭兔子制版/docs/域名与部署.md](../../彭兔子制版/docs/域名与部署.md)：

| 子域 | 彭兔子用途 | PublishKit |
|------|------------|------------|
| `www` | 纸样营销 | 不动 |
| `app` | 制版 SaaS | 不动 |
| `api` | 后端 | **PublishKit 收银路由** |
| **`get`** | （新增） | **官网静态站** |

### 2.2 支付 → 激活码

```text
用户点「购买 Pro」
  → 前端 site.js 调 api.* 创建订单
  → 国内：返回微信 code_url，页内展示二维码
  → 海外：Paddle Checkout 打开（Hosted 或 Paddle.js）
  → Webhook 验签 + 幂等
  → 生成 PKPRO-XXXX-XXXX-XXXX 入库
  → Resend 发邮件（noreply@pongrabbit.com）
  → 可选：/success.html?order= 轮询展示密钥
```

桌面端验码逻辑（已实现，无需改即可上线 v1）：

- 前缀 `PKPRO-`，长度 ≥ 12（见 `apps/desktop/src-tauri/src/license.rs`）

---

## 3. 方案自检（2026-09-22）

| # | 检查项 | 结论 | 处理 |
|---|--------|------|------|
| S1 | 静态站能否直接接微信/Paddle 密钥？ | ❌ 不行 | 必须 API + Webhook |
| S2 | `get` 子域是否已在彭兔子 Nginx？ | ❌ 未配置 | Phase 0 新增 server 块 + DNS + 证书 |
| S3 | `api.pongrabbit.com` 是否稳定？ | ⚠️ 历史进度曾记 DNS 异常 | Phase 1 前先 `curl` 健康检查 |
| S4 | 微信 Web 与小程序商户是否混用？ | ⚠️ 禁止混用 | 只用 PC Native 商户号 |
| S5 | Paddle 域名白名单 | ⚠️ 曾缺 domain 致 400 | Dashboard 加 `get.pongrabbit.com` |
| S6 | 国内 ICP / 公安备案 | ⚠️ `get.pongrabbit.cn` 需与主体一致 | Phase 0 页脚加 ICP；子域按备案规则处理 |
| S7 | `terms.html` | ❌ 未写 | Phase 0 可先链隐私政策；上线收银前补 terms |
| S8 | 官网素材 | ✅ P0 图已接入 | 见 [官网验收清单](./官网验收清单.md) |
| S9 | 本地验码无服务端 | ⚠️ v1 可接受 | 文档注明；后续可加在线验码 |
| S10 | 与制版 Paddle Price 混用 | ❌ 禁止 | 新建 PublishKit 专用 `pri_*` |
| S11 | og:image 相对路径 | ❌ 分享裂图 | Phase 0-SEO E1 改绝对 URL |
| S12 | 无 sitemap / robots / JSON-LD | ❌ 不利于收录与 GEO | Phase 0-SEO 补齐后再公网收录 |

**自检结论**：方案可行。**上线顺序**：Phase 0-1 本地验收 → **Phase 0-SEO 工程** → Phase 0 部署 → 站长登记 → Phase 1/2 收银。

---

## 4. 收银 API 规格（开发清单）

> 代码建议放在 **彭兔子 PC 工作台 `penrabbit-platform/packages/api`** 新路由，或 PublishKit 仓库 `services/license-api/`（二选一，Q1 确认后实施）。

### 4.1 路由

| 方法 | 路径 | 区域 | 说明 |
|------|------|------|------|
| POST | `/api/v1/publishkit/orders` | CN | 创建订单，返回 `{ orderId, codeUrl }` |
| GET | `/api/v1/publishkit/orders/:id` | CN/INTL | 查询状态 `pending/paid/failed` |
| POST | `/api/v1/publishkit/checkout` | INTL | 创建 Paddle 交易，返回 `checkoutUrl` 或 `transactionId` |
| POST | `/api/v1/publishkit/webhooks/wechat` | CN | 微信支付回调 |
| POST | `/api/v1/publishkit/webhooks/paddle` | INTL | Paddle `transaction.completed` 等 |

### 4.2 数据表（最小字段）

`publishkit_orders`:

- `id`, `region` (cn/intl), `amount`, `currency`, `status`, `wx_out_trade_no` / `paddle_transaction_id`
- `license_key` (nullable until paid), `email`, `created_at`, `paid_at`

### 4.3 前端改动（`website/assets/site.js`）

Phase 0（临时）：

```js
checkoutUrl: { en: "", zh: "" }  // 弹窗提示「支付即将开放」
```

Phase 1 后：

```js
// en: pkCheckout() → fetch api.pongrabbit.com/.../checkout → Paddle
// zh: pkCheckout() → fetch api.pongrabbit.cn/.../orders → 展示二维码
```

### 4.4 环境变量（参考彭兔子）

**海外 `.env.intl`**（已有 Paddle，新增 PublishKit Price ID）：

- `PADDLE_API_KEY`, `PADDLE_WEBHOOK_SECRET`, `PADDLE_CLIENT_TOKEN`
- `PADDLE_PRICE_PUBLISHKIT_PRO` = `pri_xxx`（**新建**）
- `RESEND_API_KEY`, `PUBLIC_BASE_URL=https://get.pongrabbit.com`

**国内 `.env.cn`**：

- 微信商户号、APIv3 密钥、证书序列号、回调 URL
- `PUBLISHKIT_WX_NOTIFY_URL=https://api.pongrabbit.cn/api/v1/publishkit/webhooks/wechat`

---

## 5. Phase 0-SEO — SEO 与 GEO（**首次上传前完成**）

> 完整策略见 **[PublishKit-SEO与GEO方案.md](./PublishKit-SEO与GEO方案.md)**（**v1.1 含竞品 SEO/GEO 调研**）。  
> 原则：**不要让搜索引擎第一次抓取到「半成品 meta」** — 与 Phase 0 部署同一批上传。  
> **占词策略**：抢「本地内容管理 / 发布管理」，不抢「一键分发 / 社媒调度器」（详见方案 §2）。**不用「台账」**——用户不搜、也难理解。

### 为什么要单独一步？

| 缺口 | 风险 |
|------|------|
| `og:image` 相对路径 | 微信 / Slack 分享无图 |
| 无 `canonical` / `hreflang` | cn/com 双站语言信号混乱 |
| 无 JSON-LD / FAQ Schema | Google 富摘要、AI 引用质量差 |
| 无 `llms.txt` | GEO：AI 易把产品说成「自动发帖工具」 |
| 未提交 GSC / 百度 | 收录慢，无法监控索引 |

### Step 0-SEO-1 · 工程清单（我来改代码，你验收）

| ID | 内容 | 状态 |
|----|------|------|
| E1 | canonical + og 绝对 URL | ✅ |
| E2 | meta description 对齐最新文案 | ✅ |
| E3 | hreflang（cn↔com） | ✅ |
| E4 | twitter:card | ✅ |
| E5 | JSON-LD（SoftwareApplication + FAQPage） | ✅ |
| E6 | `website/robots.txt` | ✅ |
| E7 | `website/sitemap.xml` | ✅ |
| E8 | `website/llms.txt`（含竞品边界 Compare 段） | ✅ |
| E9 | `website/scripts/verify-seo.mjs` | ✅ |
| E10 | 首页 manifesto 对比表 + FAQ 文案校对 | ✅ |
| E11 | footer 不链 llms（不对用户透传） | ✅ |

**P1（上线后 2 周内，非阻塞上线）**：静态对比页 `vs-buffer` / `vs-yixiaoer` — 见 SEO 方案 §11。

**你回复**：「开始做 0-SEO 工程」→ 我按表改 `website/` 并跑验收脚本。

### Step 0-SEO-2 · 本地 SEO 验收

```powershell
node website/scripts/verify-seo.mjs
npx --yes serve website
# 浏览器查看源代码：canonical、hreflang、application/ld+json
```

### Step 0-SEO-3 · 上线后站长登记（部署完成后 24h 内）

| 区域 | 平台 | 动作 |
|------|------|------|
| 海外 | Google Search Console | 添加 `get.pongrabbit.com`，提交 sitemap |
| 海外 | Bing Webmaster | 可选 |
| 国内 | 百度站长 | 验证 `get.pongrabbit.cn`，提交 sitemap |
| 国内 | 神马 | 复制 www 站验证 meta 或新申请 |
| 双域 | 微信分享实测 | 发链接到文件传输助手，看卡片 |

**Phase 0-SEO 完成标准**：E1～E11 全绿 + 分享卡片正常 + GSC/百度「已提交 sitemap」（索引可等待数天）。

---

## 6. Phase 0 — 静态官网上线

**目标**：`get.pongrabbit.cn/zh/`、`get.pongrabbit.com/en/` 可访问；下载可用；Pro 可暂不可用。

### Step 0-1 · 本地终验

```powershell
cd C:\Users\win\Desktop\内容营销平台工具
npx --yes serve website
```

按 [官网验收清单](./官网验收清单.md) AC1～AC10 勾选。  
**你做完回复我**：「Phase 0-1 通过」或列出未通过项。

### Step 0-2 · 打包上传目录

上传内容为仓库 **`website/` 整个文件夹**（含 `assets/`、`zh/`、`en/`、`privacy.html`、`index.html`）。

目标路径（国内 ECS）：

```text
/var/www/publishkit/
├── index.html
├── privacy.html
├── zh/index.html
├── en/index.html
└── assets/...
```

### Step 0-3 · DNS

| 记录 | 类型 | 值 |
|------|------|-----|
| `get.pongrabbit.cn` | A | 国内 ECS 公网 IP（与 `www` 同机） |
| `get.pongrabbit.com` | CNAME 或 A | Cloudflare / 海外机（与 `www` 策略一致） |

**你做完回复我**：DNS 已加 /  propagating / 已生效。

### Step 0-4 · SSL 证书（国内）

与 `app.pongrabbit.cn` 相同流程，为 **`get.pongrabbit.cn`** 申请证书，放到：

```text
/etc/nginx/ssl/get.pongrabbit.cn_nginx/
```

### Step 0-5 · Nginx 配置（国内）

在 `/etc/nginx/conf.d/pongrabbit.cn.conf` **追加**（勿覆盖现有 www/app）：

```nginx
server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name get.pongrabbit.cn;

    ssl_certificate     /etc/nginx/ssl/get.pongrabbit.cn_nginx/get.pongrabbit.cn.pem;
    ssl_certificate_key /etc/nginx/ssl/get.pongrabbit.cn_nginx/get.pongrabbit.cn.key;

    root /var/www/publishkit;
    index index.html;

    location = / {
        return 302 /zh/;
    }

    location / {
        try_files $uri $uri/ =404;
    }
}
```

```bash
nginx -t && systemctl reload nginx
```

### Step 0-6 · 海外静态站

**方案 A（推荐）Cloudflare Pages**

- 项目根目录：`website`
- Build：无（纯静态）
- 自定义域：`get.pongrabbit.com`
- 根路径行为：与 `website/index.html` 语言跳转一致

**方案 B** 海外 ECS Nginx：与 Step 0-5 类似，改 `server_name get.pongrabbit.com`。

### Step 0-7 · 页脚合规（国内）

在 `website/zh/index.html` 页脚增加 ICP 备案号（与 `www.pongrabbit.cn` 同主体）。  
公安备案链接若全站要求统一，一并复制 www 站脚代码。

### Step 0-8 · 线上冒烟

| URL | 预期 |
|-----|------|
| https://get.pongrabbit.cn/zh/ | 200，中文页，图片正常 |
| https://get.pongrabbit.cn/privacy.html | 200 |
| https://get.pongrabbit.com/en/ | 200，英文页 |
| 下载按钮 | 跳转 GitHub Releases |

**Phase 0 完成标准**：以上 URL 全绿；Console 无报错。

---

## 7. Phase 1 — 海外 Paddle 联调

| Step | 动作 | 验收 |
|------|------|------|
| 1-1 | Paddle 新建 Product「PublishKit Pro Lifetime」Price **$38** 一次性 | 拿到 `pri_*` |
| 1-2 | Dashboard → Approved domains 加 `get.pongrabbit.com` | 无 400 |
| 1-3 | 实现 API checkout + webhook + 发码邮件 | Sandbox 假付一笔收到 `PKPRO-` |
| 1-4 | 改 `site.js` 海外结账调 **`https://app.pongrabbit.com`**（不要用无 DNS 的 `api.pongrabbit.com`） | `/en/` 真付或 Sandbox 全流程 |
| 1-4b | 上线前扫描 | `node website/scripts/verify-live.mjs` 退出码 0；人工项见 [海外上线前检查](./PublishKit-海外上线前检查.md) |
| 1-5 | Live 切换 + 监控 Webhook 失败 | 生产 1 笔 $38 验收 |

---

## 8. Phase 2 — 国内微信支付联调

| Step | 动作 | 验收 |
|------|------|------|
| 2-1 | 确认 PC Native 商户号可售「软件/授权」类 | 商户平台产品配置 OK |
| 2-2 | 微信商户平台配置回调 URL | 指向 `api.pongrabbit.cn` webhook |
| 2-3 | 实现 Native 下单 + 回调 + 发码 | 沙箱/1 分钱测试收到码 |
| 2-4 | `get.pongrabbit.cn` 增加扫码 UI（弹窗或 `/pay/`） | 手机扫码付 ¥139 |
| 2-5 | 页脚 ICP + 隐私 URL 写入商户配置 | 审核无拒 |

---

## 9. 进度记录（边做边填）

### 8.1 决策答复

| 项 | 你的选择 | 日期 |
|----|----------|------|
| Q1 API 放哪 | **同意**：扩 `api.pongrabbit.cn` / `api.pongrabbit.com` | 2026-09-22 |
| Q2 微信商户 | **同意**：复用 PC Native 商户（与小程序分开） | 2026-09-22 |
| Q3 先上静态站 | **同意** | 2026-09-22 |
| 国内部署 | **彭兔子同一台 ECS**（`/var/www/publishkit`） | 2026-09-22 |
| 海外部署 | **腾讯云 ECS + WinSCP**（`get.pongrabbit.com`，Certbot） | 2026-09-22 |
| ICP / 公安备案 | **京ICP备2026012465号-3** · **京公网安备 11010802049391号**（与 www 同主体，见 `cn-compliance.js`） | 2026-09-22 |

### 8.2 Phase 0-SEO checklist

- [x] 0-SEO-1 工程 E1～E11 完成（2026-09-22）
- [x] 0-SEO-2 `verify-seo.mjs` 本地通过（2026-09-22，随 Phase 0-1）
- [x] 0-SEO-3 GSC 已提交 `https://get.pongrabbit.com/sitemap.xml`（2026-09-22）
- [ ] 0-SEO-3 百度 sitemap 已提交（新站配额常为 0，见 SEO 方案 §7.2；robots.txt 已声明 sitemap）
- [ ] 0-SEO-3 微信分享卡片正常

### 8.3 Phase 0  checklist

- [x] 0-1 本地验收通过（2026-09-22）
- [x] 0-2 文件上传到 ECS `/var/www/publishkit`（2026-09-22）
- [x] 0-3 DNS 生效（get → 120.27.194.105）（2026-09-22）
- [x] 0-4 SSL 证书已签发并下载 Nginx 格式（2026-09-22）
- [x] 0-5 Nginx 配置 get.pongrabbit.cn + reload（2026-09-22）
- [x] 0-6 海外 get.com 可访问（2026-09-22 · 腾讯 ECS + Certbot + CF 橙云）
- [x] 0-7 国内页脚 ICP + 公安备案（HTML 已写入 `zh/index.html`，线上已生效）（2026-09-22）
- [x] 0-8 线上冒烟通过（2026-09-22 · cn/en 双线 + 购买 Pro 占位弹窗）

### 8.3b Phase 1 checklist

- [x] 1-1 Paddle 商品 $38 一次性（`pri_01m33t0kd4jcfehtgxak39ekj3`）
- [x] 1-2 `get.pongrabbit.com` Website approval **Approved**（2026-09-23）
- [x] 1-3 生产 checkout + webhook 发码代码已热更（待真付验邮件）
- [x] 1-4 `site.js` → `app.pongrabbit.com` + Paddle.js overlay（2026-09-23 已部署）
- [x] 1-4b `verify-live.mjs` 通过
- [x] 1-4c 英文站真人验收：邮箱 prompt → Paddle overlay **US$38.00 now**（2026-09-23）
- [x] 1-5 **放弃生产真付与沙箱**（2026-09-23：无测试预算、不愿走 sandbox）。Phase 1 以 Overlay 验收关门；首笔真实 `$38` 由真实买家完成，Webhook 失败再查海外 API 日志

### 8.3c Phase 2 checklist

- [x] 2-1 PC Native **已开通**；经营类目「游戏、在线音视频等虚拟业务」；商家简称「彭兔子的线上店」（2026-09-23）
- [x] 2-2 回调沿用现有 `POST /api/v1/pay/wechat/notify`（不另开 publishkit webhook 路径）
- [x] 2-3 已热更国内 `penrabbit-api`：`POST /api/v1/publishkit/orders` 非法邮箱 400；`wechatPayReady: true`（2026-09-23）
- [x] 2-4 中文站扫码弹层居中，微信付款页 **¥139.00 / 彭兔子PC制版店**（2026-09-23，未付）
- [ ] 2-5 页脚 ICP 已在站上；商户平台隐私 URL 待你核对

### 8.4 问题与结论（操作反馈写这里）

| 日期 | 现象 | 结论/文档更新 |
|------|------|----------------|
| 2026-09-22 | 初版方案落盘 | 自检 S1～S10，Phase 0 先行 |
| 2026-09-22 | Phase 0 双线部署完成 | 国内 ECS Nginx + 阿里云证 · 海外腾讯 ECS + Certbot + CF DNS 橙云 |
| 2026-09-23 | Paddle `get.pongrabbit.com` Approved | 落地页改为 Paddle.js overlay，不再裸开 `_ptxn` 首页 |
| 2026-09-23 | 英文站 Get Pro 弹出 Paddle 窗（US$38） | Overlay 热更验收通过；未付生产单，1-5 仍待真付发码 |
| 2026-09-23 | 不愿生产真付 / 不愿沙箱 | 1-5 放弃。Phase 1 关门，进入 Phase 2 微信 Native |
| 2026-09-23 | 国内热更微信下单 | `pm2 restart penrabbit-api`；补 `cors.js` 放行 `get.pongrabbit.cn`；非法邮箱 400，未创建真实订单 |

### 8.5 文档变更记录

| 版本 | 日期 | 变更 |
|------|------|------|
| v1.2 | 2026-09-22 | Phase 1 热更实踩：海外上线前检查 + `verify-live.mjs` |
| v1.1 | 2026-09-22 | Phase 0-SEO、S11/S12、SEO 专册；SEO 方案 v1.1 竞品调研 |
| v1.0 | 2026-09-22 | 初版：域名 get.*、微信+Paddle、分 Phase |

---

## 10. 推荐操作顺序（总览）

```text
Phase 0-1   本地功能验收（官网验收清单）
    ↓
Phase 0-SEO 改 meta / sitemap / JSON-LD / llms.txt（同一批上传）
    ↓
Phase 0     DNS + SSL + 部署 get.*
    ↓
Phase 0-SEO  GSC / 百度 / 分享卡片
    ↓
Phase 1     Paddle 收银
    ↓
Phase 2     微信支付
```

---

## 11. 你现在要做的第一步

**决策已锁定**（2026-09-22）：Q1–Q3 ✅ · 国内 ECS ✅ · 海外 CF Pages ✅ · ICP/公安备案已写入 `zh/index.html`。

**当前阶段：Phase 2 扫码已验收。** 海外 Paddle Overlay 与国内微信 ¥139 码都已过，两边都不做测试真付。

下一步做 **2-5**：微信商户平台 → 产品中心 → Native 支付 → 产品设置。若有网站或隐私政策栏，填：

- 网站：`https://get.pongrabbit.cn/zh/`
- 隐私：`https://get.pongrabbit.cn/privacy.html`

没有这两栏就跳过，回调不用改。然后做 **0-SEO-3**：把 `https://get.pongrabbit.cn/zh/` 发到微信文件传输助手看分享卡片；百度站长提交 `https://get.pongrabbit.cn/sitemap.xml`。
