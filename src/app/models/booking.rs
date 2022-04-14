use crate::app::models::schema::bookings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Booking {
    pub id: i32,
    pub tables_id: i32,
    pub username: String,
    pub people: i32,
    pub date_book: String,
}

#[derive(Debug, Clone, FromForm, Deserialize, Serialize, Insertable)]
#[table_name = "bookings"]
pub struct NewBookings {
    pub tables_id: i32,
    pub username: String,
    pub people: i32,
    pub date_book: String,
}
