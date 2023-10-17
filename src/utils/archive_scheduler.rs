use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::exit;

use super::constants::{get_schedule_json_path, ArchiveDuration, FileType};

#[derive(Serialize, Deserialize)]
pub struct ArchiveScheduler {
    //TODO: Create second schedule where the structure is Path: Datetime to make restore way more
    //efficient
    schedule: BTreeMap<DateTime<Utc>, (String, FileType)>,
    json_path: PathBuf,
}

impl ArchiveScheduler {
    pub fn build() -> io::Result<Self> {
        let json_path = get_schedule_json_path();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&json_path)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let schedule = if content.is_empty() {
            BTreeMap::new()
        } else {
            serde_json::from_str(&content).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to deserialize schedule: {}", e),
                )
            })?
        };

        Ok(Self {
            schedule,
            json_path,
        })
    }

    pub fn insert_record(&mut self, record: Record) {
        self.schedule
            .insert(record.date, (record.path, record.file_type));
    }
    pub fn delete_record(&mut self, record_path: &str) {
        let target = match Self::find_by_path(&self, &record_path) {
            Some(target) => target,
            None => {
                eprintln!("Archive record not found, you can ignore this message.");
                exit(1);
            }
        };

        match self.schedule.remove(&target) {
            Some(_) => (),
            None => {
                eprintln!("Couldn't remove an existing record, aborting");
                exit(1);
            }
        }
    }

    fn find_by_path(&self, path: &str) -> Option<DateTime<Utc>> {
        let found = self.schedule.iter().find_map(|(key, (searched_path, _))| {
            if path == searched_path {
                Some(*key)
            } else {
                None
            }
        });
        found
    }
    pub fn save_json(&self) -> serde_json::Result<()> {
        let json = serde_json::to_string_pretty(&self.schedule)?;
        fs::write(&self.json_path, json).expect("TODO:");
        Ok(())
    }

    pub fn debug_print_records(&self) {
        for (key, (value, _)) in &self.schedule {
            println!("{}: {}", key, value);
        }
    }
    pub fn handle_due_records(&mut self) {
        let now = Utc::now();
        let keys: Vec<_> = self.schedule.range(..now).map(|(k, _)| *k).collect();

        for key in keys {
            if let Some((path, file_type)) = self.schedule.remove(&key) {
                Self::permanently_delete_file(file_type, path)
                    .unwrap_or_else(|e| eprintln!("Failed to delete: {}", e));
            }
        }
    }

    pub fn permanently_delete_file(file_type: FileType, path: String) -> io::Result<()> {
        match file_type {
            FileType::File => fs::remove_file(path),
            FileType::Folder => fs::remove_dir_all(path),
        }
    }
}

pub struct Record {
    date: DateTime<Utc>,
    path: String,
    file_type: FileType,
}

impl Record {
    pub fn new(duration: ArchiveDuration, path: String, file_type: FileType) -> Self {
        Self {
            date: Utc::now() + duration.get_duration(),
            path,
            file_type,
        }
    }
}
