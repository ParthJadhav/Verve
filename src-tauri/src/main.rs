#![warn(clippy::nursery, clippy::pedantic)]

mod util;
mod ns_panel;

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

#[allow(unused_imports)]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use util::{
    convert_all_app_icons_to_png, create_preferences_if_missing, get_icon, handle_input,
    launch_on_login, open_command,
};

fn create_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("Quit".to_string(), "Quit");
    let show = CustomMenuItem::new("Show".to_string(), "Show");
    let hide = CustomMenuItem::new("Hide".to_string(), "Hide");
    let preferences = CustomMenuItem::new("Preferences".to_string(), "Preferences");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_item(preferences)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

fn main() {
    convert_all_app_icons_to_png();
    create_preferences_if_missing();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_command,
            get_icon,
            handle_input,
            launch_on_login,
            ns_panel::init_ns_panel,
            ns_panel::show_app,
            ns_panel::hide_app
        ])
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let window = app.get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, Some(10.0))
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            window.hide().unwrap();
            Ok(())
        })
        .manage(ns_panel::State::default())
        .system_tray(create_system_tray())
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "Hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "Show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.center().unwrap();
                }
                "Preferences" => {
                    let window = app.get_window("main").unwrap();
                    window.emit("PreferencesClicked", Some("Yes")).unwrap();
                    window.show().unwrap();
                    window.center().unwrap();
                }
                "Quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
