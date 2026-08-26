# PublishKit Browser Extension (MVP)

Chrome/Edge MV3 companion for the desktop app's localhost API.

## Load unpacked

1. Start desktop app: `npm run tauri dev`
2. Open **Settings → Browser extension** and copy **port + pairing token**
3. Chrome → `chrome://extensions` → Developer mode → **Load unpacked**
4. Select this folder: `apps/extension`
5. Open extension popup → paste port/token → **Test connection**

## Workflow

1. **Test connection** — verifies `/health` and syncs UI locale from desktop
2. **Load today's tasks** — lists `ready` tasks from desktop
3. Per task:
   - **Copy body** — `POST /tasks/{id}/prepare` → clipboard
   - **This tab** — fills URL from active browser tab
   - **Mark published** — `POST /tasks/{id}/publish` with URL → desktop updates instantly

## API endpoints used

- `GET /health`
- `GET /settings/ui-locale`
- `GET /tasks/today`
- `POST /tasks/{id}/prepare`
- `POST /tasks/{id}/publish`

See `docs/技术标准-PublishKit.md` §5 for full API spec.
