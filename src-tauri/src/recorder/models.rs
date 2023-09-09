use super::api::models::{GameMetaData, SpectatorEndpoint};

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Record {
    pub version: String,
    pub endpoint: SpectatorEndpoint,
    pub game_id: String,
    pub encryption_key: String,
    pub metadata: Option<GameMetaData>,
    pub storage_path: PathBuf,
    pub keyframes: Mutex<HashSet<u32>>,
    pub game_data_chunks: Mutex<HashSet<u32>>,
}

impl Record {
    pub fn new(
        version: String,
        endpoint: SpectatorEndpoint,
        game_id: String,
        encryption_key: String,
        base_path: PathBuf,
    ) -> Result<Self, io::Error> {
        let storage_path = base_path.join(format!("{}_{}", endpoint.platform_id, game_id));
        Self::create_dir_if_not_exists(storage_path.join("game_data_chunks"))?;
        Self::create_dir_if_not_exists(storage_path.join("keyframes"))?;

        Ok(Record {
            version,
            endpoint,
            game_id,
            encryption_key,
            metadata: None,
            keyframes: Mutex::new(HashSet::new()),
            game_data_chunks: Mutex::new(HashSet::new()),
            storage_path,
        })
    }

    fn create_dir_if_not_exists(path: PathBuf) -> Result<(), io::Error> {
        match fs::create_dir_all(&path) {
            Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()), // If it already exists, just return Ok
            other => other,
        }
    }

    pub fn has_game_data_chunk(&self, chunk_id: u32) -> bool {
        self.game_data_chunks.lock().unwrap().contains(&chunk_id)
    }

    pub fn insert_game_data_chunk(&self, chunk_id: u32) {
        self.game_data_chunks.lock().unwrap().insert(chunk_id);
    }

    pub fn has_keyframe(&self, chunk_id: u32) -> bool {
        self.keyframes.lock().unwrap().contains(&chunk_id)
    }

    pub fn insert_keyframe(&self, chunk_id: u32) {
        self.keyframes.lock().unwrap().insert(chunk_id);
    }

    pub fn store_game_data_chunk(&self, chunk_id: u32, data: Vec<u8>) -> Result<(), io::Error> {
        let path = self
            .storage_path
            .join(format!("game_data_chunks/{}", chunk_id));

        fs::write(path, data)
    }

    pub fn store_key_frame(&self, keyframe_id: u32, data: Vec<u8>) -> Result<(), io::Error> {
        let path = self.storage_path.join(format!("keyframes/{}", keyframe_id));

        fs::write(path, data)
    }
}

impl Serialize for Record {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Record", 8)?;

        state.serialize_field("version", &self.version)?;
        state.serialize_field("endpoint", &self.endpoint)?;
        state.serialize_field("game_id", &self.game_id)?;
        state.serialize_field("encryption_key", &self.encryption_key)?;
        state.serialize_field("storage_path", &self.storage_path)?;

        // Sorting keyframes in ascending order
        state.serialize_field("keyframes", &{
            let mut sorted = self
                .keyframes
                .lock()
                .unwrap()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            sorted.sort();
            sorted
        })?;

        // Sorting game_data_chunks in ascending order
        state.serialize_field("game_data_chunks", &{
            let mut sorted = self
                .game_data_chunks
                .lock()
                .unwrap()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            sorted.sort();
            sorted
        })?;

        state.end()
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Version: {}\n", self.version)?;
        write!(f, "Game Id: {}\n", self.game_id)?;
        write!(f, "Encryption Key: {}\n", self.game_id)?;

        Ok(())
    }
}
