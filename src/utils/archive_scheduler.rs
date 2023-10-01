use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Read};
use std::path::PathBuf;

use super::constants::{get_schedule_json_path, FileType};

#[derive(Serialize, Deserialize)]
pub struct ArchiveScheduler {
    schedule: BTreeMap<DateTime<Utc>, (String, FileType)>,
    json_path: PathBuf,
}

impl ArchiveScheduler {
    pub fn new() -> io::Result<Self> {
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
        self.schedule.insert(record.date, (record.path, record.file_type));
    }
    
    pub fn save_json(&self) -> serde_json::Result<()> {
        let json = serde_json::to_string_pretty(&self.schedule)?;
        fs::write(&self.json_path, json).expect("TODO:");
        Ok(())
    }
    
    pub fn handle_due_records(&mut self) {
        let now = Utc::now();
        let keys: Vec<_> = self.schedule.range(..now).map(|(k, _)| *k).collect();
        
        for key in keys {
            if let Some((path, file_type)) = self.schedule.remove(&key) {
                Self::permanently_delete_file(file_type, path).unwrap_or_else(|e| eprintln!("Failed to delete: {}", e));
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
    pub fn new(date: DateTime<Utc>, path: String, file_type: FileType) -> Self {
        Self {
            date,
            path,
            file_type,
        }
    }
}

