#[tauri::command]
pub fn exit_suc(app: tauri::AppHandle) {
    app.exit(0);
}
