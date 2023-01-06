use directories::ProjectDirs;
use plist::Value;
use rust_search::SearchBuilder;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

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
            }

            return String::from("");
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
