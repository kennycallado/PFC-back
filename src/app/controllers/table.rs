use crate::config::database::Db;
use rocket::serde::json::Json;

use crate::app::models::table::{NewTable, Table};

use crate::app::repositories::bookings as bookings_repo;
use crate::app::repositories::tables as tables_repo;

#[get("/")]
pub async fn index(db: Db) -> Json<Vec<Table>> {
    let tables: Vec<Table> = tables_repo::find_all(&db).await;

    Json(tables)
}

#[get("/<id>")]
pub async fn show(db: Db, id: i32) -> Json<Table> {
    let table: Table = tables_repo::find_one(db, id).await;

    Json(table)
}

#[post("/", format = "application/json", data = "<data>")]
pub async fn store(db: Db, data: Json<NewTable>) -> Json<Table> {
    let table: Table = tables_repo::save(db, data.clone()).await;

    Json(table)
}

#[delete("/<id>")]
pub async fn destroy(db: Db, id: i32) -> Json<Table> {
    let table: Table = tables_repo::remove(db, id).await;

    Json(table)
}

#[put("/<id>", format = "application/json", data = "<data>")]
pub async fn update(db: Db, id: i32, data: Json<NewTable>) -> Json<Table> {
    let table = tables_repo::update(db, id, data.clone()).await;

    Json(table)
}

#[get("/available?<date>")] // Formato fecha String = "Y/m/d"
pub async fn available_tables(db: Db, date: String) -> Json<Vec<Table>> {
    let mut tables: Vec<Table> = tables_repo::find_all(&db).await;
    let ids_occuped: Vec<i32> = bookings_repo::occuped_tables(db, date).await;

    if ids_occuped.len() > 0 {
        tables.retain(|table| !ids_occuped.contains(&table.id))
    }

    Json(tables)
}
