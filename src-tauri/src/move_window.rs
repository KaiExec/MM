#[tauri::command]
pub fn move_window(direction: char, win: tauri::WebviewWindow) {
    if let (Ok(postion), Ok(scale_factor)) = (win.outer_position(), win.scale_factor()) {
        let move_length = 40.0;
        let mut new_position = postion.to_logical::<f64>(scale_factor);

        match direction {
            'u' => new_position.x -= move_length,
            'i' => new_position.y += move_length,
            'o' => new_position.y -= move_length,
            'p' => new_position.x += move_length,
            _ => {}
        }
        let _ = win.set_position(new_position);
    }
}
