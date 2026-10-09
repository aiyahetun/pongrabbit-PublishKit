# Chrome Web Store listing (F21)

**Live listing (2026-10):**  
https://chromewebstore.google.com/detail/publishkit-companion/dijkbipanbpkgndbhonecpfnladaiipj  
Extension ID: `dijkbipanbpkgndbhonecpfnladaiipj`

## Upload package

`C:\Users\win\Desktop\PublishKit-extension-0.2.3.zip`  
Root of the zip is `manifest.json` (version 0.2.3). Do not zip the `store/` folder into it.

Privacy policy URL: `https://get.pongrabbit.com/privacy.html`

Developer registration is a one-time **$5** fee if this Google account has never published an extension.

## Store listing (paste)

**Summary (132 max)**  
Copy a draft from the PublishKit desktop app and mark it published. Connects only to localhost.

**Description**  
PublishKit Companion is the browser side panel for the PublishKit desktop app.

It does not post for you and does not store platform passwords. After you pair it with the desktop app, you can copy a draft, fill the current tab URL, and mark the task published. The extension talks only to 127.0.0.1 on your computer.

Install the desktop app first: https://github.com/aiyahetun/pongrabbit-PublishKit/releases/latest

**Permission justifications**  
- storage: save the pairing token and local API port in the browser  
- activeTab: read the active tab URL only when you click “This tab”  
- sidePanel: open the companion panel  
- http://127.0.0.1/*: connect to the PublishKit desktop app on this computer only  

**Privacy form**  
The extension does not send content to PublishKit servers. Pairing data stays in Chrome local storage. Do not say the extension collects or sells user data.

## Short description
Copy a draft from the PublishKit desktop app and mark it published. Connects only to localhost.

## Permissions justification
- `storage`: save pairing token and API base URL
- Host permission `http://127.0.0.1/*`: connect to PublishKit desktop local API only

## Privacy policy URL
Deploy `website/privacy.html` to `https://get.pongrabbit.com/privacy.html`

## Store assets (placeholders)

SVG placeholders live in `apps/extension/store/assets/`:

| File | Size | Slot |
|------|------|------|
| `icon_128.svg` | 128×128 | CWS-01 |
| `promo_small_440x280.svg` | 440×280 | CWS-02 (required) |
| `promo_marquee_1400x560.svg` | 1400×560 | CWS-03 |
| `screenshot_01_sidepanel_1280x800.svg` | 1280×800 | CWS-04 |
| `screenshot_02_copy_1280x800.svg` | 1280×800 | CWS-05 |
| `screenshot_03_publish_1280x800.svg` | 1280×800 | CWS-06 |
| `screenshot_04_duplicate_warn_1280x800.svg` | 1280×800 | CWS-07 |
| `screenshot_05_pairing_1280x800.svg` | 1280×800 | CWS-08 |

Regenerate: `node website/scripts/generate-placeholders.mjs`  
Chrome upload requires **PNG** — export from SVG or replace with AI PNG. Prompts: `PublishKit-官网与Chrome商店配图AI描述词.md`

## Sideload (dev)
1. Open `chrome://extensions`
2. Enable Developer mode
3. Load unpacked → `apps/extension`
