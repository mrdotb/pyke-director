use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SpectatorEndpoint {
    pub base_url: String,
    pub platform_id: String,
}

impl SpectatorEndpoint {
    pub fn new(base_url: String, platform_id: String) -> Self {
        SpectatorEndpoint {
            base_url,
            platform_id,
        }
    }
}

impl fmt::Display for SpectatorEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Base Url: {}\n", self.base_url)?;
        write!(f, "Platform Id: {}\n", self.platform_id)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum Region {
    KR,
    EUW1,
    NA1,
}

impl Region {
    pub fn to_endpoint(&self) -> SpectatorEndpoint {
        SpectatorEndpoint {
            base_url: self.base_url(),
            platform_id: self.platform_id(),
        }
    }
    fn platform_id(&self) -> String {
        self.to_string().to_uppercase()
    }
    fn base_url(&self) -> String {
        format!("http://spectator-consumer.{}.lol.pvp.net:80", self)
    }
}

impl FromStr for Region {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kr" => Ok(Region::KR),
            "euw1" => Ok(Region::EUW1),
            "na1" => Ok(Region::NA1),
            _ => Err(format!("'{}' is not a valid region", s)),
        }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let region_str = match self {
            Region::KR => "kr",
            Region::EUW1 => "euw1",
            Region::NA1 => "na1",
        };
        write!(f, "{}", region_str)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameMetaData {
    pub game_key: GameKey,
    pub game_server_address: String,
    pub port: u32,
    pub encryption_key: String,
    pub chunk_time_interval: u32,
    pub start_time: String,
    pub game_ended: bool,
    pub last_chunk_id: u32,
    pub last_key_frame_id: u32,
    pub end_startup_chunk_id: u32,
    pub delay_time: u32,
    pub pending_available_chunk_info: Vec<PendingAvailableChunkInfo>,
    pub pending_available_key_frame_info: Vec<PendingAvailableKeyFrameInfo>,
    pub key_frame_time_interval: u64,
    pub decoded_encryption_key: String,
    pub start_game_chunk_id: u32,
    pub game_length: u32,
    pub client_added_lag: u32,
    pub client_back_fetching_enabled: bool,
    pub client_back_fetching_freq: u32,
    pub interest_score: u32,
    pub featured_game: bool,
    pub create_time: String,
    pub end_game_chunk_id: i32,
    pub end_game_key_frame_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameKey {
    pub game_id: u64,
    pub platform_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingAvailableChunkInfo {
    pub chunk_id: u32,
    pub duration: u32,
    pub received_time: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingAvailableKeyFrameInfo {
    pub key_frame_id: u32,
    pub received_time: String,
    pub next_chunk_id: u32,
}

impl fmt::Display for GameMetaData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Game Key: {:?}\n", self.game_key)?;
        write!(f, "Server Address: {}\n", self.game_server_address)?;
        write!(f, "Port: {}\n", self.port)?;
        write!(f, "Encryption Key: {}\n", self.encryption_key)?;
        write!(f, "Chunk Time Interval: {}\n", self.chunk_time_interval)?;
        write!(f, "Start Time: {}\n", self.start_time)?;
        write!(f, "Game Ended: {}\n", self.game_ended)?;
        write!(f, "Last Chunk ID: {}\n", self.last_chunk_id)?;
        write!(f, "Last Key Frame ID: {}\n", self.last_key_frame_id)?;
        write!(f, "End Startup Chunk ID: {}\n", self.end_startup_chunk_id)?;
        write!(f, "Delay Time: {}\n", self.delay_time)?;
        write!(
            f,
            "Key Frame Time Interval: {}\n",
            self.key_frame_time_interval
        )?;
        write!(
            f,
            "Decoded Encryption Key: {}\n",
            self.decoded_encryption_key
        )?;
        write!(f, "Start Game Chunk ID: {}\n", self.start_game_chunk_id)?;
        write!(f, "Game Length: {}\n", self.game_length)?;
        write!(f, "Client Added Lag: {}\n", self.client_added_lag)?;
        write!(
            f,
            "Client Back Fetching Enabled: {}\n",
            self.client_back_fetching_enabled
        )?;
        write!(
            f,
            "Client Back Fetching Freq: {}\n",
            self.client_back_fetching_freq
        )?;
        write!(f, "Interest Score: {}\n", self.interest_score)?;
        write!(f, "Featured Game: {}\n", self.featured_game)?;
        write!(f, "Create Time: {}\n", self.create_time)?;
        write!(f, "End Game Chunk ID: {}\n", self.end_game_chunk_id)?;
        write!(f, "End Game Key Frame ID: {}\n", self.end_game_key_frame_id)?;

        write!(f, "Pending Available Chunk Info:\n")?;
        for chunk_info in &self.pending_available_chunk_info {
            write!(
                f,
                "\tChunk ID: {}, Duration: {}, Received Time: {}\n",
                chunk_info.chunk_id, chunk_info.duration, chunk_info.received_time
            )?;
        }

        write!(f, "Pending Available Key Frame Info:\n")?;
        for key_frame_info in &self.pending_available_key_frame_info {
            write!(
                f,
                "\tKey Frame ID: {}, Received Time: {}, Next Chunk ID: {}\n",
                key_frame_info.key_frame_id,
                key_frame_info.received_time,
                key_frame_info.next_chunk_id
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChunkInfo {
    pub chunk_id: u32,
    pub available_since: u64,
    pub next_available_chunk: u32,
    pub key_frame_id: u32,
    pub next_chunk_id: u32,
    pub end_startup_chunk_id: u32,
    pub start_game_chunk_id: u32,
    pub end_game_chunk_id: u32,
    pub duration: u32,
}

impl fmt::Display for ChunkInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk ID: {}\n", self.chunk_id)?;
        write!(f, "Available Since: {}\n", self.available_since)?;
        write!(f, "Next Available Chunk: {}\n", self.next_available_chunk)?;
        write!(f, "Key Frame ID: {}\n", self.key_frame_id)?;
        write!(f, "Next Chunk ID: {}\n", self.next_chunk_id)?;
        write!(f, "End Startup Chunk ID: {}\n", self.end_startup_chunk_id)?;
        write!(f, "Start Game Chunk ID: {}\n", self.start_game_chunk_id)?;
        write!(f, "End Game Chunk ID: {}\n", self.end_game_chunk_id)?;
        write!(f, "Duration: {}\n", self.duration)?;
        Ok(())
    }
}
