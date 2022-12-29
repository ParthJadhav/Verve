pub(crate) mod utils {
    use std::{
        fs,
        path::{Path, PathBuf},
        process::Command,
    };
    extern crate directories;
    use directories::ProjectDirs;
    use rust_search::SearchBuilder;
    extern crate plist;
    use auto_launch::AutoLaunchBuilder;
    use plist::Value;
    use serde::{Deserialize, Serialize};
    use std::time::Instant;
    use strsim::jaro_winkler;

    #[derive(Serialize, Deserialize)]
    struct Preferences {
        shortcut: String,
        launch_on_login: bool,
        menu_bar_icon: bool,
    }

    fn convert_and_store_icons(icns_path: &str, app_name: &str) {
        if let Some(proj_dirs) = ProjectDirs::from("com", "parth jadhav", "verve") {
            let icon_dir = proj_dirs.config_dir().join("appIcons");
            if fs::create_dir_all(&icon_dir).is_ok() {
                let icns_path = Path::new(icns_path);
                let png_path = icon_dir.join(app_name.to_owned() + &".png");
                if icns_path.exists() {
                    convert_icns_to_png(icns_path.to_str().unwrap(), &png_path);
                } else {
                    let icns_path = icns_path.to_str().unwrap();
                    let icns_path = &icns_path[..icns_path.len() - 5];
                    convert_icns_to_png(icns_path, &png_path);
                }
            };
        }
    }

    fn convert_icns_to_png(icns_path: &str, output_path: &PathBuf) {
        if !output_path.exists() {
            Command::new("sips")
                .arg("-s")
                .arg("format")
                .arg("png")
                .arg(icns_path)
                .arg("--out")
                .arg(output_path)
                .spawn()
                .expect("failed to execute process");
        }
    }

    fn get_icon_path(app_path: &str) -> String {
        let plist = Value::from_file(app_path.to_owned() + &"/Contents/Info.plist");
        match plist {
            Ok(plist) => {
                let icon_path = plist.as_dictionary().unwrap().get("CFBundleIconFile");
                if icon_path.is_some() {
                    let icon_path = icon_path.unwrap().as_string().unwrap();
                    return app_path.to_owned() + &"/Contents/Resources/" + icon_path + &".icns";
                } else {
                    return String::from("");
                }
            }
            Err(_) => {
                return String::from("");
            }
    }
}

    pub fn convert_all_app_icons_to_png() {
        let result: Vec<String> = SearchBuilder::default()
            .location("/Applications")
            .more_locations(vec![
                "/System/Applications",
                "/System/Applications/Utilities",
            ])
            .depth(1)
            .ext(".app")
            .ignore_case()
            .build()
            .collect();
        for app_path in result {
            let app_name = app_path.split("/").last().unwrap();
            let icon_path = get_icon_path(&app_path);
            if icon_path != "" {
                convert_and_store_icons(&icon_path, app_name);
            }
        }
    }

    pub fn create_preferences_if_missing() {
        if let Some(proj_dirs) = ProjectDirs::from("com", "parth jadhav", "verve") {
            let preferences_path = proj_dirs.config_dir().join("preferences.json");
            if !preferences_path.exists() {
                let preference = Preferences {
                    shortcut: String::from("Command+Shift+G"),
                    launch_on_login: true,
                    menu_bar_icon: true,
                };
                let preference_text = serde_json::to_string(&preference).unwrap();
                println!("{}", preference_text);
                fs::write(preferences_path, &preference_text).unwrap();
            }
        }
    }

    fn file_name_from_path(path: &str) -> String {
        let path = Path::new(path);
        let file_name = path.file_name().unwrap().to_str().unwrap();
        return file_name.to_string();
    }

    fn similarity_sort(vector: &mut Vec<String>, input: &str) {
        vector.sort_by(|a, b| {
            let input = input.to_lowercase();
            let a = file_name_from_path(a).to_lowercase();
            let b = file_name_from_path(b).to_lowercase();
            let a = jaro_winkler(a.as_str(), input.as_str());
            let b = jaro_winkler(b.as_str(), input.as_str());
            b.partial_cmp(&a).unwrap()
        });
    }

    fn search(input: &str, search_locations: Vec<&str>, extension: Option<&str>, depth: Option<usize>) -> Vec<String> {
        let (location, more_locations) = search_locations.split_first().unwrap();
        let mut result: Vec<String> = SearchBuilder::default()
            .search_input(input)
            .location(location)
            .more_locations(more_locations.to_vec())
            .depth(depth.unwrap_or(1))
            .ext(extension.unwrap_or("*"))
            .limit(5)
            .ignore_case()
            .build()
            .collect();
        similarity_sort(&mut result, input);
        return result;
    }

    #[tauri::command]
    pub async fn handle_input(input: String) -> (Vec<String>, f32) {
        let mut result: Vec<String>;
        let start_time = Instant::now();
        if !input.starts_with("/") {
            result = search(
                input.as_str(),
                vec![
                    "/Applications",
                    "/System/Applications",
                    "/System/Applications/Utilities",
                ],
                Some(".app"),
                Some(1),
            );
            similarity_sort(&mut result, input.as_str());
        } else {
                result = search(
                    input.trim_start_matches("/"),
                    vec![
                        "/Users/"
                    ],
                    None,
                    Some(10000)
                );
                println!("{:?}", result);
        }
        let time_taken = start_time.elapsed().as_secs_f32();
        return (result, time_taken);
    }


    #[tauri::command]
    pub fn get_icon(app_name: &str) -> String {
        if let Some(proj_dirs) = ProjectDirs::from("com", "parth jadhav", "verve") {
            let icon_dir = proj_dirs.config_dir().join("appIcons");
            let icon_path = icon_dir.join(app_name.to_owned() + &".png");
            if icon_path.exists() {
                return icon_path.to_str().unwrap().to_owned();
            } else {
                return String::from("");
            }
        } else {
            return String::from("");
        }
    }

    #[tauri::command]
    pub fn open_command(path: &str) {
        Command::new("open")
            .arg(path.trim())
            .spawn()
            .expect("failed to execute process");
    }

    #[tauri::command]
    pub fn launch_on_login(enable: bool) -> bool {
        let auto = AutoLaunchBuilder::new()
            .set_app_name("verve")
            .set_app_path("/Applications/verve.app")
            .build()
            .unwrap();

        if enable {
            match auto.enable() {
                Ok(_) => return true,
                Err(_) => {
                    println!("Failed");
                    false
                }
            }
        } else {
            match auto.disable() {
                Ok(_) => return true,
                Err(_) => return false,
            }
        }
    }
}
