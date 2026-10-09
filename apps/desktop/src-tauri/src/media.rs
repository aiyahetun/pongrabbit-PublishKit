use image::ImageReader;
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager};

const THUMB_MAX: u32 = 320;
const MAX_DECODE_BYTES: u64 = 48 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct ThumbUpdate {
    pub id: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub status: String,
    pub error: String,
}

pub fn thumb_cache_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("cache")
        .join("thumbs");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

pub fn thumb_file_path(app: &AppHandle, asset_id: &str) -> Result<PathBuf, String> {
    Ok(thumb_cache_dir(app)?.join(format!("{asset_id}.webp")))
}

pub fn read_image_dimensions(path: &Path) -> Option<(u32, u32)> {
    let mut file = fs::File::open(path).ok()?;
    let mut header = [0u8; 32];
    let read = file.read(&mut header).ok()?;
    if read >= 24 && header.starts_with(b"\x89PNG\r\n\x1a\n") {
        let width = u32::from_be_bytes(header[16..20].try_into().ok()?);
        let height = u32::from_be_bytes(header[20..24].try_into().ok()?);
        return Some((width, height));
    }
    if read >= 10 && header.starts_with(b"GIF") {
        let width = u16::from_le_bytes(header[6..8].try_into().ok()?) as u32;
        let height = u16::from_le_bytes(header[8..10].try_into().ok()?) as u32;
        return Some((width, height));
    }
    if read >= 12 && header.starts_with(b"RIFF") && &header[8..12] == b"WEBP" {
        return webp_dimensions(path);
    }
    if read >= 2 && header[0] == 0xFF && header[1] == 0xD8 {
        return jpeg_dimensions(path);
    }
    None
}

fn exceeds_decode_budget(dims: Option<(u32, u32)>) -> bool {
    matches!(dims, Some((width, height)) if (width as u64).saturating_mul(height as u64).saturating_mul(4) > MAX_DECODE_BYTES)
}

fn dims_i64(dims: Option<(u32, u32)>) -> (Option<i64>, Option<i64>) {
    match dims {
        Some((width, height)) => (Some(width as i64), Some(height as i64)),
        None => (None, None),
    }
}

pub fn probe_thumbnail(source: &Path, dest: &Path) -> (Option<i64>, Option<i64>, Result<(), String>) {
    if !source.is_file() {
        return (None, None, Err("源文件不存在".into()));
    }
    let dims = read_image_dimensions(source);
    let (width, height) = dims_i64(dims);
    if exceeds_decode_budget(dims) {
        return (width, height, write_shell_thumbnail(source, dest));
    }
    match write_decoded_thumbnail(source, dest) {
        Ok(()) => (width, height, Ok(())),
        Err(err) => match write_shell_thumbnail(source, dest) {
            Ok(()) => (width, height, Ok(())),
            Err(_) => (width, height, Err(err)),
        },
    }
}

fn write_decoded_thumbnail(source: &Path, dest: &Path) -> Result<(), String> {
    let img = ImageReader::open(source)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;
    let thumb = img.thumbnail(THUMB_MAX, THUMB_MAX);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    thumb
        .save_with_format(dest, image::ImageFormat::WebP)
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn jpeg_dimensions(path: &Path) -> Option<(u32, u32)> {
    let bytes = fs::read(path).ok()?;
    let mut index = 2usize;
    while index + 9 < bytes.len() {
        if bytes[index] != 0xFF {
            index += 1;
            continue;
        }
        let marker = bytes[index + 1];
        if marker == 0xD8 || marker == 0xD9 {
            index += 2;
            continue;
        }
        if index + 4 > bytes.len() {
            return None;
        }
        let length = u16::from_be_bytes([bytes[index + 2], bytes[index + 3]]) as usize;
        if length < 2 || index + 2 + length > bytes.len() {
            return None;
        }
        let is_sof = matches!(marker, 0xC0 | 0xC1 | 0xC2 | 0xC3 | 0xC5 | 0xC6 | 0xC7 | 0xC9 | 0xCA | 0xCB | 0xCD | 0xCE | 0xCF);
        if is_sof && index + 9 < bytes.len() {
            let height = u16::from_be_bytes([bytes[index + 5], bytes[index + 6]]) as u32;
            let width = u16::from_be_bytes([bytes[index + 7], bytes[index + 8]]) as u32;
            if width > 0 && height > 0 {
                return Some((width, height));
            }
        }
        index += 2 + length;
    }
    None
}

fn webp_dimensions(path: &Path) -> Option<(u32, u32)> {
    let bytes = fs::read(path).ok()?;
    if bytes.len() < 30 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WEBP" {
        return None;
    }
    let chunk = &bytes[12..16];
    if chunk == b"VP8X" && bytes.len() >= 30 {
        let width = 1 + u32::from_le_bytes([bytes[24], bytes[25], bytes[26], 0]);
        let height = 1 + u32::from_le_bytes([bytes[27], bytes[28], bytes[29], 0]);
        return Some((width, height));
    }
    if chunk == b"VP8 " && bytes.len() >= 30 {
        let width = u16::from_le_bytes([bytes[26], bytes[27]]) as u32 & 0x3FFF;
        let height = u16::from_le_bytes([bytes[28], bytes[29]]) as u32 & 0x3FFF;
        return Some((width, height));
    }
    if chunk == b"VP8L" && bytes.len() >= 25 {
        let b0 = bytes[21] as u32;
        let b1 = bytes[22] as u32;
        let b2 = bytes[23] as u32;
        let b3 = bytes[24] as u32;
        let width = (b0 | (b1 << 8) | (b2 << 16)) & 0x3FFF;
        let height = ((b2 >> 6) | (b3 << 2) | ((bytes.get(25).copied().unwrap_or(0) as u32) << 10)) & 0x3FFF;
        return Some((width.saturating_add(1), height.saturating_add(1)));
    }
    None
}

pub fn thumb_path_if_exists(app: &AppHandle, asset_id: &str, kind: &str) -> Option<String> {
    if kind != "image" {
        return None;
    }
    let dest = thumb_file_path(app, asset_id).ok()?;
    if dest.exists() {
        dest.to_str().map(str::to_string)
    } else {
        None
    }
}

pub fn generate_thumbnail_batch(
    app: &AppHandle,
    assets: &[(String, String, String, String)],
    batch_size: usize,
) -> (Vec<ThumbUpdate>, usize) {
    let pending: Vec<&(String, String, String, String)> = assets
        .iter()
        .filter(|(id, _, kind, status)| {
            kind == "image"
                && status != "failed"
                && thumb_path_if_exists(app, id, kind).is_none()
        })
        .collect();
    let mut updates = Vec::new();
    for (id, path, _, _) in pending.iter().take(batch_size) {
        let dest = match thumb_file_path(app, id) {
            Ok(dest) => dest,
            Err(error) => {
                updates.push(ThumbUpdate {
                    id: id.clone(),
                    width: None,
                    height: None,
                    status: "failed".into(),
                    error,
                });
                continue;
            }
        };
        let (width, height, result) = probe_thumbnail(Path::new(path), &dest);
        match result {
            Ok(()) => updates.push(ThumbUpdate {
                id: id.clone(),
                width,
                height,
                status: "ready".into(),
                error: String::new(),
            }),
            Err(error) => updates.push(ThumbUpdate {
                id: id.clone(),
                width,
                height,
                status: "failed".into(),
                error,
            }),
        }
    }
    let remaining = pending.len().saturating_sub(batch_size.min(pending.len()));
    (updates, remaining)
}

#[cfg(windows)]
fn write_shell_thumbnail(source: &Path, dest: &Path) -> Result<(), String> {
    let image = shell_thumbnail_rgba(source, 256)?;
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    image::DynamicImage::ImageRgba8(image)
        .thumbnail(THUMB_MAX, THUMB_MAX)
        .save_with_format(dest, image::ImageFormat::WebP)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(not(windows))]
fn write_shell_thumbnail(_source: &Path, _dest: &Path) -> Result<(), String> {
    Err("当前系统没有外壳缩略图".into())
}

#[cfg(windows)]
fn shell_thumbnail_rgba(source: &Path, edge: i32) -> Result<image::RgbaImage, String> {
    use std::os::windows::ffi::OsStrExt;
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::{HWND, SIZE};
    use windows::Win32::Graphics::Gdi::{
        DeleteObject, GetDC, GetDIBits, GetObjectW, ReleaseDC, BITMAP, BITMAPINFOHEADER, BI_RGB,
        DIB_RGB_COLORS,
    };
    use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
    use windows::Win32::UI::Shell::{IShellItemImageFactory, SHCreateItemFromParsingName, SIIGBF_RESIZETOFIT};

    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let wide: Vec<u16> = source.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
        let factory: IShellItemImageFactory =
            SHCreateItemFromParsingName(PCWSTR(wide.as_ptr()), None).map_err(|e| e.to_string())?;
        let hbmp = factory
            .GetImage(SIZE { cx: edge, cy: edge }, SIIGBF_RESIZETOFIT)
            .map_err(|e| e.to_string())?;
        let mut bitmap = BITMAP::default();
        let wrote = GetObjectW(hbmp, std::mem::size_of::<BITMAP>() as i32, Some(&mut bitmap as *mut _ as *mut _));
        if wrote == 0 {
            let _ = DeleteObject(hbmp);
            return Err("读取系统缩略图失败".into());
        }
        let width = bitmap.bmWidth.max(0) as u32;
        let height = bitmap.bmHeight.unsigned_abs();
        if width == 0 || height == 0 {
            let _ = DeleteObject(hbmp);
            return Err("系统缩略图尺寸为空".into());
        }
        let mut header = BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width as i32,
            biHeight: -(height as i32),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            ..Default::default()
        };
        let mut pixels = vec![0u8; (width as usize) * (height as usize) * 4];
        let hdc = GetDC(HWND::default());
        let lines = GetDIBits(
            hdc,
            hbmp,
            0,
            height,
            Some(pixels.as_mut_ptr() as *mut _),
            &mut header as *mut BITMAPINFOHEADER as *mut _,
            DIB_RGB_COLORS,
        );
        let _ = ReleaseDC(HWND::default(), hdc);
        let _ = DeleteObject(hbmp);
        if lines == 0 {
            return Err("系统没有返回缩略图".into());
        }
        for chunk in pixels.chunks_exact_mut(4) {
            chunk.swap(0, 2);
            chunk[3] = 255;
        }
        image::RgbaImage::from_raw(width, height, pixels).ok_or_else(|| "缩略图像素无效".to_string())
    }
}

pub fn copy_image_to_clipboard(path: &str) -> Result<(), String> {
    let source = Path::new(path);
    if exceeds_decode_budget(read_image_dimensions(source)) {
        return Err("这张图太大，剪贴板放不下，请打开原图后自行复制".into());
    }
    let img = ImageReader::open(path)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    clipboard
        .set_image(arboard::ImageData {
            width: width as usize,
            height: height as usize,
            bytes: rgba.into_raw().into(),
        })
        .map_err(|e| e.to_string())?;
    Ok(())
}
