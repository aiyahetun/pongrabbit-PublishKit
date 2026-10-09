use std::collections::{HashMap, HashSet};
use std::path::Path;

use chrono::{Duration, NaiveDate};
use serde::Serialize;
use uuid::Uuid;

use crate::content_fields::parse_block;
use crate::doc_import::{is_split_format, is_table_format, read_source_content, source_format};
use crate::md_split::{preview_splits_from_content, SplitStrategy};
use crate::table_import::preview_table_import;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchImportItem {
    pub id: String,
    pub source_path: String,
    pub source_name: String,
    pub title: String,
    pub body: String,
    pub body_preview: String,
    pub keywords: Vec<String>,
    pub language: String,
    pub pair_key: Option<String>,
    pub file_date: Option<String>,
    pub channel_name: Option<String>,
    #[serde(default)]
    pub image_names: Vec<String>,
    #[serde(default)]
    pub images: Vec<MatchedImage>,
    #[serde(default)]
    pub missing_images: Vec<String>,
    #[serde(default)]
    pub images_ambiguous: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchedImage {
    pub id: String,
    pub file_name: String,
    pub path: String,
    pub included: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumb_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImageCandidate {
    pub id: String,
    pub file_name: String,
    pub path: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchImportPreview {
    pub items: Vec<BatchImportItem>,
    pub errors: Vec<String>,
}

pub fn preview_paths(paths: &[String]) -> BatchImportPreview {
    let mut items = Vec::new();
    let mut errors = Vec::new();
    for path in paths {
        let name = Path::new(path)
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or(path)
            .to_string();
        match preview_one(path, &name) {
            Ok(mut next) => items.append(&mut next),
            Err(err) => errors.push(format!("{name}: {err}")),
        }
    }
    BatchImportPreview { items, errors }
}

fn preview_one(path: &str, source_name: &str) -> Result<Vec<BatchImportItem>, String> {
    let format = source_format(Path::new(path)).ok_or_else(|| "不支持的文件格式".to_string())?;
    if is_table_format(format) {
        let table = preview_table_import(path)?;
        return Ok(table
            .rows
            .into_iter()
            .map(|row| {
                new_item(
                    path,
                    source_name,
                    row.title,
                    row.body.clone(),
                    row.body_preview,
                    row.keywords,
                    row.language,
                    row.pair_key,
                    row.scheduled_date,
                    row.channel_name,
                    row.image_names,
                )
            })
            .collect());
    }
    if !is_split_format(format) {
        return Err("不支持的文件格式".into());
    }
    let content = read_source_content(path)?;
    let smart = preview_splits_from_content(&content, SplitStrategy::Smart)?;
    let recommended: Vec<_> = smart.into_iter().filter(|item| item.recommended).collect();
    let base = if recommended.is_empty() {
        preview_splits_from_content(&content, SplitStrategy::Whole)?
    } else {
        recommended
    };
    Ok(expand_labeled(path, source_name, base))
}

fn expand_labeled(
    path: &str,
    source_name: &str,
    previews: Vec<crate::md_split::SplitPreview>,
) -> Vec<BatchImportItem> {
    let mut items = Vec::new();
    for preview in previews {
        let pieces = parse_block(&preview.title, &preview.body);
        let pair_key = if pieces.len() > 1 {
            Some(Uuid::new_v4().to_string())
        } else {
            None
        };
        for piece in pieces {
            if piece.title.trim().is_empty() && piece.body.trim().is_empty() {
                continue;
            }
            let body = piece.body;
            let clipped: String = body.chars().take(120).collect();
            let body_preview = if body.chars().count() > 120 {
                format!("{clipped}…")
            } else {
                clipped
            };
            items.push(new_item(
                path,
                source_name,
                piece.title,
                body,
                body_preview,
                piece.keywords,
                if piece.language.is_empty() {
                    preview.language.clone()
                } else {
                    piece.language
                },
                pair_key.clone(),
                None,
                None,
                piece.image_names,
            ));
        }
    }
    items
}

fn new_item(
    source_path: &str,
    source_name: &str,
    title: String,
    body: String,
    body_preview: String,
    keywords: Vec<String>,
    language: String,
    pair_key: Option<String>,
    file_date: Option<String>,
    channel_name: Option<String>,
    image_names: Vec<String>,
) -> BatchImportItem {
    BatchImportItem {
        id: Uuid::new_v4().to_string(),
        source_path: source_path.to_string(),
        source_name: source_name.to_string(),
        title,
        body,
        body_preview,
        keywords,
        language,
        pair_key,
        file_date,
        channel_name,
        image_names,
        images: Vec::new(),
        missing_images: Vec::new(),
        images_ambiguous: false,
    }
}

pub fn attach_images(
    items: &mut [BatchImportItem],
    candidates: &[ImageCandidate],
    media_root: Option<&str>,
) {
    let mut order = Vec::new();
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (index, item) in items.iter_mut().enumerate() {
        item.images.clear();
        item.missing_images.clear();
        item.images_ambiguous = false;
        if !groups.contains_key(&item.source_path) {
            order.push(item.source_path.clone());
        }
        groups.entry(item.source_path.clone()).or_default().push(index);
    }
    for path in order {
        let Some(indices) = groups.remove(&path) else {
            continue;
        };
        apply_image_group(items, &indices, candidates, media_root);
    }
}

fn apply_image_group(
    items: &mut [BatchImportItem],
    indices: &[usize],
    candidates: &[ImageCandidate],
    media_root: Option<&str>,
) {
    if indices.is_empty() {
        return;
    }
    let sections: HashSet<String> = indices.iter().map(|index| section_key(&items[*index])).collect();
    let names_empty = indices.iter().all(|index| items[*index].image_names.is_empty());
    let stem_hits = if sections.len() == 1 && names_empty {
        let stem = source_stem(&items[indices[0]]);
        candidates
            .iter()
            .filter(|candidate| candidate.kind == "image" && !stem.is_empty() && stem_key(&candidate.file_name) == stem)
            .cloned()
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    for index in indices {
        if !items[*index].image_names.is_empty() {
            let resolved = resolve_names(&items[*index].image_names, candidates, media_root);
            items[*index].images = resolved.images;
            items[*index].missing_images = resolved.missing;
            items[*index].images_ambiguous = resolved.ambiguous;
        } else if !stem_hits.is_empty() {
            items[*index].images = stem_hits.iter().map(|candidate| matched(candidate, true)).collect();
        }
    }
}

struct NameResolve {
    images: Vec<MatchedImage>,
    missing: Vec<String>,
    ambiguous: bool,
}

fn resolve_names(names: &[String], candidates: &[ImageCandidate], media_root: Option<&str>) -> NameResolve {
    let mut images = Vec::new();
    let mut missing = Vec::new();
    let mut ambiguous = false;
    for name in names {
        match lookup_name(name, candidates, media_root) {
            NameHit::Missing => missing.push(name.clone()),
            NameHit::Chosen(candidate) => push_unique(&mut images, matched(candidate, true)),
            NameHit::Choose(list) => {
                ambiguous = true;
                for candidate in list {
                    push_unique(&mut images, matched(candidate, false));
                }
            }
        }
    }
    NameResolve {
        images,
        missing,
        ambiguous,
    }
}

enum NameHit<'a> {
    Missing,
    Chosen(&'a ImageCandidate),
    Choose(Vec<&'a ImageCandidate>),
}

fn lookup_name<'a>(
    name: &str,
    candidates: &'a [ImageCandidate],
    media_root: Option<&str>,
) -> NameHit<'a> {
    let name = name.trim();
    if name.is_empty() {
        return NameHit::Missing;
    }
    let has_sep = name.contains('/') || name.contains('\\');
    let keyed = normalize_path_key(name);
    let hits: Vec<&ImageCandidate> = candidates
        .iter()
        .filter(|candidate| {
            if candidate.kind != "image" {
                return false;
            }
            if has_sep {
                let path = normalize_path_key(&candidate.path);
                path == keyed || path.ends_with(&format!("/{keyed}"))
            } else {
                candidate.file_name.eq_ignore_ascii_case(name)
            }
        })
        .collect();
    if hits.is_empty() {
        return NameHit::Missing;
    }
    if hits.len() == 1 {
        return NameHit::Chosen(hits[0]);
    }
    if let Some(root) = media_root.map(str::trim).filter(|value| !value.is_empty()) {
        let under: Vec<&ImageCandidate> = hits
            .iter()
            .copied()
            .filter(|candidate| is_under_root(&candidate.path, root))
            .collect();
        if under.len() == 1 {
            return NameHit::Chosen(under[0]);
        }
    }
    NameHit::Choose(hits)
}

fn push_unique(images: &mut Vec<MatchedImage>, image: MatchedImage) {
    if let Some(existing) = images.iter_mut().find(|item| item.id == image.id) {
        if image.included {
            existing.included = true;
        }
        return;
    }
    images.push(image);
}

fn matched(candidate: &ImageCandidate, included: bool) -> MatchedImage {
    MatchedImage {
        id: candidate.id.clone(),
        file_name: candidate.file_name.clone(),
        path: candidate.path.clone(),
        included,
        thumb_path: None,
    }
}

fn section_key(item: &BatchImportItem) -> String {
    item.pair_key
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| item.id.clone())
}

fn source_stem(item: &BatchImportItem) -> String {
    let name = item.source_name.trim();
    if !name.is_empty() {
        return stem_key(name);
    }
    Path::new(&item.source_path)
        .file_name()
        .and_then(|value| value.to_str())
        .map(stem_key)
        .unwrap_or_default()
}

fn stem_key(file_name: &str) -> String {
    Path::new(file_name.trim())
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or(file_name.trim())
        .to_ascii_lowercase()
}

fn normalize_path_key(raw: &str) -> String {
    let mut text = raw.trim().trim_end_matches(['/', '\\']).replace('\\', "/");
    while let Some(rest) = text.strip_prefix("./") {
        text = rest.to_string();
    }
    text.to_ascii_lowercase()
}

fn is_under_root(path: &str, root: &str) -> bool {
    let path = normalize_path_key(path);
    let root = normalize_path_key(root);
    if root.is_empty() {
        return false;
    }
    path == root || path.starts_with(&format!("{root}/"))
}

pub fn normalize_date(raw: &str) -> Option<String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }
    if let Some(date) = iso_prefix(raw) {
        return Some(date);
    }
    if let Some(date) = ymd_separated(raw) {
        return Some(date);
    }
    if let Some(caps) = regex::Regex::new(r"(\d{4})\s*年\s*(\d{1,2})\s*月\s*(\d{1,2})")
        .ok()?
        .captures(raw)
    {
        return ymd(&caps[1], &caps[2], &caps[3]);
    }
    excel_serial(raw)
}

fn iso_prefix(raw: &str) -> Option<String> {
    let head: String = raw.chars().take(10).collect();
    if head.len() == 10 && head.as_bytes()[4] == b'-' && head.as_bytes()[7] == b'-' {
        return ymd(&head[0..4], &head[5..7], &head[8..10]);
    }
    None
}

fn ymd_separated(raw: &str) -> Option<String> {
    let caps = regex::Regex::new(r"(\d{4})[./](\d{1,2})[./](\d{1,2})")
        .ok()?
        .captures(raw)?;
    ymd(&caps[1], &caps[2], &caps[3])
}

fn ymd(year: &str, month: &str, day: &str) -> Option<String> {
    let year: i32 = year.parse().ok()?;
    let month: u32 = month.parse().ok()?;
    let day: u32 = day.parse().ok()?;
    NaiveDate::from_ymd_opt(year, month, day).map(|date| date.format("%Y-%m-%d").to_string())
}

fn excel_serial(raw: &str) -> Option<String> {
    let number: f64 = raw.parse().ok()?;
    if !(20_000.0..80_000.0).contains(&number) {
        return None;
    }
    let epoch = NaiveDate::from_ymd_opt(1899, 12, 30)?;
    let date = epoch.checked_add_signed(Duration::days(number.floor() as i64))?;
    Some(date.format("%Y-%m-%d").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_common_date_shapes() {
        assert_eq!(normalize_date("2026-10-08").as_deref(), Some("2026-10-08"));
        assert_eq!(normalize_date("2026/10/8").as_deref(), Some("2026-10-08"));
        assert_eq!(normalize_date("2026年10月8日").as_deref(), Some("2026-10-08"));
        assert_eq!(normalize_date("2026-10-08T09:00:00").as_deref(), Some("2026-10-08"));
        assert_eq!(normalize_date("45939").as_deref(), Some("2025-10-09"));
        assert!(normalize_date("不是日期").is_none());
    }

    fn candidate(id: &str, file_name: &str, path: &str) -> ImageCandidate {
        ImageCandidate {
            id: id.into(),
            file_name: file_name.into(),
            path: path.into(),
            kind: "image".into(),
        }
    }

    fn sample(path: &str, name: &str, pair: Option<&str>, names: &[&str]) -> BatchImportItem {
        new_item(
            path,
            name,
            "标题".into(),
            "正文".into(),
            "正文".into(),
            Vec::new(),
            "zh".into(),
            pair.map(str::to_string),
            None,
            None,
            names.iter().map(|value| (*value).to_string()).collect(),
        )
    }

    #[test]
    fn image_library_matches_written_names() {
        let library = vec![
            candidate("a", "cover.jpg", r"D:\library\cover.jpg"),
            candidate("b", "cover.jpg", r"D:\other\cover.jpg"),
            candidate("c", "detail.png", r"D:\library\detail.png"),
            candidate("d", "cover.jpg", r"D:\archive\subdir\cover.jpg"),
            candidate("e", "spring.jpg", r"D:\library\spring.jpg"),
            candidate("f", "spring-2.jpg", r"D:\library\spring-2.jpg"),
            candidate("g", "spring.png", r"D:\library\spring.png"),
            candidate("h", "clip.mp4", r"D:\library\clip.mp4"),
        ];
        let mut video = candidate("h", "spring.mp4", r"D:\library\spring.mp4");
        video.kind = "video".into();

        let mut unique = vec![sample(r"D:\copy\post.md", "post.md", None, &["Detail.PNG"])];
        attach_images(&mut unique, &library, Some(r"D:\library"));
        assert_eq!(unique[0].images.len(), 1);
        assert!(unique[0].images[0].included);
        assert_eq!(unique[0].images[0].id, "c");
        assert!(unique[0].missing_images.is_empty());

        let mut rooted = vec![sample(r"D:\copy\post.md", "post.md", None, &["cover.jpg"])];
        attach_images(&mut rooted, &library, Some(r"D:\library\"));
        let chosen: Vec<_> = rooted[0]
            .images
            .iter()
            .filter(|image| image.included)
            .map(|image| image.id.as_str())
            .collect();
        assert_eq!(chosen, vec!["a"]);
        assert!(!rooted[0].images_ambiguous);

        let mut ambiguous = vec![sample(r"D:\copy\post.md", "post.md", None, &["cover.jpg"])];
        attach_images(&mut ambiguous, &library, None);
        assert!(ambiguous[0].images_ambiguous);
        assert!(ambiguous[0].images.iter().all(|image| !image.included));
        assert_eq!(ambiguous[0].images.len(), 3);

        let mut two_under_root = library.clone();
        two_under_root.push(candidate("n", "cover.jpg", r"D:\library\nested\cover.jpg"));
        let mut several_under = vec![sample(r"D:\copy\post.md", "post.md", None, &["cover.jpg"])];
        attach_images(&mut several_under, &two_under_root, Some(r"D:\library"));
        assert!(several_under[0].images_ambiguous);
        assert!(several_under[0].images.iter().all(|image| !image.included));
        assert_eq!(several_under[0].images.len(), 4);

        let mut by_path = vec![sample(
            r"D:\copy\post.md",
            "post.md",
            None,
            &["subdir/cover.jpg", "missing.jpg"],
        )];
        attach_images(&mut by_path, &library, Some(r"D:\library"));
        assert_eq!(by_path[0].images.len(), 1);
        assert_eq!(by_path[0].images[0].id, "d");
        assert!(by_path[0].images[0].included);
        assert_eq!(by_path[0].missing_images, vec!["missing.jpg".to_string()]);

        let mut stem = vec![
            sample(r"D:\copy\spring.md", "spring.md", Some("pair"), &[]),
            sample(r"D:\copy\spring.md", "spring.md", Some("pair"), &[]),
        ];
        let mut with_video = library.clone();
        with_video.push(video);
        attach_images(&mut stem, &with_video, Some(r"D:\library"));
        for item in &stem {
            let ids: Vec<_> = item.images.iter().map(|image| image.id.as_str()).collect();
            assert_eq!(ids, vec!["e", "g"]);
            assert!(item.images.iter().all(|image| image.included));
        }

        let mut many = vec![
            sample(r"D:\copy\spring.md", "spring.md", None, &[]),
            sample(r"D:\copy\spring.md", "spring.md", None, &[]),
        ];
        attach_images(&mut many, &library, Some(r"D:\library"));
        assert!(many.iter().all(|item| item.images.is_empty()));

        let mut named_file = vec![sample(r"D:\copy\spring.md", "spring.md", None, &["detail.png"])];
        attach_images(&mut named_file, &library, Some(r"D:\library"));
        assert_eq!(named_file[0].images.len(), 1);
        assert_eq!(named_file[0].images[0].id, "c");
    }
}
