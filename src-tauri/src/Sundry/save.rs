use tauri::Manager;
mod utils;
use utils::Content;

#[tauri::command]
pub async fn save(app: tauri::AppHandle, content: Content, defaultly: bool) -> Result<(), String> {
    use sanitise_file_name::sanitise;
    use std::fs::{create_dir_all, exists, write};
    use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
    let safe_name = sanitise(&content.motivation);
    let base_name = if safe_name.is_empty() || safe_name.chars().all(|c| c == '_') {
        "Untitled".to_string()
    } else {
        safe_name
    };

    let file_path =
    // Default Path
    if defaultly {
        let data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Get Data Directory Error: {}", e))?;
        create_dir_all(&data_dir).map_err(|e| format!("Create {:?} Error: {}", data_dir, e))?;
        {
            let mut n = 0;
            loop {
                let file_name = if n == 0 {
                    format!("{}.mm", base_name)
                } else {
                    format!("{}({}).mm", base_name, n)
                };
                 let iter_path = data_dir.join(&file_name);
                if exists(&iter_path).map_err(|e| format!("Check file existence failed:{}", e))? {
                    n += 1;
                    continue;
                } else {
                    break iter_path;
                }
            }
        }
    }
    // Custom Path
    else {
        let file_path_opt = app
            .dialog()
            .file()
            .add_filter("My Filter", &["mm"])
            .set_file_name(&base_name)
            .blocking_save_file();

        match file_path_opt {
            Some(path) => {
                path
                    .into_path()
                    .map_err(|e| format!("Convert FilePath into PathBuf failed: {}", e))?
            }
            None => {
                println!("Saving Canceled");
                return Ok(());
            }
        }
    };

    let json_content = serde_json::to_string_pretty(&content)
        .map_err(|e| format!("Convert Json to string: {} failed", e))?;
    write(&file_path, json_content)
        .map_err(|e| format!("Write content in {:?} failed: {}", file_path, e))?;

    app.dialog()
        .message("So for so good.")
        .kind(MessageDialogKind::Info)
        .title("Save Successfully")
        .blocking_show();
    Ok(())
}
