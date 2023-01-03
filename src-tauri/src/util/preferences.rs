use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;

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

#[derive(Serialize, Deserialize)]
struct Preferences {
    shortcut: String,
    launch_on_login: bool,
    menu_bar_icon: bool,
}

pub fn create_preferences_if_missing() {
    if let Some(proj_dirs) = ProjectDirs::from("com", "parth jadhav", "verve") {
        let preferences_path = proj_dirs.config_dir().join("preferences.json");
        let theme_path = proj_dirs.config_dir().join("theme.json");
        if !preferences_path.exists() {
            let preference = Preferences {
                shortcut: String::from("Command+Shift+G"),
                launch_on_login: true,
                menu_bar_icon: true,
            };
            let preference_text = serde_json::to_string(&preference).unwrap();
            fs::write(preferences_path, &preference_text).unwrap();
        }
        if !theme_path.exists() {
            let theme = Theme {
                primary_bg_color: String::from("rgba(20, 20, 30, 0.6)"),
                secondary_bg_color: String::from("rgba(84, 101, 115, 0.6)"),
                primary_text_color: String::from("#FFFFFF"),
                secondary_text_color: String::from("#878787"),
                primary_accent_color: String::from("#556CE5"),
                secondary_accent_color: String::from("#48A5FF"),
                highlight_overlay: String::from("rgba(255, 255, 255, 0.1)"),
                dark_overlay: String::from("rgba(0, 0, 0, 0.1)"),
            };
            let theme_text = serde_json::to_string(&theme).unwrap();
            fs::write(theme_path, &theme_text).unwrap();
        }
    }
}
