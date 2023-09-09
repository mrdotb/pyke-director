use super::api::endpoints;
use super::api::models::SpectatorEndpoint;
use super::error::RecordingError;
use super::models::Record;

use log::debug;
use tokio::spawn;
use tokio::time::{sleep, Duration};

use std::path::PathBuf;
use std::sync::Arc;

pub async fn new(
    endpoint: SpectatorEndpoint,
    game_id: String,
    encryption_key: String,
    storage_path: PathBuf,
) -> Result<Record, RecordingError> {
    let version = endpoints::fetch_api_version(&endpoint).await?;
    let mut record = Record::new(version, endpoint, game_id, encryption_key, storage_path)?;

    let metadata = endpoints::fetch_game_meta_data(&record.endpoint, &record.game_id).await?;
    record.metadata = Some(metadata);

    let arc_record = Arc::new(record);

    record_media_data(arc_record).await
}

async fn record_media_data(record: Arc<Record>) -> Result<Record, RecordingError> {
    let endpoint = record.endpoint.clone();
    let game_id = record.game_id.clone();
    let mut tasks = Vec::new();
    let mut current_chunk_id = 1;
    let mut current_keyframe_id = 1;

    loop {
        match endpoints::fetch_last_chunk_info(&endpoint, &game_id).await {
            Ok(chunk_info) => {
                if chunk_info.chunk_id != current_chunk_id
                    || chunk_info.key_frame_id != current_keyframe_id
                {
                    debug!("Received first chunk info but there is a gap between chunk_id or keyframe_id try to download previous media data");
                    let record_clone = record.clone();
                    let process_previous_media_data_task = spawn(async move {
                        let _ = process_previous_media_data(
                            record_clone,
                            chunk_info.chunk_id,
                            chunk_info.key_frame_id,
                        )
                        .await;
                    });
                    tasks.push(process_previous_media_data_task);

                    current_chunk_id = chunk_info.chunk_id;
                    current_keyframe_id = chunk_info.key_frame_id;
                }

                let record_clone = record.clone();
                let process_media_data_task = spawn(async move {
                    let _ = process_media_data(
                        record_clone,
                        chunk_info.chunk_id,
                        chunk_info.key_frame_id,
                    )
                    .await;
                });

                tasks.push(process_media_data_task);

                if chunk_info.chunk_id == chunk_info.end_game_chunk_id {
                    debug!("Received last chunk info");
                    break;
                }

                current_chunk_id += 1;
                current_keyframe_id += 1;

                let waiting_time = Duration::from_millis(chunk_info.next_available_chunk as u64)
                    + Duration::from_secs(1);
                debug!("Wait {:?} milliseconds before next iteration", waiting_time);
                sleep(waiting_time).await;
            }
            Err(error) => {
                debug!(
                    "Record Frames received error {} retry in 10 seconds...",
                    error
                );
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        }
    }

    debug!("Awaiting for tasks");
    for task in tasks {
        let _ = task.await;
    }

    Arc::try_unwrap(record).map_err(|_| RecordingError::ArcUnwrapError)
}

async fn process_previous_media_data(
    record: Arc<Record>,
    current_chunk_id: u32,
    current_key_frame_id: u32,
) -> Result<(), reqwest::Error> {
    for chunk_id in (1..=current_chunk_id - 1).rev() {
        let _ = fetch_and_store_game_data_chunk(record.clone(), chunk_id).await;
    }

    for keyframe_id in (1..=current_key_frame_id - 1).rev() {
        let _ = fetch_and_store_keyframe(record.clone(), keyframe_id).await;
    }

    Ok(())
}

async fn process_media_data(
    record: Arc<Record>,
    chunk_id: u32,
    keyframe_id: u32,
) -> Result<(), reqwest::Error> {
    let _ = fetch_and_store_game_data_chunk(record.clone(), chunk_id).await;
    let _ = fetch_and_store_keyframe(record, keyframe_id).await;

    Ok(())
}

async fn fetch_and_store_game_data_chunk(
    record: Arc<Record>,
    chunk_id: u32,
) -> Result<(), reqwest::Error> {
    // Return if the chunk ID is already in the set
    if record.has_game_data_chunk(chunk_id) {
        return Ok(());
    }

    match endpoints::fetch_game_data_chunk(&record.endpoint, &record.game_id, chunk_id).await {
        Ok(game_data_chunk) => {
            debug!("Storing game data chunk id {}", chunk_id);
            if let Err(e) = record.store_game_data_chunk(chunk_id, game_data_chunk) {
                debug!("Error while storing chunk: {}", e);
            } else {
                record.insert_game_data_chunk(chunk_id);
            }
        }
        Err(error) => {
            debug!("error {}", error);
            return Err(error);
        }
    }
    Ok(())
}

async fn fetch_and_store_keyframe(
    record: Arc<Record>,
    keyframe_id: u32,
) -> Result<(), reqwest::Error> {
    // Return if the keyframe ID is already in the set
    if record.has_keyframe(keyframe_id) {
        return Ok(());
    }

    match endpoints::fetch_keyframe(&record.endpoint, &record.game_id, keyframe_id).await {
        Ok(keyframe) => {
            debug!("Storing keyframe {}", keyframe_id);
            if let Err(e) = record.store_key_frame(keyframe_id, keyframe) {
                debug!("Error while storing keyframe: {}", e);
            } else {
                record.insert_keyframe(keyframe_id);
            }
        }
        Err(error) => {
            debug!("error {}", error);
            return Err(error);
        }
    }
    Ok(())
}
