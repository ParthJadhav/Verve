use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::create_dir_all;
use tauri::api::path::app_data_dir;
use tauri::Config;

#[derive(Serialize, Deserialize)]
struct Theme {
    primary_bg_color: String,
    secondary_bg_color: String,
    primary_text_color: String,
    secondary_text_color: String,
    primary_accent_color: String,
    secondary_accent_color: String,
    highlight_overlay: String,
    dark_overlay: String,
}

trait Linux {
    fn get_theme_linux() -> Theme;
}

trait MacOS {
    fn get_theme_mac_os() -> Theme;
}

impl MacOS for Theme {
    fn get_theme_mac_os() -> Self {
        Self {
            primary_bg_color: String::from("rgba(20, 20, 30, 0.6)"),
            secondary_bg_color: String::from("rgba(84, 101, 115, 0.6)"),
            primary_text_color: String::from("#FFFFFF"),
            secondary_text_color: String::from("#878787"),
            primary_accent_color: String::from("#556CE5"),
            secondary_accent_color: String::from("#48A5FF"),
            highlight_overlay: String::from("rgba(255, 255, 255, 0.1)"),
            dark_overlay: String::from("rgba(0, 0, 0, 0.1)"),
        }
    }
}

impl Linux for Theme {
    fn get_theme_linux() -> Theme {
        Self {
            primary_bg_color: String::from("rgba(20, 20, 30, 1)"),
            secondary_bg_color: String::from("rgba(84, 101, 115, 1)"),
            primary_text_color: String::from("#FFFFFF"),
            secondary_text_color: String::from("#878787"),
            primary_accent_color: String::from("#556CE5"),
            secondary_accent_color: String::from("#48A5FF"),
            highlight_overlay: String::from("rgba(255, 255, 255, 0.1)"),
            dark_overlay: String::from("rgba(0, 0, 0, 0.1)"),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Preferences {
    shortcut: String,
    launch_on_login: bool,
    menu_bar_icon: bool,
}

pub fn create_preferences_if_missing() {
    if let Some(proj_dirs) = app_data_dir(&Config::default()) {
        let preferences_path = proj_dirs.join("com.parth-jadhav.verve/preferences.json");
        let theme_path = proj_dirs.join("com.parth-jadhav.verve/theme.json");
        println!("{:?}", preferences_path);
        if !preferences_path.exists() {
            let preference = Preferences {
                shortcut: String::from("CTRL+Shift+G"),
                launch_on_login: true,
                menu_bar_icon: true,
            };
            create_dir_all(&preferences_path.parent().unwrap()).unwrap();
            let preference_text = serde_json::to_string(&preference).unwrap();
            fs::write(preferences_path, &preference_text).unwrap();
        }
        if !theme_path.exists() {
            #[cfg(target_os = "macos")]
            let theme = Theme::get_theme_mac_os();
            #[cfg(target_os = "linux")]
            let theme = Theme::get_theme_linux();
            let theme_text = serde_json::to_string(&theme).unwrap();
            fs::write(theme_path, &theme_text).unwrap();
        }
    }
}
