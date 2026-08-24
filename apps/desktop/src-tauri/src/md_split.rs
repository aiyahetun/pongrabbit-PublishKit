use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceAnchor {
    pub start_line: u32,
    pub end_line: u32,
    pub heading: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SectionKind {
    Content,
    Meta,
    Platform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitPreview {
    pub index: usize,
    pub title: String,
    pub heading: Option<String>,
    pub start_line: u32,
    pub end_line: u32,
    pub body_preview: String,
    pub body: String,
    pub language: String,
    pub section_kind: SectionKind,
    /// Whether this block is pre-selected for import (false for meta / ops text).
    pub recommended: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitStrategy {
    Whole,
    H1,
    H2,
    H3,
    Smart,
}

impl SplitStrategy {
    pub fn parse(raw: &str) -> Result<Self, String> {
        match raw {
            "whole" => Ok(Self::Whole),
            "h1" => Ok(Self::H1),
            "h2" => Ok(Self::H2),
            "h3" => Ok(Self::H3),
            "smart" => Ok(Self::Smart),
            // legacy alias from earlier build
            "pattern_no" => Ok(Self::H1),
            _ => Err(format!("未知拆分策略: {raw}")),
        }
    }
}

const META_KEYWORDS: &[&str] = &[
    "怎么用",
    "如何使用",
    "使用说明",
    "使用场景",
    "怎么用这份",
    "速查",
    "发布前",
    "调研",
    "对照",
    "分工",
    "目录",
    "说明",
    "指南",
    "自检",
    "how to use",
    "table of contents",
    "readme",
    "changelog",
    "版本",
    "用途",
    "定位",
    "与旧稿",
    "技术口径",
];

const PLATFORM_KEYWORDS: &[&str] = &[
    "小红书",
    "抖音",
    "视频号",
    "知乎",
    "微博",
    "公众号",
    "instagram",
    "pinterest",
    "reddit",
    "facebook",
    "product hunt",
    "口播",
    "成稿",
    "caption",
    "title",
    "description",
    "english copy",
    "中文文案",
    "短版",
    "钩子",
    "reply",
    "回复调",
    "布料卡",
    "缝纫要点",
];

pub fn preview_splits(path: &str, strategy: SplitStrategy) -> Result<Vec<SplitPreview>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    preview_splits_from_content(&content, strategy)
}

pub fn preview_splits_from_content(
    content: &str,
    strategy: SplitStrategy,
) -> Result<Vec<SplitPreview>, String> {
    let lines: Vec<&str> = content.lines().collect();
    let sections = split_sections(&lines, strategy)?;

    Ok(sections
        .into_iter()
        .enumerate()
        .map(|(index, section)| {
            let body = section.body.join("\n");
            let preview = body.chars().take(180).collect::<String>();
            let language = detect_language(&body);
            let section_kind = classify_section(&section.title, section.heading.as_deref());
            let recommended = section_kind != SectionKind::Meta;
            SplitPreview {
                index,
                title: section.title,
                heading: section.heading,
                start_line: section.start_line,
                end_line: section.end_line,
                body_preview: if body.chars().count() > 180 {
                    format!("{preview}…")
                } else {
                    preview
                },
                body,
                language,
                section_kind,
                recommended,
            }
        })
        .collect())
}

struct Section {
    title: String,
    heading: Option<String>,
    start_line: u32,
    end_line: u32,
    body: Vec<String>,
}

fn split_sections(lines: &[&str], strategy: SplitStrategy) -> Result<Vec<Section>, String> {
    match strategy {
        SplitStrategy::Whole => Ok(vec![whole_section(lines)]),
        SplitStrategy::H1 => split_by_heading_level(lines, 1),
        SplitStrategy::H2 => split_by_heading_level(lines, 2),
        SplitStrategy::H3 => split_by_heading_level(lines, 3),
        SplitStrategy::Smart => smart_split(lines),
    }
}

fn whole_section(lines: &[&str]) -> Section {
    let title = document_title(lines);
    Section {
        title,
        heading: None,
        start_line: 1,
        end_line: lines.len().max(1) as u32,
        body: lines.iter().map(|s| (*s).to_string()).collect(),
    }
}

fn document_title(lines: &[&str]) -> String {
    for line in lines.iter().take(30) {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("# ") {
            if !rest.is_empty() {
                return rest.to_string();
            }
        }
    }
    "Untitled".to_string()
}

fn split_by_heading_level(lines: &[&str], level: u8) -> Result<Vec<Section>, String> {
    let pattern = match level {
        1 => r"(?m)^#\s+(.+)$",
        2 => r"(?m)^##\s+(.+)$",
        3 => r"(?m)^###\s+(.+)$",
        _ => return Err("unsupported heading level".into()),
    };
    split_by_regex(lines, pattern, |cap| cap[1].trim().to_string())
}

fn smart_split(lines: &[&str]) -> Result<Vec<Section>, String> {
    let h3 = split_by_heading_level(lines, 3)?;
    let platform_sections: Vec<Section> = h3
        .into_iter()
        .filter(|s| classify_section(&s.title, s.heading.as_deref()) == SectionKind::Platform)
        .collect();
    if platform_sections.len() >= 2 {
        return Ok(platform_sections);
    }

    let h2 = split_by_heading_level(lines, 2)?;
    let content_h2: Vec<Section> = h2
        .into_iter()
        .filter(|s| classify_section(&s.title, s.heading.as_deref()) != SectionKind::Meta)
        .collect();
    if content_h2.len() >= 2 {
        return Ok(content_h2);
    }

    let h1 = split_by_heading_level(lines, 1)?;
    let content_h1: Vec<Section> = h1
        .into_iter()
        .filter(|s| {
            let kind = classify_section(&s.title, s.heading.as_deref());
            kind == SectionKind::Content || kind == SectionKind::Platform
        })
        .collect();
    if !content_h1.is_empty() {
        return Ok(content_h1);
    }

    Ok(vec![whole_section(lines)])
}

fn split_by_regex<F>(lines: &[&str], pattern: &str, title_from: F) -> Result<Vec<Section>, String>
where
    F: Fn(&regex::Captures) -> String,
{
    let re = Regex::new(pattern).map_err(|e| e.to_string())?;
    let mut headings: Vec<(usize, String, String)> = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        if let Some(cap) = re.captures(line) {
            headings.push((idx, line.trim().to_string(), title_from(&cap)));
        }
    }

    if headings.is_empty() {
        return Ok(vec![whole_section(lines)]);
    }

    let mut sections = Vec::new();
    for (i, (start_idx, heading_line, title)) in headings.iter().enumerate() {
        let end_idx = headings
            .get(i + 1)
            .map(|(idx, _, _)| idx.saturating_sub(1))
            .unwrap_or(lines.len().saturating_sub(1));

        sections.push(Section {
            title: title.clone(),
            heading: Some(heading_line.clone()),
            start_line: (*start_idx + 1) as u32,
            end_line: (end_idx + 1) as u32,
            body: lines[*start_idx..=end_idx]
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
        });
    }

    Ok(sections)
}

pub fn classify_section(title: &str, heading: Option<&str>) -> SectionKind {
    let haystack = format!(
        "{} {}",
        title.to_lowercase(),
        heading.unwrap_or("").to_lowercase()
    );

    if META_KEYWORDS
        .iter()
        .any(|k| haystack.contains(&k.to_lowercase()))
    {
        return SectionKind::Meta;
    }

    if PLATFORM_KEYWORDS
        .iter()
        .any(|k| haystack.contains(&k.to_lowercase()))
    {
        return SectionKind::Platform;
    }

    SectionKind::Content
}

pub fn detect_language(text: &str) -> String {
    let mut cjk = 0usize;
    let mut latin = 0usize;

    for ch in text.chars() {
        if ('\u{4e00}'..='\u{9fff}').contains(&ch) {
            cjk += 1;
        } else if ch.is_ascii_alphabetic() {
            latin += 1;
        }
    }

    if cjk > 40 && latin > 40 {
        "bilingual".into()
    } else if cjk >= latin {
        "zh".into()
    } else {
        "en".into()
    }
}

pub fn anchor_json(anchor: &SourceAnchor) -> Result<String, String> {
    serde_json::to_string(anchor).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h2_split_finds_angle_sections() {
        let text = "# Root\n\n## Angle one\nBody A\n\n## Angle two\nBody B\n";
        let lines: Vec<&str> = text.lines().collect();
        let sections = split_by_heading_level(&lines, 2).unwrap();
        assert_eq!(sections.len(), 2);
    }

    #[test]
    fn meta_section_is_not_recommended() {
        let previews = preview_splits_from_content(
            "# Doc\n\n## 0. 怎么用这份稿\n说明\n\n## 角度一\n正文\n",
            SplitStrategy::H2,
        )
        .unwrap();
        assert_eq!(previews.len(), 2);
        assert!(!previews[0].recommended);
        assert!(previews[1].recommended);
    }

    #[test]
    fn smart_prefers_platform_h3() {
        let text = "# Root\n\n## 01 Bib\n### 小红书成稿\nXHS body\n\n### 抖音口播\nDY body\n";
        let previews = preview_splits_from_content(text, SplitStrategy::Smart).unwrap();
        assert_eq!(previews.len(), 2);
        assert!(previews.iter().all(|p| p.section_kind == SectionKind::Platform));
    }
}
