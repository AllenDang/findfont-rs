#[cfg(not(target_os = "macos"))]
use std::env;
use std::fs;
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

#[cfg(target_os = "macos")]
fn get_macos_sys_font_dirs() -> Vec<PathBuf> {
    const SYS_FONT_PATHS: &[&str] = &[
        "/Library/Fonts/",
        "/System/Library/Fonts/",
        "/System/Library/Fonts/Supplemental",
        "/System/Library/AssetsV2/com_apple_MobileAsset_Font7/3419f2a427639ad8c8e139149a287865a90fa17e.asset/AssetData",
    ];

    let mut font_dirs = Vec::with_capacity(5);

    if let Some(home_dir) = dirs::home_dir() {
        font_dirs.push(home_dir.join("Library/Fonts/"));
    }
    font_dirs.extend(SYS_FONT_PATHS.iter().map(|p| PathBuf::from(*p)));

    font_dirs
}

#[cfg(target_os = "linux")]
fn get_linux_sys_font_dirs() -> Vec<PathBuf> {
    const SYS_FONT_PATHS: &[&str] = &["/usr/share/fonts/", "/usr/local/share/fonts/"];

    let mut font_dirs = Vec::with_capacity(6);

    if let Some(home) = dirs::home_dir() {
        font_dirs.push(home.join(".fonts/"));
        font_dirs.push(home.join(".local/share/fonts/"));
    }

    font_dirs.extend(SYS_FONT_PATHS.iter().map(|p| PathBuf::from(*p)));

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

///
/// Will return all font sys directories that are known for the current platform
///
/// # Examples
/// ```
/// let dirs = findfont::get_sys_font_dirs();
///
/// assert!(dirs.len() > 0, "Seems like your system is not supported or it does not have known font directories :(");
/// ```
pub fn get_sys_font_dirs() -> Vec<PathBuf> {
    #[cfg(target_os = "macos")]
    let dirs = get_macos_sys_font_dirs();

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

///
/// Will return the first font sys directory that exists
///
/// # Examples
/// ```
/// let dir = findfont::get_sys_font_dir();
///
/// assert!(dir.is_some(), "Seems like your system is not supported or it does not have known font directory :(");
/// ```
pub fn get_sys_font_dir() -> Option<PathBuf> {
    get_sys_font_dirs()
        .into_iter()
        .find(|path| std::path::Path::new(path).exists())
}

/// Will return the path to the requested font in system font directories (font file included).
/// [`None`] will be returned in case the font is not found.
///
/// **Note:** `font_name` should only have the font name without file extensions or spaces.
///
/// # Examples
///
/// ```
/// let font_name = "Arial";
/// let font = findfont::find(font_name);
/// ```
///
pub fn find(font_name: &str) -> Option<PathBuf> {
    let exts = ["ttf", "ttc", "otf"];
    let variants = ["Light", "Medium"];

    for dir in get_sys_font_dirs().iter() {
        if let Some(found_path) = find_font_in_dir_recursive(dir, font_name, &exts, &variants) {
            return Some(found_path);
        }
    }

    None
}

/// Recursively searches for a font file in the given directory and its subdirectories.
///
/// Note: This will not follow symlinks to avoid potential infinite loops.
fn find_font_in_dir_recursive(
    dir_path: &PathBuf,
    font_name: &str,
    exts: &[&str],
    variants: &[&str],
) -> Option<PathBuf> {
    let Ok(entries) = fs::read_dir(dir_path) else {
        return None;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();

        if path.is_dir() && !path.is_symlink() {
            if let Some(found_path) = find_font_in_dir_recursive(&path, font_name, exts, variants) {
                return Some(found_path);
            }
        } else if path.is_file() {
            let file_stem = path.file_stem().and_then(|s| s.to_str());
            let file_ext = path.extension().and_then(|s| s.to_str());

            if let Some((file_stem, file_ext)) = file_stem.zip(file_ext) {
                for ext in exts {
                    if !file_ext.eq_ignore_ascii_case(ext) {
                        continue;
                    }

                    if file_stem.eq_ignore_ascii_case(font_name) {
                        return Some(path);
                    }

                    for var in variants {
                        let variant_font_name = format!("{}-{}", font_name, var);
                        if file_stem.eq_ignore_ascii_case(&variant_font_name) {
                            return Some(path);
                        }
                    }
                }
            }
        }
    }
    None
}
