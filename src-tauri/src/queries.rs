use crate::db;
use crate::models::record::Record;
use crate::schema::records;
use crate::schema::records::dsl;

use diesel::prelude::*;

pub fn create_record(new_record: &Record) {
    let connection = &mut db::establish_db_connection();

    diesel::insert_into(records::table)
        .values(new_record)
        .execute(connection)
        .expect("Error saving new record");
}

pub fn get_record(game_id: String) -> Option<Record> {
    let connection = &mut db::establish_db_connection();

    dsl::records
        .filter(dsl::game_id.eq(game_id))
        .first::<Record>(connection)
        .ok()
}
