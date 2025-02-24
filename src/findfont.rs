#[cfg(not(target_os = "macos"))]
use std::env;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
fn split_path_list(path_list: &str) -> Vec<String> {
    #[cfg(target_os = "windows")]
    {
        path_list.split(';').map(|s| s.to_string()).collect()
    }

    #[cfg(not(target_os = "windows"))]
    {
        path_list.split(':').map(|s| s.to_string()).collect()
    }
}

#[cfg(target_os = "linux")]
fn get_linux_sys_font_dirs() -> Vec<PathBuf> {
    let mut font_dirs: Vec<PathBuf> = vec![
        PathBuf::from("~/.fonts/"),
        PathBuf::from("~/.local/share/fonts/"),
        PathBuf::from("/usr/local/share/fonts/"),
        PathBuf::from("/usr/share/fonts/"),
    ];

    if let Ok(data_path) = env::var("XDG_DATA_HOME") {
        if !data_path.is_empty() {
            font_dirs.push(PathBuf::from(data_path).join("fonts"));
        }
    }

    if let Ok(data_path) = env::var("XDG_DATA_DIRS") {
        font_dirs.extend(
            split_path_list(&data_path)
                .iter()
                .map(|p| PathBuf::from(p).join("fonts"))
                .collect::<Vec<PathBuf>>(),
        );
    }

    font_dirs
}

fn get_sys_font_dirs() -> Vec<PathBuf> {
    #[cfg(target_os = "macos")]
    let dirs = vec![
        PathBuf::from("~/Library/Fonts/"),
        PathBuf::from("/Library/Fonts/"),
        PathBuf::from("/System/Library/Fonts/"),
        PathBuf::from("/System/Library/Fonts/Supplemental"),
        PathBuf::from("/System/Library/AssetsV2/com_apple_MobileAsset_Font7/3419f2a427639ad8c8e139149a287865a90fa17e.asset/AssetData")
    ];

    #[cfg(target_os = "windows")]
    let dirs = vec![
        PathBuf::from(env::var("windir").unwrap()).join("Fonts"),
        PathBuf::from(env::var("localappdata").unwrap())
            .join("Microsoft")
            .join("Windows")
            .join("Fonts"),
    ];

    #[cfg(target_os = "linux")]
    let dirs = get_linux_sys_font_dirs();

    dirs
}

pub fn find(font_name: &str) -> Option<PathBuf> {
    let exts = ["ttf", "ttc", "otf"];
    let variant = ["Light", "Medium"];

    for dir in get_sys_font_dirs().iter() {
        for ext in exts.iter() {
            let font_path = dir.join(format!("{}.{}", font_name, ext));
            if font_path.exists() {
                return Some(font_path);
            }

            for var in variant.iter() {
                let font_path = dir.join(format!("{} {}.{}", font_name, var, ext));
                if font_path.exists() {
                    return Some(font_path);
                }
            }
        }
    }

    None
}
