use dirs;
use std::path::PathBuf;

pub fn get_archive_dir() -> PathBuf {
    get_home_dir().join(".archives")
}

pub fn get_home_dir() -> PathBuf {
    dirs::home_dir().expect("Cannot properly access the file system on your device.")
}

pub fn get_schedule_json_path() -> PathBuf {
    get_archive_dir().join(".scheduler.json")
}
