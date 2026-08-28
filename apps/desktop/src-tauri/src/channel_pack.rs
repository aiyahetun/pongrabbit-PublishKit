use crate::rich_text;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackTextFile {
    pub file_name: &'static str,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelPackSpec {
    pub folder_tag: String,
    pub files: Vec<PackTextFile>,
    pub readme: String,
}

pub fn markdown_plain(body: &str) -> String {
    rich_text::markdown_to_plain(body)
}

pub fn format_plain_for_channel(channel_id: &str, title: &str, body: &str) -> String {
    let plain = markdown_plain(body);
    match channel_id {
        "wechat_mp" => format!("{title}\n\n{plain}"),
        "producthunt" => {
            let mut lines = plain.lines();
            let tagline = lines.next().unwrap_or(title).trim();
            let rest = lines.collect::<Vec<_>>().join("\n").trim().to_string();
            if rest.is_empty() {
                tagline.to_string()
            } else {
                format!("{tagline}\n\n{rest}")
            }
        }
        "pinterest" => format!("{title}\n\n{plain}"),
        _ => plain,
    }
}

pub fn build_channel_pack(
    channel_id: &str,
    channel_name: &str,
    title: &str,
    body: &str,
) -> ChannelPackSpec {
    let plain = format_plain_for_channel(channel_id, title, body);
    let md = format!("# {title}\n\n{body}");

    match channel_id {
        "xhs" => ChannelPackSpec {
            folder_tag: "xhs".to_string(),
            files: vec![
                PackTextFile {
                    file_name: "正文.txt",
                    content: plain.clone(),
                },
                PackTextFile {
                    file_name: "标签.txt",
                    content: "#话题1 #话题2\n（请替换为小红书话题标签）".to_string(),
                },
            ],
            readme: format!(
                "{channel_name} 发布包\n\n1. 复制 正文.txt 到笔记编辑器\n2. 添加 标签.txt 中的话题\n3. 从 media/ 上传配图"
            ),
        },
        "douyin" | "kuaishou" | "channels" => ChannelPackSpec {
            folder_tag: channel_id.to_string(),
            files: vec![PackTextFile {
                file_name: "口播.txt",
                content: plain,
            }],
            readme: format!("{channel_name} 发布包\n\n复制 口播.txt 作为视频描述/口播文案。"),
        },
        "instagram" | "tiktok" | "threads" => ChannelPackSpec {
            folder_tag: channel_id.to_string(),
            files: vec![PackTextFile {
                file_name: "caption.txt",
                content: plain,
            }],
            readme: format!("{channel_name} 发布包\n\n复制 caption.txt 到帖子说明。"),
        },
        "wechat_mp" => ChannelPackSpec {
            folder_tag: "wechat".to_string(),
            files: vec![
                PackTextFile {
                    file_name: "title.txt",
                    content: title.to_string(),
                },
                PackTextFile {
                    file_name: "正文.md",
                    content: body.to_string(),
                },
            ],
            readme: format!("{channel_name} 发布包\n\n标题用 title.txt，正文用 正文.md（可转富文本）。"),
        },
        "zhihu" | "weibo" | "toutiao" | "reddit" | "linkedin" | "facebook" | "twitter" => {
            ChannelPackSpec {
                folder_tag: channel_id.to_string(),
                files: vec![PackTextFile {
                    file_name: "正文.txt",
                    content: plain,
                }],
                readme: format!("{channel_name} 发布包\n\n复制 正文.txt 到编辑器。"),
            }
        }
        "youtube" => ChannelPackSpec {
            folder_tag: "youtube".to_string(),
            files: vec![PackTextFile {
                file_name: "description.txt",
                content: plain,
            }],
            readme: "YouTube 发布包\n\n复制 description.txt 到视频描述。".to_string(),
        },
        "pinterest" => ChannelPackSpec {
            folder_tag: "pinterest".to_string(),
            files: vec![
                PackTextFile {
                    file_name: "title.txt",
                    content: title.to_string(),
                },
                PackTextFile {
                    file_name: "description.txt",
                    content: markdown_plain(body),
                },
            ],
            readme: "Pinterest 发布包\n\ntitle.txt → Pin 标题；description.txt → 描述。".to_string(),
        },
        "producthunt" => ChannelPackSpec {
            folder_tag: "producthunt".to_string(),
            files: vec![
                PackTextFile {
                    file_name: "tagline.txt",
                    content: plain.lines().next().unwrap_or(title).trim().to_string(),
                },
                PackTextFile {
                    file_name: "description.txt",
                    content: plain,
                },
            ],
            readme: "Product Hunt 发布包\n\ntagline.txt → Tagline；description.txt → 详情。".to_string(),
        },
        "bilibili" => ChannelPackSpec {
            folder_tag: "bilibili".to_string(),
            files: vec![
                PackTextFile {
                    file_name: "标题.txt",
                    content: title.to_string(),
                },
                PackTextFile {
                    file_name: "简介.txt",
                    content: plain,
                },
            ],
            readme: "哔哩哔哩发布包\n\n标题.txt → 视频标题；简介.txt → 简介区。".to_string(),
        },
        _ => ChannelPackSpec {
            folder_tag: if channel_id.is_empty() {
                "pack".to_string()
            } else {
                channel_id.to_string()
            },
            files: vec![PackTextFile {
                file_name: "content.md",
                content: md,
            }],
            readme: format!("{channel_name} 发布包\n\ncontent.md 含 Markdown 正文；media/ 为配图。"),
        },
    }
}

pub fn uses_plain_copy(channel_id: &str) -> bool {
    matches!(
        channel_id,
        "xhs"
            | "douyin"
            | "kuaishou"
            | "channels"
            | "instagram"
            | "tiktok"
            | "threads"
            | "zhihu"
            | "weibo"
            | "toutiao"
            | "reddit"
            | "linkedin"
            | "facebook"
            | "twitter"
            | "youtube"
            | "pinterest"
            | "producthunt"
            | "bilibili"
    )
}

pub fn copy_task_body(channel_id: &str, title: &str, markdown_body: &str) -> Result<(), String> {
    if uses_plain_copy(channel_id) {
        let plain = format_plain_for_channel(channel_id, title, markdown_body);
        rich_text::copy_plain_text(&plain)
    } else {
        rich_text::copy_markdown_rich_text(markdown_body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xhs_pack_has_body_and_tags_files() {
        let pack = build_channel_pack("xhs", "小红书", "春季上新", "这是正文 **加粗**");
        assert_eq!(pack.files.len(), 2);
        assert_eq!(pack.files[0].file_name, "正文.txt");
        assert!(pack.files[0].content.contains("这是正文"));
        assert!(!pack.files[0].content.contains("**"));
        assert_eq!(pack.files[1].file_name, "标签.txt");
    }

    #[test]
    fn wechat_pack_has_title_and_body() {
        let pack = build_channel_pack("wechat_mp", "微信公众号", "标题", "正文");
        assert_eq!(pack.files[0].file_name, "title.txt");
        assert_eq!(pack.files[1].file_name, "正文.md");
    }

    #[test]
    fn default_pack_uses_markdown() {
        let pack = build_channel_pack("custom", "自定义", "标题", "正文");
        assert_eq!(pack.files[0].file_name, "content.md");
        assert!(pack.files[0].content.starts_with("# 标题"));
    }
}
