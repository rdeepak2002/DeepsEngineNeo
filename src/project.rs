use std::fs;
use std::path::{Path, PathBuf};

pub fn blank_project_path() -> PathBuf {
    let current_path_ostr = std::env::current_dir().unwrap().into_os_string();

    // let paths = fs::read_dir(current_path_ostr).unwrap();

    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }
    //
    // return Path::new("./").to_path_buf();

    return Path::new(current_path_ostr.to_str().unwrap())
        .join("examples")
        .join("blank");
}
