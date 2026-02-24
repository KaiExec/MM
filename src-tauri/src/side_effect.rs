#[tauri::command]
pub async fn trigger_side_effect(win: tauri::WebviewWindow, is_expand: bool) {
    use tauri::{LogicalPosition, LogicalSize, Position, Size};

    if let (Ok(size), Ok(postion)) = (win.inner_size(), win.outer_position()) {
        let buffer = 100.0;
        let logical_size = size.to_logical(win.scale_factor().unwrap());
        let logical_position = postion.to_logical(win.scale_factor().unwrap());
        let (new_logical_size, new_logical_position): (LogicalSize<f64>, LogicalPosition<f64>) =
            if is_expand {
                (
                    LogicalSize {
                        width: logical_size.width - buffer,
                        height: logical_size.height,
                    },
                    LogicalPosition {
                        x: logical_position.x + buffer,
                        y: logical_position.y,
                    },
                )
            } else {
                (
                    LogicalSize {
                        width: logical_size.width + buffer,
                        height: logical_size.height,
                    },
                    LogicalPosition {
                        x: logical_position.x - buffer,
                        y: logical_position.y,
                    },
                )
            };
        // Set position
        win.set_position(Position::Logical(new_logical_position))
            .expect("Convert Position error in 'invoke.rs'");
        println!("New_position:{:?}", new_logical_position);

        // Extend width
        win.set_size(Size::Logical(new_logical_size))
            .expect("Convert Size error in 'invoke.rs'");
        println!("New_size:{:?}", new_logical_size);
    }
}
