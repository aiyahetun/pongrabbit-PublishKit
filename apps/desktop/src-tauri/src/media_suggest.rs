use std::path::Path;

fn file_stem_lower(path_or_name: &str) -> String {
    Path::new(path_or_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase()
}

pub fn score_media_for_content(
    source_path: &str,
    title: &str,
    media_path: &str,
    file_name: &str,
    copy_root: Option<&str>,
    media_root: Option<&str>,
) -> (i32, &'static str) {
    let mut score = 0;
    let mut reason = "";

    if !source_path.starts_with("manual://") {
        let src = Path::new(source_path);
        let media = Path::new(media_path);
        if let (Some(src_parent), Some(media_parent)) = (src.parent(), media.parent()) {
            if src_parent == media_parent {
                score += 100;
                reason = "same_dir";
            }
        }

        if score == 0 {
            if let (Some(copy_root), Some(media_root)) = (copy_root, media_root) {
                let copy = Path::new(copy_root);
                let media_root_path = Path::new(media_root);
                if let (Ok(src_rel), Ok(media_rel)) = (
                    src.strip_prefix(copy),
                    media.strip_prefix(media_root_path),
                ) {
                    let src_parent = src_rel.parent();
                    let media_parent = media_rel.parent();
                    let src_stem = file_stem_lower(source_path);
                    let media_stem = file_stem_lower(file_name);
                    if src_parent == media_parent && src_stem.len() >= 2 && src_stem == media_stem {
                        score += 90;
                        reason = "mirror_path";
                    }
                }
            }
        }

        let src_stem = file_stem_lower(source_path);
        let media_stem = file_stem_lower(file_name);
        if src_stem.len() >= 2
            && (src_stem == media_stem
                || media_stem.contains(&src_stem)
                || src_stem.contains(&media_stem))
        {
            score += 70;
            if reason.is_empty() {
                reason = "source_match";
            }
        }
    }

    let title_lower = title.to_lowercase();
    let file_lower = file_name.to_lowercase();
    let stem = file_stem_lower(file_name);

    if stem.len() >= 2 && title_lower.contains(&stem) {
        score += 60;
        if reason.is_empty() {
            reason = "name_match";
        }
    }

    if stem.len() >= 2 && stem.contains(&title_lower) && title_lower.len() >= 2 {
        score += 50;
        if reason.is_empty() {
            reason = "name_match";
        }
    }

    for token in title.split(|c: char| !c.is_alphanumeric()) {
        let token = token.trim().to_lowercase();
        if token.len() >= 2 && file_lower.contains(&token) {
            score += 40;
            if reason.is_empty() {
                reason = "name_match";
            }
            break;
        }
    }

    (score, reason)
}

#[cfg(test)]
mod tests {
    use super::score_media_for_content;
    use std::path::PathBuf;

    fn fixture_path(parts: &[&str]) -> String {
        let mut path = PathBuf::from(std::env::temp_dir());
        for part in parts {
            path.push(part);
        }
        path.to_string_lossy().into_owned()
    }

    #[test]
    fn same_directory_scores() {
        let root = fixture_path(&["publishkit-media-suggest-test", "docs"]);
        let (score, reason) = score_media_for_content(
            &fixture_path(&["publishkit-media-suggest-test", "docs", "post.md"]),
            "Post title",
            &fixture_path(&["publishkit-media-suggest-test", "docs", "cover.jpg"]),
            "cover.jpg",
            None,
            None,
        );
        assert!(score >= 100, "score={score} for root {root}");
        assert_eq!(reason, "same_dir");
    }

    #[test]
    fn mirror_roots_scores() {
        let copy_root = fixture_path(&["publishkit-media-suggest-test", "copy"]);
        let media_root = fixture_path(&["publishkit-media-suggest-test", "media"]);
        let (score, reason) = score_media_for_content(
            &fixture_path(&["publishkit-media-suggest-test", "copy", "2024", "spring.md"]),
            "Spring",
            &fixture_path(&["publishkit-media-suggest-test", "media", "2024", "spring.jpg"]),
            "spring.jpg",
            Some(&copy_root),
            Some(&media_root),
        );
        assert!(score >= 90, "score={score}");
        assert_eq!(reason, "mirror_path");
    }
}
