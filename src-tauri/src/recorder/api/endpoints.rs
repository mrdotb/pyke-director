use super::models::{ChunkInfo, GameMetaData, SpectatorEndpoint};
use log::debug;
use reqwest;

pub async fn fetch_api_version(endpoint: &SpectatorEndpoint) -> Result<String, reqwest::Error> {
    let url = format!("{}/observer-mode/rest/consumer/version", endpoint.base_url);

    debug!("Fetching API version from URL: {}", url);

    let response: String = reqwest::get(&url).await?.error_for_status()?.text().await?;

    debug!("Received API version response: {}", response);
    Ok(response.to_string())
}

pub async fn fetch_game_meta_data(
    endpoint: &SpectatorEndpoint,
    game_id: &str,
) -> Result<GameMetaData, reqwest::Error> {
    let url = format!(
        "{base_url}/observer-mode/rest/consumer/getGameMetaData/{platform_id}/{game_id}/1/token",
        base_url = endpoint.base_url,
        platform_id = endpoint.platform_id,
        game_id = game_id
    );
    debug!("Fetching API game meta data from URL: {}", url);

    let response: GameMetaData = reqwest::get(&url).await?.error_for_status()?.json().await?;

    debug!("Received API game meta data response: {}", response);

    Ok(response)
}

pub async fn fetch_last_chunk_info(
    endpoint: &SpectatorEndpoint,
    game_id: &str,
) -> Result<ChunkInfo, reqwest::Error> {
    let url = format!(
        "{base_url}/observer-mode/rest/consumer/getLastChunkInfo/{platform_id}/{game_id}/0/token",
        base_url = endpoint.base_url,
        platform_id = endpoint.platform_id,
        game_id = game_id
    );
    debug!("Fetching API last chunk info data from URL: {}", url);

    let response: ChunkInfo = reqwest::get(&url).await?.error_for_status()?.json().await?;

    debug!("Received API last chunk info response: {}", response);

    Ok(response)
}

pub async fn fetch_game_data_chunk(
    endpoint: &SpectatorEndpoint,
    game_id: &str,
    chunk_id: u32,
) -> Result<Vec<u8>, reqwest::Error> {
    let url = format!(
        "{base_url}/observer-mode/rest/consumer/getGameDataChunk/{platform_id}/{game_id}/{chunk_id}/token",
        base_url = endpoint.base_url,
        platform_id = endpoint.platform_id,
        game_id = game_id,
        chunk_id = chunk_id
    );
    debug!("Fetching API game data chunk from URL: {}", url);

    let response = reqwest::get(url).await?.error_for_status()?;

    debug!("Received API game data chunk");

    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}

pub async fn fetch_keyframe(
    endpoint: &SpectatorEndpoint,
    game_id: &str,
    keyframe_id: u32,
) -> Result<Vec<u8>, reqwest::Error> {
    let url = format!(
        "{base_url}/observer-mode/rest/consumer/getKeyFrame/{platform_id}/{game_id}/{keyframe_id}/token",
        base_url = endpoint.base_url,
        platform_id = endpoint.platform_id,
        game_id = game_id,
        keyframe_id = keyframe_id
    );
    debug!("Fetching API keyframe from URL: {}", url);

    let response = reqwest::get(url).await?.error_for_status()?;

    debug!("Received API keyframe");

    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}

// TODO write endOfGameStats and featured endpoints

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    async fn test_fetch_api_version() {
        init();
        let mut server = Server::new_async().await;
        let _m = server
            .mock("GET", "/observer-mode/rest/consumer/version")
            .with_body("2.0.0")
            .create();

        let endpoint = SpectatorEndpoint {
            base_url: server.url(),
            platform_id: "KR".to_string(),
        };

        let version = fetch_api_version(&endpoint).await.unwrap();
        assert_eq!(version, "2.0.0");
    }

    #[tokio::test]
    async fn test_fetch_game_meta_data() {
        init();
        let mut server = Server::new_async().await;
        let _m = server
            .mock(
                "GET",
                "/observer-mode/rest/consumer/getGameMetaData/KR/6654667050/1/token",
            )
            .with_status(200)
            .with_body(
                r#"{"gameKey":{"gameId":6654667050,"platformId":"KR"},"gameServerAddress":"","port":0,"encryptionKey":"","chunkTimeInterval":30000,"startTime":"Aug 15, 2023 8:01:42 PM","gameEnded":false,"lastChunkId":39,"lastKeyFrameId":19,"endStartupChunkId":1,"delayTime":180000,"pendingAvailableChunkInfo":[{"chunkId":33,"duration":30020,"receivedTime":"Aug 15, 2023 8:17:42 PM"},{"chunkId":34,"duration":29987,"receivedTime":"Aug 15, 2023 8:18:12 PM"},{"chunkId":35,"duration":30026,"receivedTime":"Aug 15, 2023 8:18:42 PM"},{"chunkId":36,"duration":29984,"receivedTime":"Aug 15, 2023 8:19:12 PM"},{"chunkId":37,"duration":30007,"receivedTime":"Aug 15, 2023 8:19:42 PM"},{"chunkId":38,"duration":29998,"receivedTime":"Aug 15, 2023 8:20:12 PM"},{"chunkId":39,"duration":30027,"receivedTime":"Aug 15, 2023 8:20:42 PM"}],"pendingAvailableKeyFrameInfo":[{"keyFrameId":16,"receivedTime":"Aug 15, 2023 8:17:42 PM","nextChunkId":33},{"keyFrameId":17,"receivedTime":"Aug 15, 2023 8:18:42 PM","nextChunkId":35},{"keyFrameId":18,"receivedTime":"Aug 15, 2023 8:19:42 PM","nextChunkId":37},{"keyFrameId":19,"receivedTime":"Aug 15, 2023 8:20:42 PM","nextChunkId":39}],"keyFrameTimeInterval":60000000,"decodedEncryptionKey":"","startGameChunkId":2,"gameLength":0,"clientAddedLag":0,"clientBackFetchingEnabled":false,"clientBackFetchingFreq":1000,"interestScore":3325,"featuredGame":false,"createTime":"Aug 15, 2023 8:01:55 PM","endGameChunkId":-1,"endGameKeyFrameId":-1}"#,
            )
            .create();

        let endpoint = SpectatorEndpoint {
            base_url: server.url(),
            platform_id: "KR".to_string(),
        };

        let result = fetch_game_meta_data(&endpoint, "6654667050").await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.game_key.game_id, 6654667050);
        assert_eq!(response.game_key.platform_id, "KR");
    }

    #[tokio::test]
    async fn test_fetch_last_chunk_info() {
        init();
        let mut server = Server::new_async().await;
        let _m = server.mock("GET", "/observer-mode/rest/consumer/getLastChunkInfo/KR/6654667050/0/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"chunkId":53,"availableSince":4197815,"nextAvailableChunk":0,"keyFrameId":26,"nextChunkId":53,"endStartupChunkId":1,"startGameChunkId":2,"endGameChunkId":53,"duration":18869}"#)
            .create();

        let endpoint = SpectatorEndpoint {
            base_url: server.url(),
            platform_id: "KR".to_string(),
        };

        let result = fetch_last_chunk_info(&endpoint, "6654667050").await;
        assert!(result.is_ok());

        let chunk_info = result.unwrap();
        assert_eq!(chunk_info.chunk_id, 53);
        assert_eq!(chunk_info.next_available_chunk, 0);
    }

    #[tokio::test]
    async fn test_fetch_game_data_chunk() {
        init();
        let mut server = Server::new_async().await;
        let mock_data = b"mocked binary data";
        let _m = server
            .mock(
                "GET",
                "/observer-mode/rest/consumer/getGameDataChunk/KR/6654667050/1/token",
            )
            .with_status(200)
            .with_header("content-type", "application/octet-stream")
            .with_body(mock_data)
            .create();

        let endpoint = SpectatorEndpoint {
            base_url: server.url(),
            platform_id: "KR".to_string(),
        };

        let result = fetch_game_data_chunk(&endpoint, "6654667050", 1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_keyframe() {
        init();
        let mut server = Server::new_async().await;
        let mock_data = b"mocked binary data";
        let _m = server
            .mock(
                "GET",
                "/observer-mode/rest/consumer/getKeyFrame/KR/6654667050/1/token",
            )
            .with_status(200)
            .with_header("content-type", "application/octet-stream")
            .with_body(mock_data)
            .create();

        let endpoint = SpectatorEndpoint {
            base_url: server.url(),
            platform_id: "KR".to_string(),
        };

        let result = fetch_keyframe(&endpoint, "6654667050", 1).await;
        assert!(result.is_ok());
    }
}
