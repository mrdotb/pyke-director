use crate::schema::records;

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Queryable, Serialize, Insertable)]
#[diesel(table_name = records)]
pub struct Record {
    pub id: String,
    pub version: String,
    pub base_url: String,
    pub platform_id: String,
    pub game_id: String,
    pub encryption_key: String,
    pub metadata: String,
    pub keyframes: String,
    pub game_data_chunks: String,
    pub storage_path: String,
    pub created_at: NaiveDateTime,
}
