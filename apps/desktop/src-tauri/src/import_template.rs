//! Downloadable writing templates for schedule import.
//! Markdown is for one piece per heading. The spreadsheet is for one piece per row.

use std::io::{Cursor, Write};

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

pub fn article_template_markdown() -> String {
    r#"# 发稿匣文案模板

## 使用说明

每一节二级标题是一条内容。本节标题里有「说明」，导入时会跳过，不会变成待发任务。

- 发布日期和平台在「排期导入」窗口里填。这份文案不写日期。
- 关联词用顿号或逗号隔开，不要加 #。井号在复制时按平台另加。
- 配图写在同一节的最后：`**配图：** cover.jpg、detail.png`。多个文件名用顿号、逗号或换行隔开。配图后面的行会接着当成文件名，直到下一个字段。这些文件要先出现在当前项目的图片库里。
- 整份文件只有一节、又没写配图时，图片库里主文件名和这份文案相同的图会自动对上，例如 spring.md 对 spring.jpg。一份文件里有多节时，请在每一节写上配图。
- 中英要一起留档的，写在同一节，照第二节示例。导入后拆成两条，并记成一对。同一节的配图两篇都带上。
- 用 Word 时，每一篇设为「标题 2」。字段写成「标题：」「正文：」「关联词：」「配图：」，或带（中文）（英文）。另存为 .docx。旧版 .doc 不读。

## 示例：本地优先的发布台账

**标题：** 本地优先的发布台账

**正文：**
发稿匣把标题、正文和关联词放在一条内容里。你在各平台自己的发布页粘贴。软件不登录账号，也不代发。

**关联词：** 本地优先、发布台账、内容日历

**配图：** cover.jpg、detail.png

## 示例：中英一对

**标题（中文）：** 内容日历，不代发

**正文（中文）：**
同一节里的中文和英文会拆成两条内容，并共用一个配对标记。渠道任务仍挂在各自那一条上。

**关联词（中文）：** 内容日历、跨境内容

**Title:** A content calendar that does not post for you

**Description:**
Chinese and English in one section become two items. They stay linked as a pair.

**Keywords:** content calendar, local-first

**配图：** pair-cover.jpg
"#
    .to_string()
}

pub fn schedule_template_xlsx() -> Result<Vec<u8>, String> {
    let rows = [
        vec![
            "标题".to_string(),
            "正文".to_string(),
            "关联词".to_string(),
            "语言".to_string(),
            "平台".to_string(),
            "发布日期".to_string(),
            "配图".to_string(),
        ],
        vec![
            "本地优先的发布台账".into(),
            "发稿匣把标题、正文和关联词放在一条内容里。你在各平台自己的发布页粘贴。软件不登录账号，也不代发。".into(),
            "本地优先、发布台账、内容日历".into(),
            "中文".into(),
            "小红书".into(),
            "2026-10-10".into(),
            "cover.jpg、detail.png".into(),
        ],
        vec![
            "A content calendar that does not post for you".into(),
            "Leave the date empty when this row should follow the start date and interval in the import window.".into(),
            "content calendar, local-first".into(),
            "English".into(),
            "Pinterest".into(),
            String::new(),
            "pin-cover.jpg".into(),
        ],
        vec![
            "同一篇，两个平台".into(),
            "平台列可以写多个，用顿号或逗号隔开。这一行有自己的日期，就不占用起始日的间隔名额。".into(),
            "发布台账、内容日历".into(),
            "中文".into(),
            "小红书、抖音".into(),
            "2026-10-12".into(),
            String::new(),
        ],
    ];
    xlsx_bytes("排期", &rows)
}

pub fn xlsx_bytes(sheet_name: &str, rows: &[Vec<String>]) -> Result<Vec<u8>, String> {
    let sheet = worksheet_xml(rows);
    let mut cursor = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(&mut cursor);
    let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    write_entry(&mut zip, opts, "[Content_Types].xml", CONTENT_TYPES)?;
    write_entry(&mut zip, opts, "_rels/.rels", ROOT_RELS)?;
    write_entry(&mut zip, opts, "xl/workbook.xml", &workbook_xml(sheet_name))?;
    write_entry(&mut zip, opts, "xl/_rels/workbook.xml.rels", WORKBOOK_RELS)?;
    write_entry(&mut zip, opts, "xl/worksheets/sheet1.xml", &sheet)?;
    zip.finish().map_err(|e| e.to_string())?;
    Ok(cursor.into_inner())
}

fn write_entry<W: Write + std::io::Seek>(
    zip: &mut ZipWriter<W>,
    opts: SimpleFileOptions,
    name: &str,
    contents: &str,
) -> Result<(), String> {
    zip.start_file(name, opts).map_err(|e| e.to_string())?;
    zip.write_all(contents.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

fn worksheet_xml(rows: &[Vec<String>]) -> String {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>"#,
    );
    for (row_idx, row) in rows.iter().enumerate() {
        let row_num = row_idx + 1;
        xml.push_str(&format!(r#"<row r="{row_num}">"#));
        for (col_idx, cell) in row.iter().enumerate() {
            let col = column_name(col_idx);
            xml.push_str(&format!(
                r#"<c r="{col}{row_num}" t="inlineStr"><is><t xml:space="preserve">{}</t></is></c>"#,
                xml_escape(cell)
            ));
        }
        xml.push_str("</row>");
    }
    xml.push_str("</sheetData></worksheet>");
    xml
}

fn column_name(index: usize) -> String {
    let mut n = index + 1;
    let mut out = String::new();
    while n > 0 {
        n -= 1;
        out.insert(0, (b'A' + (n % 26) as u8) as char);
        n /= 26;
    }
    out
}

fn sheet_name(raw: &str) -> String {
    let cleaned: String = raw
        .chars()
        .map(|ch| match ch {
            '\\' | '/' | '?' | '*' | '[' | ']' | ':' => '-',
            _ => ch,
        })
        .collect();
    let cleaned = cleaned.trim().trim_matches('\'').trim();
    let name: String = cleaned.chars().take(31).collect();
    if name.is_empty() {
        "Sheet1".to_string()
    } else {
        name
    }
}

fn workbook_xml(name: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <sheets><sheet name="{}" sheetId="1" r:id="rId1"/></sheets>
</workbook>"#,
        xml_escape(&sheet_name(name))
    )
}

fn xml_escape(value: &str) -> String {
    value
        .chars()
        .filter(|ch| {
            let code = *ch as u32;
            code == 0x9 || code == 0xA || code == 0xD || code >= 0x20
        })
        .collect::<String>()
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

const CONTENT_TYPES: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
</Types>"#;

const ROOT_RELS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>
</Relationships>"#;

const WORKBOOK_RELS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
</Relationships>"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article_template_keeps_examples_and_skips_instructions() {
        let path = std::env::temp_dir().join(format!(
            "publishkit-article-template-{}.md",
            uuid::Uuid::new_v4()
        ));
        std::fs::write(&path, article_template_markdown()).unwrap();
        let preview = crate::batch_import::preview_paths(&[path.to_string_lossy().to_string()]);
        let _ = std::fs::remove_file(&path);
        assert!(preview.errors.is_empty(), "{:?}", preview.errors);
        let titles: Vec<_> = preview.items.iter().map(|item| item.title.as_str()).collect();
        assert_eq!(
            titles,
            vec![
                "本地优先的发布台账",
                "内容日历，不代发",
                "A content calendar that does not post for you",
            ]
        );
        assert!(preview.items.iter().all(|item| !item.title.contains("说明")));
        assert_eq!(
            preview.items[0].keywords,
            vec!["本地优先".to_string(), "发布台账".into(), "内容日历".into()]
        );
        assert!(preview.items[0].body.contains("不登录账号"));
        assert_eq!(preview.items[1].pair_key, preview.items[2].pair_key);
        assert!(preview.items[1].pair_key.is_some());
        assert_eq!(preview.items[2].keywords, vec!["content calendar".to_string(), "local-first".into()]);
        assert_eq!(
            preview.items[0].image_names,
            vec!["cover.jpg".to_string(), "detail.png".into()]
        );
        assert!(!preview.items[0].body.contains("cover.jpg"));
        assert_eq!(preview.items[1].image_names, vec!["pair-cover.jpg".to_string()]);
        assert_eq!(preview.items[2].image_names, preview.items[1].image_names);
    }

    #[test]
    fn schedule_template_reads_back_as_rows() {
        let path = std::env::temp_dir().join(format!(
            "publishkit-schedule-template-{}.xlsx",
            uuid::Uuid::new_v4()
        ));
        std::fs::write(&path, schedule_template_xlsx().unwrap()).unwrap();
        let preview = crate::table_import::preview_table_import(&path.to_string_lossy()).unwrap();
        let _ = std::fs::remove_file(&path);
        assert_eq!(preview.rows.len(), 3);
        assert_eq!(preview.rows[0].title, "本地优先的发布台账");
        assert_eq!(preview.rows[0].scheduled_date.as_deref(), Some("2026-10-10"));
        assert_eq!(preview.rows[0].channel_name.as_deref(), Some("小红书"));
        assert_eq!(preview.rows[0].language, "zh");
        assert_eq!(
            preview.rows[0].keywords,
            vec!["本地优先".to_string(), "发布台账".into(), "内容日历".into()]
        );
        assert!(preview.rows[1].scheduled_date.is_none());
        assert_eq!(preview.rows[1].channel_name.as_deref(), Some("Pinterest"));
        assert_eq!(preview.rows[1].language, "en");
        assert_eq!(preview.rows[2].scheduled_date.as_deref(), Some("2026-10-12"));
        assert_eq!(preview.rows[2].channel_name.as_deref(), Some("小红书、抖音"));
        assert_eq!(
            preview.rows[0].image_names,
            vec!["cover.jpg".to_string(), "detail.png".into()]
        );
        assert_eq!(preview.rows[1].image_names, vec!["pin-cover.jpg".to_string()]);
        assert!(preview.rows[2].image_names.is_empty());
        assert!(!preview.rows[0].body.contains("cover.jpg"));
    }

    #[test]
    fn template_sheet_keeps_markup_and_line_breaks() {
        use calamine::Reader;
        let path = std::env::temp_dir().join(format!(
            "publishkit-sheet-markup-{}.xlsx",
            uuid::Uuid::new_v4()
        ));
        let rows = vec![vec![
            "标题".to_string(),
            "第一行\n第二行 <b> & \"引号\"".into(),
        ]];
        std::fs::write(&path, xlsx_bytes("发布记录", &rows).unwrap()).unwrap();
        let mut workbook: calamine::Xlsx<_> = calamine::open_workbook(&path).unwrap();
        let range = workbook.worksheet_range_at(0).unwrap().unwrap();
        let _ = std::fs::remove_file(&path);
        let cell = range.get((0, 1)).unwrap().to_string();
        assert!(cell.contains("第一行"));
        assert!(cell.contains("第二行"));
        assert!(cell.contains("<b>"));
        assert!(cell.contains('&'));
        assert!(cell.contains("引号"));
    }
}
