# PublishKit · 海外上线前检查

| 属性 | 内容 |
|------|------|
| 适用 | `get.pongrabbit.com` 静态站 + 海外 `penrabbit-api-1` 收银 |
| 配套脚本 | `node website/scripts/verify-live.mjs` |
| 来源 | 2026-09-22 Phase 1 热更实踩 |

上线或热更前先跑脚本，再按本文「脚本扫不到的」勾一遍。国内 `get.pongrabbit.cn` 静态站可一起扫；国内收银（微信）未上线前，脚本不测微信支付。

---

## 1. 一键扫描（本机）

在仓库根目录：

```powershell
node website/scripts/verify-seo.mjs
node website/scripts/verify-live.mjs
```

`verify-live.mjs` 只打公网 HTTP，**不会**创建 Paddle 真实待支付单（只用非法邮箱，预期 400）。

国内站从海外网络超时可跳过：

```powershell
$env:PUBLISHKIT_SKIP_CN="1"; node website/scripts/verify-live.mjs
```

通过标准：退出码 0，输出 `Live verification passed`。

---

## 2. 脚本会查什么

| # | 检查 | 预期 |
|---|------|------|
| L1 | `https://get.pongrabbit.com/en/` | HTTP 200 |
| L2 | `https://get.pongrabbit.com/assets/site.js` | 含 `pkCheckoutIntl`；海外 `apiBase` 为 `https://app.pongrabbit.com` |
| L3 | 海外 `site.js` **不得**把英文结账打到 `api.pongrabbit.com` | 该主机名无 DNS，浏览器会直接失败 |
| L4 | `https://app.pongrabbit.com/api/v1/health` | 200，`ok=true`，`region=intl`，`dbOk=true` |
| L5 | `POST .../api/v1/publishkit/checkout` 空/非法邮箱 | 400，文案含 email |
| L6 | 同上请求带 `Origin: https://get.pongrabbit.com` | 响应有 `Access-Control-Allow-Origin: https://get.pongrabbit.com` |
| L7 | `https://get.pongrabbit.cn/zh/`（可跳过） | 200 |

官网 200 **不能**代替 L4。`get.*` 是 Nginx 静态目录；收银走 `app.pongrabbit.com` → 容器 `:3000`。

---

## 3. 脚本扫不到、必须人工看的

### 3.1 两条线不要混

| 你看到的 | 实际 | 和 API 容器的关系 |
|----------|------|-------------------|
| `get.pongrabbit.com` | `/var/www/publishkit` | 无关 |
| `app.pongrabbit.com` 页面 | `penrabbit-web-1` | 页面能开也不等于 API 活着 |
| 结账 / 制版下单 | `penrabbit-api-1` `:3000` | **必须** `health` 200 |

判断 API：浏览器打开 `https://app.pongrabbit.com/api/v1/health`，或 SSH `curl -sS http://127.0.0.1:3000/api/v1/health`。

### 3.2 禁止重建容器

现网 API 镜像不完整，业务文件靠 `docker cp` 补在可写层。

| 命令 | 能不能用 |
|------|----------|
| `docker cp` 单个文件进**已有**容器 | 可以 |
| `docker restart penrabbit-api-1`（同一容器） | 可以；PID 1 的 node **必须**用这个才能读到新 JS |
| `docker start`（Exited 之后） | 可以，先确认缺的文件已 cp 进去 |
| `docker compose up` / `up --force-recreate` / `compose build api` | **禁止**（会丢 zipStore、svg-canvas、收银补丁） |
| `ops/intl/reload-api-node.js` | 仅当 PID 1 是 `sleep infinity`、node 是子进程时才有效 |
| `recover-api.sh` / `diagnose-api.sh` | 会 `docker restart`；容器已经 Exited 时先 `docker logs`，不要当第一反应 |

2026-09-22 现网：`penrabbit-api-1` 的 Cmd 是 `npm run migrate:pg && node server.js`，**node 就是 PID 1**。`reload-api-node` 对 PID 1 发 SIGTERM 杀不掉，只会再起一个抢 3000 失败的 node，**热更文件在磁盘上、进程仍是旧代码**。

热更顺序（现网）：

```text
WinSCP/SFTP 写到 /opt/penrabbit/...
  → docker cp 进 penrabbit-api-1
  → 若改了 .env.intl：再 cp 到容器内
       /app/penrabbit-platform/packages/api/.env
     （loadEnv 只补空键，不会覆盖已有 DATABASE_URL）
  → docker restart penrabbit-api-1
  → 等 15～25 秒（migrate + 监听）
  → curl health + 本仓库 verify-live.mjs
```

### 3.3 容器起不来：缺模块，不是「重启一下」

`batchZip.js` / `previewService.js` 在 `server.js` 加载时就会 `require`。缺文件则 **Exited (1)**，`app` 接口 502，`get` 官网仍 200。

历史上出现过：

| 报错路径 | 处理 |
|----------|------|
| `.../pdf-worker/zipStore` | `docker cp` 宿主机 `packages/pdf-worker/zipStore.js` |
| `.../penrabbit-platform/vendor/generatePDF/svg-canvas` | 容器里旧 `previewService.js` 读这个路径；宿主机文件在 `/opt/penrabbit/vendor/generatePDF/`。先 `docker cp` 整个目录树（父目录不存在时，`docker cp` 文件会失败），或把宿主机较新的 `previewService.js` 打进去（它会回退到 `/app/vendor/generatePDF`） |

`docker logs --tail 80 penrabbit-api-1` 看最后一次 `Cannot find module`。只补一个文件后仍 Exited，看下一条缺的模块，不要 `compose up`。

`docker cp` 可以对 **已停止** 的容器写文件；目标父目录必须存在。不存在就先拷一层目录，例如：

```bash
# 宿主机准备 generatePDF 目录后再整夹拷入，才能创建 vendor
docker cp /tmp/pr-vendor-tree/. penrabbit-api-1:/app/penrabbit-platform/vendor
```

### 3.4 收银代码约束

| 项 | 要求 |
|----|------|
| 海外前端 API | `site.js` → `https://app.pongrabbit.com`（Nginx 反代 `/api`） |
| CORS | `packages/api/lib/cors.js` 含 `https://get.pongrabbit.com` |
| 建单 | `orders.user_id` 外键到 `users`。必须 `findOrCreateIntlUser(email)`，禁止写死 `publishkit_guest` |
| 价格 | `.env.intl` 的 `PADDLE_PRICE_PUBLISHKIT_PRO=pri_...` 要进**进程**环境。容器是很久以前 create 的，改宿主机 `.env.intl` 不会自动进 Config.Env |
| 回跳/Overlay | Paddle Checkout 域名白名单要有 `get.pongrabbit.com`（Approved，不是 Pending） |
| Webhook | 仍走现有 `POST /api/v1/webhooks/paddle`，无单独 publishkit webhook 路径 |

验路由不要用真邮箱：`POST {"email":"x"}` 预期 400。真邮箱会在 Paddle **生产**建一笔未支付交易。

### 3.5 静态站热更

`site.js` 只上传 `/var/www/publishkit/assets/site.js` 即可（`/en/`、`/zh/` 引用同一文件）。改完让用户强刷。Cloudflare 橙云若缓存 JS，必要时 Purge `assets/site.js`。

---

## 4. SSH 最短确认（海外机）

容器名：`penrabbit-api-1`。宿主机代码：`/opt/penrabbit/`。

```bash
docker ps -a --filter name=penrabbit-api-1 --format 'table {{.Names}}\t{{.Status}}\t{{.Ports}}'
docker inspect penrabbit-api-1 --format 'Pid1Cmd={{json .Config.Cmd}}'
curl -sS -m 8 http://127.0.0.1:3000/api/v1/health
# 非法邮箱，禁止用真实客户邮箱
curl -sS -m 15 -H 'Content-Type: application/json' \
  -H 'Origin: https://get.pongrabbit.com' \
  -d '{"email":"x"}' \
  http://127.0.0.1:3000/api/v1/publishkit/checkout
```

Exited 时：

```bash
docker logs --tail 80 penrabbit-api-1
# 按模块补文件后 docker start / docker restart 同一容器
```

---

## 5. 上线前总表

- [x] `verify-seo.mjs` 通过
- [x] `verify-live.mjs` 通过
- [x] 未执行 `compose up` / 未重建 `penrabbit-api-1`
- [x] 若刚 `docker cp` 了 API js：已 `docker restart penrabbit-api-1` 且 health 仍 200
- [x] Paddle `get.pongrabbit.com` 为 Approved
- [x] 英文站硬刷新后，Buy Pro 会要邮箱并跳出 Paddle（2026-09-23 真人验收）
- [x] 未把生产 SSH 密码写进仓库、脚本、文档

---

## 6. 记录（补坑）

| 日期 | 现象 | 原因 | 以后怎么防 |
|------|------|------|------------|
| 2026-09-22 | get 能开、app health 502 | `penrabbit-api-1` Exited；缺 `zipStore` 后又缺 `svg-canvas` | L4；先 logs 再 cp，禁止 compose 重建 |
| 2026-09-22 | `reload-api-node` 后收银仍是旧逻辑 | node 为 PID 1，SIGTERM 无效；3000 仍被旧进程占用 | 现网改用同一容器 `docker restart` |
| 2026-09-22 | checkout 400 `orders_user_id_fkey` | `userId: publishkit_guest` 不在 `users` | `findOrCreateIntlUser` |
| 2026-09-22 | 本机/ECS 解析不了 `api.pongrabbit.com` | 子域未配置；现网 API 在 `app.` 反代 | `site.js` 用 `app.pongrabbit.com`；脚本 L3 |
