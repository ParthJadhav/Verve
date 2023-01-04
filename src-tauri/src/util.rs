mod calculator;
mod icons;
mod preferences;
mod search;

extern crate directories;
extern crate plist;

use auto_launch::AutoLaunchBuilder;
use calculator::calculate;
use directories::ProjectDirs;
use std::{process::Command, time::Instant};

pub use icons::convert_all_app_icons_to_png;
pub use preferences::create_preferences_if_missing;
pub use search::{search, similarity_sort};

pub enum ResultType {
    Applications = 1,
    Files = 2,
    Calculation = 3,
}

#[tauri::command]
pub async fn handle_input(input: String) -> (Vec<String>, f32, i32) {
    let mut result: Vec<String>;
    let mut result_type: ResultType;
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
        result_type = ResultType::Applications;
    } else {
        result = search(
            input.trim_start_matches("/"),
            vec!["/Users/"],
            None,
            Some(10000),
        );
        println!("{:?}", result);
        result_type = ResultType::Files;
    }
    if result.len() == 0 {
        let calculation_result = calculate(input.as_str());
        if calculation_result != "" {
            result.push(calculation_result);
            result_type = ResultType::Calculation;
        }
    }
    let time_taken = start_time.elapsed().as_secs_f32();
    return (result, time_taken, result_type as i32);
}

#[tauri::command]
pub fn get_icon(app_name: &str) -> String {
    if let Some(proj_dirs) = ProjectDirs::from("com", "parth jadhav", "verve") {
        let icon_dir = proj_dirs.config_dir().join("appIcons");
        let icon_path = icon_dir.join(app_name.to_owned() + &".png");
        if icon_path.exists() {
            return icon_path.to_str().unwrap().to_owned();
        }
        return String::from("");
    }
    return String::from("");
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
