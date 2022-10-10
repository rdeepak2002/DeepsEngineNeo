use std::path::{Path, PathBuf};

pub fn blank_project_path() -> PathBuf {
    let current_path_ostr = std::env::current_dir().unwrap().into_os_string();
    return Path::new(current_path_ostr.to_str().unwrap())
        .join("examples")
        .join("blank");
}
