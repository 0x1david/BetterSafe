use dirs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
pub fn get_archive_dir() -> PathBuf {
    dirs::home_dir().expect("Cannot properly acces the file system on your device.").join(".archives")
}

pub fn get_home_dir_str() -> String {
        dirs::home_dir().expect("Cannot properly access the file system on your device.").to_string_lossy().to_string()
}

pub fn get_schedule_json_path() -> PathBuf {
    get_archive_dir().join(".scheduler.json")
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

