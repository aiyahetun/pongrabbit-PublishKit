# 导入格式验收样例

将本文件夹设为「来源文件 → 选择文案文件夹」，点击扫描即可验收 v0.2.0 多格式导入。

| 文件 | 操作 | 预期 |
|------|------|------|
| `sample.md` | **拆分** · 智能识别 | 识别「小红书 / Instagram」两个块 |
| `sample.txt` | **拆分** · 整篇 | 导入 1 条 |
| `sample.docx` | **拆分** · 一级标题 | 按 H1 拆成 1 条（含中英文小节） |
| `sample.pdf` | **拆分** · 整篇 | 提取文本并导入 1 条 |
| `feishu-export.csv` | **导入表格** | 2 行，列映射标题/正文 |
| `feishu-export.xlsx` | **导入表格** | 同上（需本机生成时带 openpyxl） |

自动化自检：`cd apps/desktop/src-tauri && cargo test --test import_self_check`
