from pathlib import Path
import csv
import shutil
import zipfile

ROOT = Path(__file__).resolve().parent
PDF_FIXTURE = (
    ROOT.parent.parent
    / "apps/desktop/src-tauri/tests/fixtures/sample.pdf"
)

(ROOT / "sample.md").write_text(
    "# 春季上新主稿\n\n## 小红书\n\n柔软亚麻，轻松叠穿，本周上新。\n\n"
    "## Instagram\n\nSoft linen drop — link in bio.\n",
    encoding="utf-8",
)
(ROOT / "sample.txt").write_text(
    "整篇导入测试：Plain text body for whole-file import.\n",
    encoding="utf-8",
)

with (ROOT / "feishu-export.csv").open("w", encoding="utf-8-sig", newline="") as f:
    writer = csv.writer(f)
    writer.writerow(["标题", "正文", "语言", "渠道"])
    writer.writerow(["小红书春季稿", "柔软亚麻，轻松叠穿，本周上新。", "中文", "小红书"])
    writer.writerow(
        ["Instagram caption", "Soft linen drop — link in bio.", "English", "Instagram"]
    )

shutil.copy2(PDF_FIXTURE, ROOT / "sample.pdf")

document_xml = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>春季上新</w:t></w:r></w:p>
    <w:p><w:r><w:t>Word 导入测试：适合小红书发布的主文案。</w:t></w:r></w:p>
    <w:p><w:pPr><w:pStyle w:val="Heading2"/></w:pPr><w:r><w:t>英文短版</w:t></w:r></w:p>
    <w:p><w:r><w:t>Spring drop is live — soft linen, easy layers.</w:t></w:r></w:p>
  </w:body>
</w:document>"""
content_types = """<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"""
docx_path = ROOT / "sample.docx"
with zipfile.ZipFile(docx_path, "w") as archive:
    archive.writestr("[Content_Types].xml", content_types)
    archive.writestr("word/document.xml", document_xml)

try:
    import openpyxl

    workbook = openpyxl.Workbook()
    sheet = workbook.active
    sheet.append(["标题", "正文", "语言", "渠道"])
    sheet.append(["小红书春季稿", "柔软亚麻，轻松叠穿，本周上新。", "中文", "小红书"])
    sheet.append(
        ["Instagram caption", "Soft linen drop — link in bio.", "English", "Instagram"]
    )
    workbook.save(ROOT / "feishu-export.xlsx")
    print("wrote feishu-export.xlsx")
except ImportError:
    print("openpyxl not installed — skip xlsx (csv still available)")

print("samples ready:", ROOT)
