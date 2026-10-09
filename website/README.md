# PublishKit Website

Static landing pages for deployment to `get.pongrabbit.com` / `get.pongrabbit.cn`.

## Structure

- `index.html` — redirect to `/zh/` or `/en/` by browser language
- `en/index.html` — English landing
- `zh/index.html` — Chinese landing
- `assets/site.js` — download, checkout, FAQ, Pro modal
- `assets/site.css` — screenshot / hero image styles
- `assets/images/` — marketing screenshots (LP-* SVG placeholders)
- `assets/images/manifest.json` — image slot inventory
- `privacy.html` — bilingual privacy policy
- `scripts/generate-placeholders.mjs` — regenerate SVG placeholders

## Local preview

```powershell
npx --yes serve website
```

Then open `http://localhost:3000/zh/` or `/en/`.

Full acceptance checklist: [`docs/官网验收清单.md`](../docs/官网验收清单.md)

Launch / payment runbook: [`docs/PublishKit-上线联调收银方案.md`](../docs/PublishKit-上线联调收银方案.md)

Overseas preflight (API/Paddle pitfalls + live scan): [`docs/PublishKit-海外上线前检查.md`](../docs/PublishKit-海外上线前检查.md)

SEO / GEO: [`docs/PublishKit-SEO与GEO方案.md`](../docs/PublishKit-SEO与GEO方案.md)

## Placeholder images

Current pages reference SVG placeholders under `assets/images/`. Regenerate:

```powershell
node website/scripts/generate-placeholders.mjs
```

Chrome Web Store listing (live): https://chromewebstore.google.com/detail/publishkit-companion/dijkbipanbpkgndbhonecpfnladaiipj  
Store art sources: `apps/extension/store/assets/`

AI prompt docs: `PublishKit-官网与Chrome商店配图AI描述词.md`

## Before launch

```powershell
node website/scripts/verify-seo.mjs
node website/scripts/verify-live.mjs
```

Edit `website/assets/site.js`:

- `checkoutUrl.en` — Paddle checkout（见上线方案 Phase 1）
- `checkoutUrl.zh` — 微信支付下单页（见上线方案 Phase 2）
- `pricing` — launch ¥139 / $38 (already set)

Replace SVG placeholders with final PNG/WebP using the same filenames (update extensions in HTML if needed).

## Design source

`视觉稿设计/stitch_publishkit_quiet_landing_page/.../publishkit_cross_border_content_ops_landing_page_2/code.html`
