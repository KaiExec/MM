mod utils;
use tauri::Manager;

#[tauri::command]
pub async fn import(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use std::fs::read;
    let default_path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Get Default Directory Failed: {}", e))?;

    use tauri_plugin_dialog::DialogExt;
    let Some(file_path) = app
        .dialog()
        .file()
        .set_directory(default_path)
        .blocking_pick_file()
    else {
        println!("Selecting Canceled");
        return Ok(None);
    };

    let content: String = String::from_utf8(
        // Convert into String

        // Get Content Bytes
        read(
            // Path type Transformation
            file_path
                .into_path()
                .map_err(|e| format!("Convert FilePath Into PathBuf Failed: {}", e))?,
        )
        .map_err(|e| format!("Read Failed {}", e))?,
    )
    .map_err(|e| format!("Convert into String Failed {}", e))?;

    Ok(Some(content))
}
