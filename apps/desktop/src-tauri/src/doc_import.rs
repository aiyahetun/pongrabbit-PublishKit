use roxmltree::Document;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use zip::ZipArchive;

const DOCX_MIME_PART: &str = "word/document.xml";

pub fn source_format(path: &Path) -> Option<&'static str> {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "md" | "markdown" => Some("md"),
        "txt" => Some("txt"),
        "docx" => Some("docx"),
        "pdf" => Some("pdf"),
        "xlsx" | "xls" => Some("xlsx"),
        "csv" => Some("csv"),
        _ => None,
    }
}

pub fn is_table_format(format: &str) -> bool {
    matches!(format, "csv" | "xlsx" | "xls")
}

pub fn is_split_format(format: &str) -> bool {
    matches!(format, "md" | "txt" | "docx" | "pdf")
}

pub fn is_supported_source(path: &Path) -> bool {
    source_format(path).is_some()
}

pub fn read_source_content(path: &str) -> Result<String, String> {
    let path = PathBuf::from(path);
    if !path.is_file() {
        return Err("文件不存在".into());
    }
    match source_format(&path) {
        Some("docx") => docx_to_markdown(&path),
        Some("pdf") => pdf_to_markdown(&path),
        Some(format) if is_table_format(format) => Err("表格文件请使用表格导入向导".into()),
        Some("md" | "txt" | "markdown") => std::fs::read_to_string(&path).map_err(|e| e.to_string()),
        Some(_) => std::fs::read_to_string(&path).map_err(|e| e.to_string()),
        None => Err("不支持的文件格式".into()),
    }
}

pub fn pdf_to_markdown(path: &Path) -> Result<String, String> {
    let text = pdf_extract::extract_text(path).map_err(|e| format!("PDF 解析失败: {e}"))?;
    let normalized = normalize_pdf_text(&text);
    if normalized.trim().is_empty() {
        return Err("PDF 中没有可提取的文本（可能是扫描件）".into());
    }
    Ok(normalized)
}

fn normalize_pdf_text(text: &str) -> String {
    let paragraphs: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    if paragraphs.is_empty() {
        return String::new();
    }
    paragraphs.join("\n\n") + "\n"
}

pub fn docx_to_markdown(path: &Path) -> Result<String, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
    let mut xml = String::new();
    archive
        .by_name(DOCX_MIME_PART)
        .map_err(|_| "无效的 Word 文档（缺少 document.xml）".to_string())?
        .read_to_string(&mut xml)
        .map_err(|e| e.to_string())?;
    word_xml_to_markdown(&xml)
}

fn word_xml_to_markdown(xml: &str) -> Result<String, String> {
    let doc = Document::parse(xml).map_err(|e| format!("Word XML 解析失败: {e}"))?;
    let mut markdown = String::new();

    for node in doc.descendants().filter(|n| n.tag_name().name() == "p") {
        let text = paragraph_text(node);
        if text.trim().is_empty() {
            continue;
        }
        if let Some(level) = paragraph_style(node).and_then(|style| heading_level(&style)) {
            markdown.push_str(&format!("{} {}\n\n", "#".repeat(level), text.trim()));
        } else {
            markdown.push_str(text.trim());
            markdown.push_str("\n\n");
        }
    }

    if markdown.trim().is_empty() {
        return Err("Word 文档中没有可提取的正文".into());
    }

    Ok(markdown.trim().to_string() + "\n")
}

fn paragraph_text(paragraph: roxmltree::Node<'_, '_>) -> String {
    paragraph
        .descendants()
        .filter(|node| node.tag_name().name() == "t")
        .filter_map(|node| node.text())
        .collect::<String>()
}

const W_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

fn paragraph_style(paragraph: roxmltree::Node<'_, '_>) -> Option<String> {
    paragraph
        .descendants()
        .find(|node| node.tag_name().name() == "pStyle")
        .and_then(|node| node.attribute((W_NS, "val")).map(str::to_string))
}

fn heading_level(style: &str) -> Option<usize> {
    let normalized = style.trim().to_ascii_lowercase();
    if normalized.contains("heading1")
        || normalized == "1"
        || style.contains("标题1")
        || normalized == "title"
    {
        return Some(1);
    }
    if normalized.contains("heading2") || normalized == "2" || style.contains("标题2") {
        return Some(2);
    }
    if normalized.contains("heading3") || normalized == "3" || style.contains("标题3") {
        return Some(3);
    }
    if normalized.contains("heading4") || normalized == "4" || style.contains("标题4") {
        return Some(4);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_heading_style_with_word_namespace() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>Title</w:t></w:r></w:p>
  </w:body>
</w:document>"#;
        let md = word_xml_to_markdown(xml).unwrap();
        assert!(md.starts_with("# Title"));
    }
}
