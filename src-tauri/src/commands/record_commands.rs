use crate::models::record::Record;
use crate::queries;
use crate::recorder;
use crate::recorder::api::models::{Region, SpectatorEndpoint};
use uuid::Uuid;

use std::path::Path;

#[tauri::command]
pub async fn record(region: Region, game_id: String, encryption_key: String) -> Result<String, ()> {
    println!(
        "record: region {} game_id {} encryption_key {}",
        region, game_id, encryption_key
    );
    let endpoint = region.to_endpoint();
    let storage_path = Path::new("/home/john/Downloads/lol").to_path_buf();

    recorder::process::new(endpoint, game_id, encryption_key, storage_path).await;

    Ok("Ok".to_string())
}

#[tauri::command]
pub async fn record_custom_endpoint(
    base_url: String,
    platform_id: String,
    game_id: String,
    encryption_key: String,
) -> Result<String, ()> {
    println!("record custom endpoint");

    let endpoint = SpectatorEndpoint::new(base_url, platform_id);
    let storage_path = Path::new("/home/john/Downloads/lol").to_path_buf();

    match recorder::process::new(endpoint, game_id, encryption_key, storage_path).await {
        Err(error) => {
            println!("error {}", error)
        }
        Ok(record) => {
            let locked_keyframes = record.keyframes.lock().unwrap();
            let serialized_keyframes = serde_json::to_string(&*locked_keyframes).unwrap();

            let locked_game_data_chunks = record.game_data_chunks.lock().unwrap();
            let serialized_game_data_chunks =
                serde_json::to_string(&*locked_game_data_chunks).unwrap();

            let new_record = Record {
                id: Uuid::new_v4().to_string(),
                base_url: record.endpoint.base_url,
                platform_id: record.endpoint.platform_id,
                version: record.version,
                game_id: record.game_id,
                encryption_key: record.encryption_key,
                metadata: serde_json::to_string(&record.metadata).unwrap(),
                storage_path: record.storage_path.display().to_string(),
                keyframes: serialized_keyframes,
                game_data_chunks: serialized_game_data_chunks,
                created_at: chrono::Utc::now().naive_utc(),
            };
            queries::create_record(&new_record);
            println!("success record done");
        }
    }

    Ok("Ok".to_string())
}
