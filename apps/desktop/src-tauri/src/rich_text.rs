use comrak::{markdown_to_html as comrak_to_html, ComrakOptions};

pub fn markdown_to_html(markdown: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    comrak_to_html(markdown, &options)
}

pub fn markdown_to_plain(markdown: &str) -> String {
    let mut text = markdown.to_string();
    text = regex::Regex::new(r"(?m)^\s*\|[-: |]+\|\s*$")
        .unwrap()
        .replace_all(&text, "")
        .into_owned();
    text = regex::Regex::new(r"(?m)^\|(.+)\|$")
        .unwrap()
        .replace_all(&text, |caps: &regex::Captures| {
            caps[1]
                .split('|')
                .map(str::trim)
                .filter(|cell| !cell.is_empty())
                .collect::<Vec<_>>()
                .join("\t")
        })
        .into_owned();
    text = regex::Regex::new(r"(?m)^#{1,6}\s+")
        .unwrap()
        .replace_all(&text, "")
        .into_owned();
    text = regex::Regex::new(r"\*\*(.+?)\*\*")
        .unwrap()
        .replace_all(&text, "$1")
        .into_owned();
    text = regex::Regex::new(r"\*(.+?)\*")
        .unwrap()
        .replace_all(&text, "$1")
        .into_owned();
    text = regex::Regex::new(r"`(.+?)`")
        .unwrap()
        .replace_all(&text, "$1")
        .into_owned();
    text = regex::Regex::new(r"(?m)^\s*[-*+]\s+")
        .unwrap()
        .replace_all(&text, "• ")
        .into_owned();
    text = regex::Regex::new(r"(?m)^\s*\d+\.\s+")
        .unwrap()
        .replace_all(&text, "")
        .into_owned();
    text = regex::Regex::new(r"\[([^\]]+)\]\([^)]+\)")
        .unwrap()
        .replace_all(&text, "$1")
        .into_owned();
    text = regex::Regex::new(r"(?m)^>\s?")
        .unwrap()
        .replace_all(&text, "")
        .into_owned();
    text.trim().to_string()
}

pub fn copy_markdown_rich_text(markdown: &str) -> Result<(), String> {
    let html = markdown_to_html(markdown);
    let plain = markdown_to_plain(markdown);
    copy_rich_text(&html, &plain)
}

pub fn copy_plain_text(plain: &str) -> Result<(), String> {
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    clipboard
        .set_text(plain.to_string())
        .map_err(|e| e.to_string())
}

pub fn copy_rich_text(html: &str, plain: &str) -> Result<(), String> {
    let fragment = format!(
        "<!DOCTYPE html><html><body><!--StartFragment-->{html}<!--EndFragment--></body></html>"
    );
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    clipboard
        .set_html(fragment, Some(plain.to_string()))
        .map_err(|e| e.to_string())
}
