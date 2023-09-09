use std::sync::Mutex;

use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};
use tauri::AppHandle;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::queries;
use crate::recorder::api::models::ChunkInfo;

struct TauriAppState {
    app: Mutex<AppHandle>,
}

#[get("/version")]
async fn version() -> HttpResponse {
    HttpResponse::Ok().body("2.0.0")
}

#[get("/getGameMetaData/{platform_id}/{game_id}/{_}/token")]
async fn get_game_meta_data(path_info: web::Path<(String, String, String)>) -> HttpResponse {
    let (_platform_id, game_id, _unamed) = path_info.into_inner();

    if let Some(record) = queries::get_record(game_id) {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(record.metadata)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/getLastChunkInfo/{platform_id}/{game_id}/{_}/token")]
async fn get_last_chunk_info(path_info: web::Path<(String, String, String)>) -> HttpResponse {
    let (_platform_id, game_id, _unamed) = path_info.into_inner();

    let data = ChunkInfo {
        chunk_id: 1,
        available_since: 30000,
        next_available_chunk: 10000,
        key_frame_id: 1,
        next_chunk_id: 2,
        end_startup_chunk_id: 1,
        start_game_chunk_id: 2,
        end_game_chunk_id: 0,
        duration: 3000,
    };

    HttpResponse::Ok().json(data)
}

#[get("/getGameDataChunk/{platform_id}/{game_id}/{chunk_id}/token")]
async fn get_game_data_chunk(
    path_info: web::Path<(String, String, u32)>,
) -> Result<HttpResponse, Error> {
    let (_platform_id, game_id, chunk_id) = path_info.into_inner();

    if let Some(record) = queries::get_record(game_id) {
        let path = format!("{}/game_data_chunks/{}", record.storage_path, chunk_id);
        let mut file = File::open(path).await?;
        let mut content = Vec::new();
        file.read_to_end(&mut content).await?;

        Ok(HttpResponse::Ok().body(content))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("/getKeyFrame/{platform_id}/{game_id}/{keyframe_id}/token")]
async fn get_key_frame(path_info: web::Path<(String, String, u32)>) -> Result<HttpResponse, Error> {
    let (_platform_id, game_id, keyframe_id) = path_info.into_inner();

    if let Some(record) = queries::get_record(game_id) {
        let path = format!("{}/keyframes/{}", record.storage_path, keyframe_id);
        let mut file = File::open(path).await?;
        let mut content = Vec::new();
        file.read_to_end(&mut content).await?;

        Ok(HttpResponse::Ok().body(content))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[actix_web::main]
pub async fn init(app: AppHandle) -> std::io::Result<()> {
    let tauri_app = web::Data::new(TauriAppState {
        app: Mutex::new(app),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(tauri_app.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/observer-mode/rest/consumer")
                    .service(version)
                    .service(get_last_chunk_info)
                    .service(get_game_meta_data)
                    .service(get_game_data_chunk)
                    .service(get_key_frame),
            )
    })
    .bind(("127.0.0.1", 4875))?
    .run()
    .await
}
