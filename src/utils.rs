use dirs::home_dir;
use std::path::PathBuf;

pub fn pp_path(path: &PathBuf) -> String {
    if let Some(home_dir) = home_dir() {
        if path.starts_with(&home_dir) {
            let display_path = path.strip_prefix(&home_dir).unwrap_or(path);
            return format!("~/{}", display_path.display());
        }
    }
    if let Some(path_str) = path.to_str() {
        if path_str.starts_with("./") {
            return format!("{}", path_str[2..].to_string());
        }
        return path_str.to_string();
    }
    return format!("{}", path.display());
}

pub fn pp_size(size: u64) -> String {
    let kb = size / 1024;
    let mb = kb / 1024;
    let gb = mb / 1024;
    if gb > 0 {
        return format!("{}GB", gb);
    } else if mb > 0 {
        return format!("{}MB", mb);
    } else if kb > 0 {
        return format!("{}KB", kb);
    } else {
        return format!("{}B", size);
    }
}
