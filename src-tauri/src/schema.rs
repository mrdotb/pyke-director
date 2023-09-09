// @generated automatically by Diesel CLI.

diesel::table! {
    records (id) {
        id -> Text,
        version -> Text,
        base_url -> Text,
        platform_id -> Text,
        game_id -> Text,
        encryption_key -> Text,
        metadata -> Text,
        keyframes -> Text,
        game_data_chunks -> Text,
        storage_path -> Text,
        created_at -> Timestamp,
    }
}
