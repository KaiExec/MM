use ab_glyph::{FontVec, PxScale};
use image::{ImageFormat, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use std::{fs::read, io::Cursor};
#[tauri::command]
pub fn icon(
    app: tauri::AppHandle,
    text: String,
    font_path: String,
    color: [u8; 4],
    font_size: f32,
) -> Result<Vec<u8>, String> {
    let font_data =
        read(&font_path).map_err(|err| format!("Get font from {} Failed {}", &font_path, err))?;

    let font =
        FontVec::try_from_vec(font_data).map_err(|err| format!("Convert font Failed {}", err))?;
    let scale = PxScale::from(font_size);

    let (w, h) = text_size(1.0, &font, &text);
    let rgba_color = Rgba(color);

    // +4 For 2px padding
    let mut image = RgbaImage::new(w as u32 + 4, h as u32 + 4);
    draw_text_mut(&mut image, rgba_color, 2, 2, scale, &font, &text);
    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    Ok(bytes)
}
