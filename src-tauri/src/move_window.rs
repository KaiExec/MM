use tauri::LogicalPosition;

#[tauri::command]
pub fn move_window(direction: char, win: tauri::WebviewWindow) {
    if let (Ok(pos), Ok(scale), Ok(size), Ok(Some(monitor))) = (
        win.outer_position(),
        win.scale_factor(),
        win.outer_size(),
        win.current_monitor(),
    ) {
        let move_length = 20.0;

        let mut current_logic_pos = pos.to_logical::<f64>(scale);
        let logic_size = size.to_logical::<f64>(scale);

        let rect = monitor.work_area();
        let work_pos = rect.position.to_logical::<f64>(scale);
        let work_size = rect.size.to_logical::<f64>(scale);

        match direction {
            'u' => current_logic_pos.x -= move_length,
            'p' => current_logic_pos.x += move_length,
            'o' => current_logic_pos.y -= move_length,
            'i' => current_logic_pos.y += move_length,
            _ => return,
        }

        let final_x = current_logic_pos
            .x
            .clamp(work_pos.x, work_pos.x + work_size.width - logic_size.width);

        let final_y = current_logic_pos.y.clamp(
            work_pos.y,
            work_pos.y + work_size.height - logic_size.height,
        );

        let _ = win.set_position(LogicalPosition::new(final_x, final_y));
    }
}
