use tauri::{AppHandle, Manager};
mod icon;
mod process;
mod side_effect;

#[path = "Sundry/save.rs"]
mod save;

#[path = "Sundry/import.rs"]
mod import;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    fn reset_position(app: &AppHandle) {
        use tauri_plugin_positioner::{Position, WindowExt};
        if let Some(win) = app.get_webview_window("main") {
            let _ = win.as_ref().window().move_window(Position::TopRight);
        };
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            side_effect::trigger_side_effect,
            save::save,
            icon::icon,
            process::exit_suc,
            import::import
        ])
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin({
            use tauri_plugin_global_shortcut::{Builder, Code, Modifiers, Shortcut, ShortcutState};

            let hotkey_trigger =
                Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyN);
            let hotkey_reset =
                Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyR);
            let hotkeys = [hotkey_trigger, hotkey_reset];

            Builder::new()
                .with_shortcuts(hotkeys.clone())
                .unwrap()
                .with_handler(move |app, shortcut, event| {
                    if let Some(win) = app.get_webview_window("main") {
                        if shortcut == &hotkey_trigger {
                            match event.state() {
                                ShortcutState::Pressed => {}
                                ShortcutState::Released => {
                                    println!("Released: {:?}", shortcut);
                                    if win.is_visible().unwrap() {
                                        win.hide().expect("Window Hiding Error")
                                    } else {
                                        win.show().expect("Window Showing Error")
                                    }
                                }
                            }
                        }
                        if shortcut == &hotkey_reset {
                            match event.state() {
                                ShortcutState::Pressed => {}
                                ShortcutState::Released => {
                                    println!("Released: {:?}", shortcut);
                                    if win.is_visible().unwrap() {
                                        reset_position(app);
                                    }
                                }
                            }
                        }
                    }
                })
                .build()
        })
        .setup(|app| {
            // Hide Window until Initialized
            if let Some(win) = app.get_webview_window("main") {
                win.hide().expect("Window Hiding Error")
            }

            // Treat as Accessory
            // No way to control it with tauri.conf.json!
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Blur window
            {
                if let Some(win) = app.get_webview_window("main") {
                    #[cfg(target_os = "macos")]
                    {
                        use window_vibrancy::{
                            apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState,
                        };
                        apply_vibrancy(
                            &win,
                            NSVisualEffectMaterial::HudWindow,
                            Some(NSVisualEffectState::Active),
                            Some(0.0),
                        )
                        .expect("apply_vibrancy method Failed");
                    }
                    #[cfg(target_os = "windows")]
                    {
                        use window_vibrancy::apply_acrylic;
                        apply_acrylic(&win, Some((20, 20, 20, 10)))
                            .expect("Failed to apply acrylic effect");
                    }
                }
            }

            // Move to TopRight
            reset_position(app.handle());

            // Initialization Done
            {
                if let Some(win) = app.get_webview_window("main") {
                    win.show().expect("Window Showing Error")
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Function 'RUN in lib.rs' Error!");
}
