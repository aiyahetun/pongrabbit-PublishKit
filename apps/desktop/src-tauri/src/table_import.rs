use calamine::{open_workbook_auto, Data, Reader};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRowPreview {
    pub index: usize,
    pub title: String,
    pub body: String,
    pub language: String,
    pub channel_name: Option<String>,
    pub body_preview: String,
    pub recommended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableImportPreview {
    pub columns: Vec<String>,
    pub title_column: String,
    pub body_column: String,
    pub language_column: Option<String>,
    pub channel_column: Option<String>,
    pub rows: Vec<TableRowPreview>,
}

#[derive(Debug, Clone)]
struct ColumnMapping {
    columns: Vec<String>,
    title_idx: usize,
    body_idx: usize,
    language_idx: Option<usize>,
    channel_idx: Option<usize>,
}

pub fn preview_table_import(path: &str) -> Result<TableImportPreview, String> {
    let path_ref = Path::new(path);
    let ext = path_ref
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let (headers, data_rows) = match ext.as_str() {
        "csv" => read_csv_rows(path_ref)?,
        "xlsx" | "xls" => read_excel_rows(path_ref)?,
        _ => return Err("不是表格文件".into()),
    };

    if data_rows.is_empty() {
        return Err("表格中没有数据行".into());
    }

    let mapping = detect_columns(&headers, &data_rows)?;
    let rows = build_row_previews(&mapping, &data_rows);

    Ok(TableImportPreview {
        columns: mapping.columns.clone(),
        title_column: mapping.columns[mapping.title_idx].clone(),
        body_column: mapping.columns[mapping.body_idx].clone(),
        language_column: mapping
            .language_idx
            .map(|idx| mapping.columns[idx].clone()),
        channel_column: mapping
            .channel_idx
            .map(|idx| mapping.columns[idx].clone()),
        rows,
    })
}

pub fn selected_table_rows(
    path: &str,
    selected_indexes: &[usize],
) -> Result<Vec<TableRowPreview>, String> {
    let preview = preview_table_import(path)?;
    if selected_indexes.is_empty() {
        return Ok(preview.rows);
    }
    Ok(preview
        .rows
        .into_iter()
        .filter(|row| selected_indexes.contains(&row.index))
        .collect())
}

fn read_csv_rows(path: &Path) -> Result<(Vec<String>, Vec<Vec<String>>), String> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(path)
        .map_err(|e| e.to_string())?;

    let headers = reader
        .headers()
        .map_err(|e| e.to_string())?
        .iter()
        .enumerate()
        .map(|(idx, header)| {
            let trimmed = header.trim();
            if trimmed.is_empty() {
                format!("列{}", idx + 1)
            } else {
                trimmed.to_string()
            }
        })
        .collect::<Vec<_>>();

    let mut rows = Vec::new();
    for record in reader.records() {
        let record = record.map_err(|e| e.to_string())?;
        rows.push(record.iter().map(|cell| cell.trim().to_string()).collect());
    }

    Ok((headers, rows))
}

fn read_excel_rows(path: &Path) -> Result<(Vec<String>, Vec<Vec<String>>), String> {
    let mut workbook = open_workbook_auto(path).map_err(|e| e.to_string())?;
    let sheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or_else(|| "Excel 文件没有工作表".to_string())?;
    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| e.to_string())?;

    let mut iter = range.rows();
    let header_row = iter
        .next()
        .ok_or_else(|| "Excel 工作表为空".to_string())?;
    let headers = header_row
        .iter()
        .enumerate()
        .map(|(idx, cell)| {
            let text = cell_to_string(cell);
            if text.trim().is_empty() {
                format!("列{}", idx + 1)
            } else {
                text
            }
        })
        .collect::<Vec<_>>();

    let mut rows = Vec::new();
    for row in iter {
        rows.push(row.iter().map(cell_to_string).collect());
    }

    Ok((headers, rows))
}

fn cell_to_string(cell: &Data) -> String {
    match cell {
        Data::Empty => String::new(),
        Data::String(value) => value.trim().to_string(),
        Data::Float(value) => {
            if value.fract() == 0.0 {
                format!("{}", *value as i64)
            } else {
                value.to_string()
            }
        }
        Data::Int(value) => value.to_string(),
        Data::Bool(value) => value.to_string(),
        Data::DateTime(value) => value.to_string(),
        Data::DateTimeIso(value) => value.clone(),
        Data::DurationIso(value) => value.clone(),
        Data::Error(_) => String::new(),
    }
}

fn detect_columns(headers: &[String], rows: &[Vec<String>]) -> Result<ColumnMapping, String> {
    let normalized: Vec<String> = headers.iter().map(|header| normalize_header(header)).collect();

    let mut title_idx = find_column(&normalized, TITLE_KEYS);
    let mut body_idx = find_column(&normalized, BODY_KEYS);
    let language_idx = find_column(&normalized, LANGUAGE_KEYS);
    let channel_idx = find_column(&normalized, CHANNEL_KEYS);

    if body_idx.is_none() {
        body_idx = find_longest_text_column(rows);
    }
    if title_idx.is_none() {
        title_idx = (0..headers.len())
            .find(|idx| Some(*idx) != body_idx)
            .or(Some(0));
    }

    let title_idx = title_idx.ok_or_else(|| "无法识别标题列".to_string())?;
    let body_idx = body_idx.ok_or_else(|| "无法识别正文列".to_string())?;

    if title_idx == body_idx {
        return Err("标题列与正文列不能相同".into());
    }

    Ok(ColumnMapping {
        columns: headers.to_vec(),
        title_idx,
        body_idx,
        language_idx,
        channel_idx,
    })
}

fn build_row_previews(mapping: &ColumnMapping, rows: &[Vec<String>]) -> Vec<TableRowPreview> {
    rows.iter()
        .enumerate()
        .filter_map(|(index, row)| {
            let title = cell_at(row, mapping.title_idx);
            let body = cell_at(row, mapping.body_idx);
            if title.is_empty() && body.is_empty() {
                return None;
            }
            let title = if title.is_empty() {
                format!("第 {} 行", index + 1)
            } else {
                title
            };
            if body.is_empty() {
                return None;
            }
            let language = mapping
                .language_idx
                .map(|idx| cell_at(row, idx))
                .filter(|value| !value.is_empty())
                .map(|value| crate::md_split::detect_language(&value))
                .unwrap_or_else(|| crate::md_split::detect_language(&body));
            let channel_name = mapping
                .channel_idx
                .map(|idx| cell_at(row, idx))
                .filter(|value| !value.is_empty());
            let preview = body.chars().take(180).collect::<String>();
            let body_preview = if body.chars().count() > 180 {
                format!("{preview}…")
            } else {
                preview
            };
            Some(TableRowPreview {
                index,
                title,
                body: body.clone(),
                language,
                channel_name,
                body_preview,
                recommended: true,
            })
        })
        .collect()
}

fn cell_at(row: &[String], idx: usize) -> String {
    row.get(idx).cloned().unwrap_or_default().trim().to_string()
}

fn normalize_header(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(' ', "")
}

const TITLE_KEYS: &[&str] = &[
    "title", "标题", "名称", "name", "内容标题", "topic", "主题", "条目",
];
const BODY_KEYS: &[&str] = &[
    "body", "正文", "content", "文案", "内容", "copy", "稿件", "text", "描述", "description",
    "caption", "post",
];
const LANGUAGE_KEYS: &[&str] = &["language", "语言", "lang", "语种"];
const CHANNEL_KEYS: &[&str] = &["channel", "渠道", "platform", "平台", "发布平台"];

fn find_column(headers: &[String], keys: &[&str]) -> Option<usize> {
    headers.iter().position(|header| {
        keys.iter()
            .any(|key| header.contains(key) || key.contains(header.as_str()))
    })
}

fn find_longest_text_column(rows: &[Vec<String>]) -> Option<usize> {
    let column_count = rows.iter().map(|row| row.len()).max().unwrap_or(0);
    (0..column_count)
        .map(|idx| {
            let total: usize = rows
                .iter()
                .map(|row| cell_at(row, idx).chars().count())
                .sum();
            (idx, total)
        })
        .max_by_key(|(_, total)| *total)
        .map(|(idx, _)| idx)
}
