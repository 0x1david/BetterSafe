use std::path::PathBuf;

use dirs;
use serde::{Deserialize, Serialize};
pub fn get_archive_dir() -> String {
    dirs::home_dir()
        .expect("Cannot properly acces the file system on your device.")
        .join(".archives")
        .to_string_lossy()
        .to_string()
}

pub fn get_home_dir_str() -> String {
    dirs::home_dir()
        .expect("Cannot properly access the file system on your device.")
        .to_string_lossy()
        .to_string()
}

pub fn get_schedule_json_path() -> PathBuf {
    let mut schedule_json_path = get_archive_dir();
    schedule_json_path.push_str(".scheduler.json");
    schedule_json_path.into()
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FileType {
    Folder,
    File,
}

pub enum ArchiveDuration {
    Short,
    Medium,
    Long,
}

impl ArchiveDuration {
    pub fn get_duration(&self) -> std::time::Duration {
        match self {
            ArchiveDuration::Short => std::time::Duration::from_secs(60 * 60 * 24), // 24 Hours
            ArchiveDuration::Medium => std::time::Duration::from_secs(60 * 60 * 24 * 3), // 3 Days
            ArchiveDuration::Long => std::time::Duration::from_secs(60 * 60 * 24 * 24 * 14), // 14 Days
        }
    }
}
