use crate::config::database::Db;
use rocket::serde::json::Json;

use crate::app::models::booking::{Booking, NewBookings};

use crate::app::repositories::bookings as bookings_repo;

#[get("/")]
pub async fn index(db: Db) -> Json<Vec<Booking>> {
    let bookings: Vec<Booking> = bookings_repo::find_all(db).await;

    Json(bookings)
}

#[get("/<id>")]
pub async fn show(db: Db, id: i32) -> Json<Booking> {
    let booking: Booking = bookings_repo::find_one(db, id).await;

    Json(booking)
}

#[options("/")]
pub async fn log_store_booking() {
    println!("Agrega una reserva - ??")
}

#[post("/", format = "application/json", data = "<data>")]
pub async fn store(db: Db, data: Json<NewBookings>) -> Json<Booking> {
    let booking: Booking = bookings_repo::save(db, data.clone()).await;

    Json(booking)
}

#[delete("/<id>")]
pub async fn destroy(db: Db, id: i32) -> Json<Booking> {
    let booking: Booking = bookings_repo::remove(db, id).await;

    Json(booking)
}

#[options("/<id>")]
pub async fn log_update_booking(id: i32) {
    println!("Actualiza una mesa - {}", id);
}

#[put("/<id>", format = "application/json", data = "<data>")]
pub async fn update(db: Db, id: i32, data: Json<NewBookings>) -> Json<Booking> {
    let updated_booking: Booking = bookings_repo::update(db, id, data.clone()).await;

    Json(updated_booking)
}

// Mesas ocupadas para dicha fecha
// #[get("/date?<date>")]
// pub async fn find_by_date(db: Db, date: String) -> Json<Vec<Booking>> {
//     let by_date: Vec<Booking> = db
//         .run(move |conn| {
//             bookings::table
//                 .filter(book_date_book.eq(date))
//                 .get_results(conn)
//         })
//         .await
//         .unwrap();

//     Json(by_date)
// }

// #[get("/occuped?<date>")]
// pub async fn occuped_date(db: Db, date: String) {
//     let id_tables_occuped: Vec<i32> = db
//         .run(move |conn| {
//             bookings::table
//                 .select(book_tables_id)
//                 .filter(book_date_book.eq(date))
//                 .get_results(conn)
//         })
//         .await
//         .unwrap();
// }
