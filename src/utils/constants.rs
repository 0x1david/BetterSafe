use dirs;
use std::path::PathBuf;

pub fn get_archive_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Unable to determine home directory")
        .join(".archives")
}
