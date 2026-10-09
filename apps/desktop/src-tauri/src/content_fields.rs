//! Split one source block into title / body / keywords.
//! A block that contains both Chinese and English labeled fields becomes two pieces.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedPiece {
    pub language: String,
    pub title: String,
    pub body: String,
    pub keywords: Vec<String>,
    pub image_names: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FieldKind {
    Title,
    Body,
    Keywords,
    Images,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LangHint {
    Zh,
    En,
}

struct Segment {
    kind: FieldKind,
    lang: LangHint,
    text: String,
}

pub fn parse_block(fallback_title: &str, raw_body: &str) -> Vec<ParsedPiece> {
    let lines: Vec<&str> = raw_body.lines().collect();
    let mut segments: Vec<Segment> = Vec::new();
    let mut current: Option<Segment> = None;
    let mut unlabeled: Vec<String> = Vec::new();
    let mut saw_label = false;
    let mut image_names: Vec<String> = Vec::new();

    for line in &lines {
        if let Some((kind, lang, inline)) = match_label_line(line) {
            saw_label = true;
            if let Some(seg) = current.take() {
                segments.push(seg);
            }
            current = Some(Segment {
                kind,
                lang,
                text: inline,
            });
        } else if let Some(seg) = current.as_mut() {
            if !seg.text.is_empty() {
                seg.text.push('\n');
            }
            seg.text.push_str(line);
        } else {
            unlabeled.push((*line).to_string());
        }
    }
    if let Some(seg) = current.take() {
        segments.push(seg);
    }

    if !saw_label {
        return vec![unlabeled_piece(fallback_title, raw_body)];
    }

    let mut zh = PieceBuf::default();
    let mut en = PieceBuf::default();
    for seg in segments {
        let buf = match seg.lang {
            LangHint::Zh => &mut zh,
            LangHint::En => &mut en,
        };
        let text = seg.text.trim().to_string();
        match seg.kind {
            FieldKind::Title => buf.title = text,
            FieldKind::Body => buf.body = text,
            FieldKind::Keywords => buf.keywords = split_keyword_text(&text),
            FieldKind::Images => extend_image_names(&mut image_names, &text),
        }
    }

    // A same-line keyword label has no 正文 field. The prose above it is the body.
    // When a 正文 label exists, leave the preamble alone so a "do not paste" note stays out.
    if zh.body.trim().is_empty() && en.body.trim().is_empty() {
        let preamble = strip_duplicate_heading(&unlabeled.join("\n"), fallback_title);
        if !preamble.is_empty() {
            if zh.has_content() {
                zh.body = preamble;
            } else if en.has_content() {
                en.body = preamble;
            }
        }
    }

    let mut pieces = Vec::new();
    if zh.has_content() {
        pieces.push(zh.into_piece("zh", fallback_title));
    }
    if en.has_content() {
        pieces.push(en.into_piece("en", fallback_title));
    }
    if pieces.is_empty() {
        pieces.push(unlabeled_piece(fallback_title, &unlabeled.join("\n")));
    }
    for piece in &mut pieces {
        piece.image_names = image_names.clone();
    }
    pieces
}

pub fn split_keyword_text(raw: &str) -> Vec<String> {
    let cleaned = raw.replace('`', "");
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    let parts: Vec<String> = if trimmed.contains('·')
        || trimmed.contains('|')
        || trimmed.contains('、')
        || trimmed.contains('，')
        || trimmed.contains(',')
        || trimmed.contains(';')
        || trimmed.contains('；')
    {
        trimmed
            .split(['·', '|', '、', '，', ',', ';', '；', '\n'])
            .map(clean_keyword)
            .filter(|part| !part.is_empty())
            .collect()
    } else if trimmed.contains('#') {
        trimmed
            .split('#')
            .map(clean_keyword)
            .filter(|part| !part.is_empty())
            .collect()
    } else {
        trimmed
            .lines()
            .map(clean_keyword)
            .filter(|part| !part.is_empty())
            .collect()
    };

    let mut out = Vec::new();
    for part in parts {
        if !out.iter().any(|existing: &String| existing == &part) {
            out.push(part);
        }
    }
    out
}

/// Filenames stay whole. Spaces, middle dots, and hashes are part of the name.
pub fn split_image_names(raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    extend_image_names(&mut out, raw);
    out
}

fn extend_image_names(out: &mut Vec<String>, raw: &str) {
    for part in raw.split(['、', '，', ',', ';', '；', '|', '\n']) {
        let name = part
            .trim()
            .trim_matches(|ch: char| {
                matches!(ch, '*' | '_' | '`' | '"' | '\'' | '“' | '”')
            })
            .trim()
            .to_string();
        if name.is_empty() {
            continue;
        }
        if out.iter().any(|existing| existing.eq_ignore_ascii_case(&name)) {
            continue;
        }
        out.push(name);
    }
}

fn clean_keyword(raw: &str) -> String {
    raw.trim()
        .trim_matches(|c: char| {
            c == '#' || c == '`' || c == '"' || c == '\'' || c == '“' || c == '”' || c == '*' || c == '_'
        })
        .trim()
        .to_string()
}

fn unlabeled_piece(fallback_title: &str, raw_body: &str) -> ParsedPiece {
    let mut body_lines: Vec<String> = Vec::new();
    let mut keywords = Vec::new();
    for line in raw_body.lines() {
        if let Some((kind, _, inline)) = match_label_line(line) {
            if kind == FieldKind::Keywords {
                keywords.extend(split_keyword_text(&inline));
                continue;
            }
            if kind == FieldKind::Images {
                continue;
            }
        }
        body_lines.push(line.to_string());
    }
    let body = strip_duplicate_heading(&body_lines.join("\n"), fallback_title);
    ParsedPiece {
        language: String::new(),
        title: fallback_title.trim().to_string(),
        body,
        keywords,
        image_names: Vec::new(),
    }
}

fn strip_duplicate_heading(body: &str, title: &str) -> String {
    let title = title.trim();
    let mut lines = body.lines();
    if let Some(first) = lines.next() {
        let stripped = first.trim().trim_start_matches('#').trim();
        if !title.is_empty() && stripped == title {
            return lines.collect::<Vec<_>>().join("\n").trim().to_string();
        }
    }
    body.trim().to_string()
}

#[derive(Default)]
struct PieceBuf {
    title: String,
    body: String,
    keywords: Vec<String>,
}

impl PieceBuf {
    fn has_content(&self) -> bool {
        !self.title.trim().is_empty() || !self.body.trim().is_empty() || !self.keywords.is_empty()
    }

    fn into_piece(self, language: &str, fallback_title: &str) -> ParsedPiece {
        let title = if self.title.trim().is_empty() {
            fallback_title.trim().to_string()
        } else {
            self.title.trim().to_string()
        };
        ParsedPiece {
            language: language.to_string(),
            title,
            body: self.body.trim().to_string(),
            keywords: self.keywords,
            image_names: Vec::new(),
        }
    }
}

fn match_label_line(line: &str) -> Option<(FieldKind, LangHint, String)> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let stripped = strip_emphasis(trimmed);
    parse_label(&stripped)
}

fn strip_emphasis(line: &str) -> String {
    let mut text = line.trim();
    loop {
        let next = text
            .strip_prefix("**")
            .and_then(|rest| rest.strip_suffix("**"))
            .or_else(|| text.strip_prefix('*').and_then(|rest| rest.strip_suffix('*')))
            .or_else(|| text.strip_prefix('_').and_then(|rest| rest.strip_suffix('_')));
        match next {
            Some(inner) if inner != text => text = inner.trim(),
            _ => break,
        }
    }
    text.to_string()
}

fn parse_label(stripped: &str) -> Option<(FieldKind, LangHint, String)> {
    let (head, rest, has_colon) = split_label_head(stripped)?;
    // `**关联词：** value` only wraps the label, so the stars sit on the head and the rest.
    let head = head.trim().trim_matches(|ch: char| ch == '*' || ch == '_');
    let (word, lang_from_suffix) = split_lang_suffix(head.trim())?;
    let (kind, default_lang) = kind_of_word(word)?;
    if !has_colon && !rest.trim().is_empty() {
        return None;
    }
    let lang = lang_from_suffix.unwrap_or(default_lang);
    let rest = rest
        .trim()
        .trim_matches(|ch: char| ch == '*' || ch == '_')
        .trim();
    Some((kind, lang, rest.to_string()))
}

fn split_label_head(stripped: &str) -> Option<(&str, &str, bool)> {
    if let Some((head, rest)) = stripped.split_once('：') {
        return Some((head, rest, true));
    }
    if let Some((head, rest)) = stripped.split_once(':') {
        return Some((head, rest, true));
    }
    Some((stripped, "", false))
}

fn split_lang_suffix(head: &str) -> Option<(&str, Option<LangHint>)> {
    let head = head.trim();
    if head.is_empty() {
        return None;
    }
    let (_, last) = head.char_indices().next_back()?;
    if last != ')' && last != '）' {
        return Some((head, None));
    }
    let open = head.rfind(['(', '（'])?;
    let word = head[..open].trim();
    if word.is_empty() {
        return None;
    }
    let open_len = head[open..].chars().next()?.len_utf8();
    let inner_start = open + open_len;
    let inner_end = head.len() - last.len_utf8();
    if inner_start > inner_end {
        return Some((word, None));
    }
    let mark = head[inner_start..inner_end].trim();
    let lang = match mark.to_ascii_lowercase().as_str() {
        "中文" | "zh" | "cn" => Some(LangHint::Zh),
        "英文" | "en" | "english" => Some(LangHint::En),
        _ => None,
    };
    Some((word, lang))
}

fn kind_of_word(word: &str) -> Option<(FieldKind, LangHint)> {
    match word.trim().to_ascii_lowercase().as_str() {
        "标题" => Some((FieldKind::Title, LangHint::Zh)),
        "正文" | "文案" => Some((FieldKind::Body, LangHint::Zh)),
        "关联词" | "关键词" | "标签" | "话题" => Some((FieldKind::Keywords, LangHint::Zh)),
        "title" => Some((FieldKind::Title, LangHint::En)),
        "description" | "body" | "caption" => Some((FieldKind::Body, LangHint::En)),
        "keywords" | "keyword" | "tags" | "hashtags" => Some((FieldKind::Keywords, LangHint::En)),
        "配图" | "图片" => Some((FieldKind::Images, LangHint::Zh)),
        "images" | "image" | "media" => Some((FieldKind::Images, LangHint::En)),
        _ => None,
    }
}

pub fn is_multi_field_channel(channel_id: &str) -> bool {
    matches!(channel_id, "xhs" | "pinterest" | "wechat_mp" | "youtube")
}

/// Single-box composers and custom channels append keywords to the body.
/// No channel means the content library, which copies the body by itself.
pub fn appends_keywords_to_body(channel_id: Option<&str>) -> bool {
    match channel_id {
        None => false,
        Some(id) => !is_multi_field_channel(id),
    }
}

pub fn default_keyword_hash(channel_id: Option<&str>) -> bool {
    matches!(
        channel_id,
        Some(
            "xhs" | "douyin" | "kuaishou" | "channels" | "bilibili" | "instagram" | "tiktok" | "threads"
                | "weibo" | "toutiao"
        )
    )
}

pub fn format_keywords(channel_id: Option<&str>, keywords: &[String]) -> String {
    format_keywords_with_hash(channel_id, keywords, default_keyword_hash(channel_id))
}

pub fn format_keywords_with_hash(
    channel_id: Option<&str>,
    keywords: &[String],
    hash: bool,
) -> String {
    let words: Vec<&str> = keywords.iter().map(|word| word.trim()).filter(|word| !word.is_empty()).collect();
    if words.is_empty() {
        return String::new();
    }
    if hash {
        return words
            .into_iter()
            .map(|word| {
                let bare = word.trim_start_matches('#');
                format!("#{bare}")
            })
            .collect::<Vec<_>>()
            .join(" ");
    }
    match channel_id {
        Some("pinterest") => words.join(", "),
        Some(_) => join_plain(&words),
        None => words.join(" "),
    }
}

fn join_plain(words: &[&str]) -> String {
    let cjk = words.iter().any(|word| word.chars().any(|ch| ('\u{4e00}'..='\u{9fff}').contains(&ch)));
    if cjk {
        words.join("、")
    } else {
        words.join(", ")
    }
}

/// Title, body, and keywords as one plain block. Body is the stored text, not the single-box copy that already appends keywords.
pub fn full_text_for_copy(channel_id: Option<&str>, title: &str, body: &str, keywords: &[String]) -> String {
    let title = title.trim();
    let body = crate::rich_text::markdown_to_plain(body.trim());
    let body = body.trim();
    let formatted = format_keywords(channel_id, keywords);
    full_text_from_parts(title, body, &formatted)
}

pub fn full_text_for_copy_with_hash(
    channel_id: Option<&str>,
    title: &str,
    body: &str,
    keywords: &[String],
    hash: bool,
) -> String {
    let title = title.trim();
    let body = crate::rich_text::markdown_to_plain(body.trim());
    let body = body.trim();
    let formatted = format_keywords_with_hash(channel_id, keywords, hash);
    full_text_from_parts(title, body, &formatted)
}

fn full_text_from_parts<'a>(title: &'a str, body: &'a str, formatted: &'a str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    if !title.is_empty() {
        parts.push(title);
    }
    if !body.is_empty() {
        parts.push(body);
    }
    if !formatted.is_empty() {
        parts.push(formatted);
    }
    parts.join("\n\n")
}

pub fn body_for_channel_copy(channel_id: Option<&str>, body: &str, keywords: &[String]) -> String {
    body_for_channel_copy_with_hash(channel_id, body, keywords, default_keyword_hash(channel_id))
}

pub fn body_for_channel_copy_with_hash(
    channel_id: Option<&str>,
    body: &str,
    keywords: &[String],
    hash: bool,
) -> String {
    let body = body.trim();
    if !appends_keywords_to_body(channel_id) {
        return body.to_string();
    }
    let formatted = format_keywords_with_hash(channel_id, keywords, hash);
    if formatted.is_empty() {
        return body.to_string();
    }
    if body.is_empty() {
        formatted
    } else {
        format!("{body}\n\n{formatted}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinterest_block_splits_zh_and_en() {
        let raw = "\
**标题（中文）**\n\
发稿匣：文案和状态放在同一台电脑\n\
\n\
**正文（中文）**\n\
桌面端记发布状态。\n\
\n\
**关联词（中文）**\n\
内容资产管理 · 发布台账 · 本地优先\n\
\n\
**Title (EN)**\n\
PublishKit on your PC\n\
\n\
**Description (EN)**\n\
Local ledger, you still publish yourself.\n\
\n\
**Keywords (EN)**\n\
content asset management · local-first · publish tracker\n";
        let pieces = parse_block("publishkit_hero.png", raw);
        assert_eq!(pieces.len(), 2);
        assert_eq!(pieces[0].language, "zh");
        assert_eq!(pieces[0].title, "发稿匣：文案和状态放在同一台电脑");
        assert_eq!(pieces[0].body, "桌面端记发布状态。");
        assert_eq!(
            pieces[0].keywords,
            vec!["内容资产管理", "发布台账", "本地优先"]
        );
        assert_eq!(pieces[1].language, "en");
        assert_eq!(pieces[1].title, "PublishKit on your PC");
        assert!(pieces[1].body.contains("Local ledger"));
        assert_eq!(pieces[1].keywords[0], "content asset management");
        assert!(!pieces[0].body.contains("关联词"));
        assert!(!pieces[1].body.contains("Keywords"));
    }

    #[test]
    fn same_line_keywords_leave_the_body() {
        let raw = "正文第一句。\n\n**关联词：** `local-first` · `content calendar`";
        let pieces = parse_block("本地台账", raw);
        assert_eq!(pieces.len(), 1);
        assert_eq!(pieces[0].title, "本地台账");
        assert_eq!(pieces[0].keywords, vec!["local-first", "content calendar"]);
        assert!(!pieces[0].body.contains("关联词"));
        assert!(pieces[0].body.contains("正文第一句"));
    }

    #[test]
    fn a_sentence_that_starts_with_title_is_not_a_label() {
        let raw = "标题写在句首，后面是正文，没有冒号。";
        let pieces = parse_block("备用标题", raw);
        assert_eq!(pieces.len(), 1);
        assert_eq!(pieces[0].title, "备用标题");
        assert!(pieces[0].body.contains("标题写在句首"));
        assert!(pieces[0].keywords.is_empty());
    }

    #[test]
    fn section_images_stay_out_of_the_body() {
        let raw = "\
**标题（中文）：** 春季上新\n\
**正文（中文）：**\n\
橱窗换了。\n\
**配图：** my cover.jpg、detail.png\n\
notes/extra.jpg\n\
**关联词（中文）：** 春季、上新\n\
**Title:** Spring drop\n\
**Description:**\n\
The window changed.\n\
**Keywords:** spring, drop\n";
        let pieces = parse_block("spring.md", raw);
        assert_eq!(pieces.len(), 2);
        assert_eq!(
            pieces[0].image_names,
            vec![
                "my cover.jpg".to_string(),
                "detail.png".into(),
                "notes/extra.jpg".into()
            ]
        );
        assert_eq!(pieces[1].image_names, pieces[0].image_names);
        assert!(!pieces[0].body.contains("cover"));
        assert!(!pieces[0].body.contains("配图"));
        assert!(!pieces[1].body.contains("detail.png"));
        assert!(pieces[0].body.contains("橱窗换了"));
        assert_eq!(split_image_names("cover.jpg， detail.png\nboard.jpg"), vec![
            "cover.jpg".to_string(),
            "detail.png".into(),
            "board.jpg".into(),
        ]);
    }

    #[test]
    fn does_not_guess_a_slogan_line() {
        let raw = "这是正文。\n\n本地优先 · 不代发 · 买断制";
        let pieces = parse_block("标题", raw);
        assert!(pieces[0].keywords.is_empty());
        assert!(pieces[0].body.contains("买断制"));
    }

    #[test]
    fn xhs_keywords_get_hashes_and_douyin_appends() {
        let words = vec!["本地优先".into(), "发布台账".into()];
        assert_eq!(format_keywords(Some("xhs"), &words), "#本地优先 #发布台账");
        let body = body_for_channel_copy(Some("douyin"), "口播正文", &words);
        assert!(body.starts_with("口播正文"));
        assert!(body.contains("#本地优先"));
        let xhs = body_for_channel_copy(Some("xhs"), "笔记正文", &words);
        assert_eq!(xhs, "笔记正文");
    }

    #[test]
    fn full_text_keeps_keywords_once() {
        let words = vec!["本地优先".into(), "发布台账".into()];
        let text = full_text_for_copy(Some("douyin"), "标题", "口播正文", &words);
        assert_eq!(text, "标题\n\n口播正文\n\n#本地优先 #发布台账");
        assert_eq!(text.matches("本地优先").count(), 1);
        let plain = format_keywords_with_hash(Some("xhs"), &words, false);
        assert_eq!(plain, "本地优先、发布台账");
        let xhs = full_text_for_copy(Some("xhs"), "标题", "**笔记**正文", &words);
        assert!(xhs.contains("笔记正文"));
        assert!(!xhs.contains("**"));
        assert_eq!(xhs.matches("#本地优先").count(), 1);
    }
}
