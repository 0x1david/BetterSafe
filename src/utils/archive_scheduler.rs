use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, ErrorKind};
use std::io::{self, Read};
use std::path::PathBuf;

use super::constants::get_schedule_json_path;

#[derive(Serialize, Deserialize)]
pub struct ArchiveScheduler {
    schedule: BTreeMap<String, Vec<String>>,
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
}
