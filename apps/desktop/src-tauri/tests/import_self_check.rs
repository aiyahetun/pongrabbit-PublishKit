//! Self-check for v0.2.0 multi-format import (no external fixture files required).

use publishkit_desktop_lib::doc_import;
use publishkit_desktop_lib::md_split::{preview_splits, SplitStrategy};
use publishkit_desktop_lib::table_import;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

fn temp_fixture_dir() -> PathBuf {
    let dir = std::env::temp_dir().join("publishkit-import-self-check");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn write_text(path: &Path, content: &str) {
    fs::write(path, content).expect("write text");
}

fn write_min_docx(path: &Path) {
    let file = File::create(path).expect("create docx");
    let mut zip = ZipWriter::new(file);
    let opts = SimpleFileOptions::default();
    zip.start_file("[Content_Types].xml", opts)
        .expect("zip content types");
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#,
    )
    .expect("write content types");
    zip.start_file("word/document.xml", opts)
        .expect("zip document");
    let document_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>春季上新</w:t></w:r></w:p>
    <w:p><w:r><w:t>这是春季上新的主文案，适合小红书发布。</w:t></w:r></w:p>
    <w:p><w:pPr><w:pStyle w:val="Heading2"/></w:pPr><w:r><w:t>英文短版</w:t></w:r></w:p>
    <w:p><w:r><w:t>Spring drop is live — soft linen, easy layers.</w:t></w:r></w:p>
  </w:body>
</w:document>"#;
    zip.write_all(document_xml.as_bytes())
        .expect("write document xml");
    zip.finish().expect("finish docx zip");
}

fn copy_fixture(name: &str, dest: &Path) {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    fs::copy(src, dest).unwrap_or_else(|e| panic!("copy fixture {name}: {e}"));
}

fn write_min_xlsx(path: &Path) {
    let file = File::create(path).expect("create xlsx");
    let mut zip = ZipWriter::new(file);
    let opts = SimpleFileOptions::default();

    zip.start_file("[Content_Types].xml", opts).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
  <Override PartName="/xl/sharedStrings.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"/>
</Types>"#,
    )
    .unwrap();

    zip.start_file("_rels/.rels", opts).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>
</Relationships>"#,
    )
    .unwrap();

    zip.start_file("xl/_rels/workbook.xml.rels", opts).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings" Target="sharedStrings.xml"/>
</Relationships>"#,
    )
    .unwrap();

    zip.start_file("xl/workbook.xml", opts).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <sheets><sheet name="Sheet1" sheetId="1" r:id="rId1"/></sheets>
</workbook>"#,
    )
    .unwrap();

    zip.start_file("xl/sharedStrings.xml", opts).unwrap();
    let shared_strings = r#"<?xml version="1.0" encoding="UTF-8"?>
<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="12" uniqueCount="12">
  <si><t>标题</t></si><si><t>正文</t></si><si><t>语言</t></si><si><t>渠道</t></si>
  <si><t>小红书春季稿</t></si><si><t>柔软亚麻，轻松叠穿，本周上新。</t></si><si><t>中文</t></si><si><t>小红书</t></si>
  <si><t>Instagram caption</t></si><si><t>Soft linen drop — link in bio.</t></si><si><t>English</t></si><si><t>Instagram</t></si>
</sst>"#;
    zip.write_all(shared_strings.as_bytes()).unwrap();

    zip.start_file("xl/worksheets/sheet1.xml", opts).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8"?>
<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <sheetData>
    <row r="1">
      <c r="A1" t="s"><v>0</v></c><c r="B1" t="s"><v>1</v></c><c r="C1" t="s"><v>2</v></c><c r="D1" t="s"><v>3</v></c>
    </row>
    <row r="2">
      <c r="A2" t="s"><v>4</v></c><c r="B2" t="s"><v>5</v></c><c r="C2" t="s"><v>6</v></c><c r="D2" t="s"><v>7</v></c>
    </row>
    <row r="3">
      <c r="A3" t="s"><v>8</v></c><c r="B3" t="s"><v>9</v></c><c r="C3" t="s"><v>10</v></c><c r="D3" t="s"><v>11</v></c>
    </row>
  </sheetData>
</worksheet>"#,
    )
    .unwrap();

    zip.finish().expect("finish xlsx");
}

#[test]
fn self_check_all_import_formats() {
    let dir = temp_fixture_dir();

    let md_path = dir.join("sample.md");
    write_text(
        &md_path,
        "# 主标题\n\n## 小红书\n\n中文种草文案第一段。\n\n## Instagram\n\nEnglish caption for spring drop.\n",
    );

    let txt_path = dir.join("sample.txt");
    write_text(&txt_path, "Plain text article body for whole-file import.\n");

    let docx_path = dir.join("sample.docx");
    write_min_docx(&docx_path);

    let pdf_path = dir.join("sample.pdf");
    copy_fixture("sample.pdf", &pdf_path);

    let csv_path = dir.join("feishu-export.csv");
    write_text(
        &csv_path,
        "标题,正文,语言,渠道\n\
小红书春季稿,柔软亚麻轻松叠穿,中文,小红书\n\
Instagram caption,Soft linen drop link in bio,English,Instagram\n",
    );

    let xlsx_path = dir.join("feishu-export.xlsx");
    write_min_xlsx(&xlsx_path);

    // --- format detection ---
    assert_eq!(doc_import::source_format(&md_path), Some("md"));
    assert_eq!(doc_import::source_format(&txt_path), Some("txt"));
    assert_eq!(doc_import::source_format(&docx_path), Some("docx"));
    assert_eq!(doc_import::source_format(&pdf_path), Some("pdf"));
    assert_eq!(doc_import::source_format(&csv_path), Some("csv"));
    assert_eq!(doc_import::source_format(&xlsx_path), Some("xlsx"));
    assert!(doc_import::is_table_format("csv"));
    assert!(doc_import::is_table_format("xlsx"));
    assert!(doc_import::is_split_format("pdf"));
    assert!(doc_import::is_split_format("docx"));

    // --- read_source_content ---
    let md_content = doc_import::read_source_content(md_path.to_str().unwrap()).expect("md");
    assert!(md_content.contains("# 主标题"));

    let docx_content =
        doc_import::read_source_content(docx_path.to_str().unwrap()).expect("docx");
    assert!(docx_content.contains("# 春季上新"));
    assert!(docx_content.contains("Spring drop"));

    let pdf_content = doc_import::read_source_content(pdf_path.to_str().unwrap()).expect("pdf");
    assert!(
        pdf_content.to_lowercase().contains("publishkit"),
        "pdf text: {pdf_content}"
    );

    let table_err = doc_import::read_source_content(csv_path.to_str().unwrap());
    assert!(
        table_err.is_err(),
        "csv should route to table wizard, got: {table_err:?}"
    );

    // --- split previews ---
    let md_splits =
        preview_splits(md_path.to_str().unwrap(), SplitStrategy::Smart).expect("md splits");
    assert!(
        md_splits.iter().any(|p| p.title.contains("小红书") || p.body.contains("种草")),
        "md smart split: {:?}",
        md_splits.iter().map(|p| &p.title).collect::<Vec<_>>()
    );

    let docx_splits =
        preview_splits(docx_path.to_str().unwrap(), SplitStrategy::H1).expect("docx splits");
    assert_eq!(docx_splits.len(), 1, "docx h1: {docx_splits:?}");
    assert!(docx_splits[0].body.contains("春季上新"));

    let pdf_splits =
        preview_splits(pdf_path.to_str().unwrap(), SplitStrategy::Whole).expect("pdf splits");
    assert_eq!(pdf_splits.len(), 1);
    assert!(pdf_splits[0].body.to_lowercase().contains("publishkit"));

    let txt_splits =
        preview_splits(txt_path.to_str().unwrap(), SplitStrategy::Whole).expect("txt splits");
    assert_eq!(txt_splits.len(), 1);

    // --- table import ---
    let csv_preview =
        table_import::preview_table_import(csv_path.to_str().unwrap()).expect("csv preview");
    assert_eq!(csv_preview.title_column, "标题");
    assert_eq!(csv_preview.body_column, "正文");
    assert_eq!(csv_preview.rows.len(), 2);
    assert!(csv_preview.rows[0].title.contains("小红书"));
    assert_eq!(
        csv_preview.rows[0].channel_name.as_deref(),
        Some("小红书")
    );

    let xlsx_preview =
        table_import::preview_table_import(xlsx_path.to_str().unwrap()).expect("xlsx preview");
    assert_eq!(xlsx_preview.rows.len(), 2, "xlsx rows: {xlsx_preview:?}");
    assert_eq!(xlsx_preview.title_column, "标题");

    let selected = table_import::selected_table_rows(csv_path.to_str().unwrap(), &[0]).expect("select");
    assert_eq!(selected.len(), 1);
}
