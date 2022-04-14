use crate::config::database::Db;

use crate::app::models::booking::{Booking, NewBookings};

use crate::app::models::schema::bookings;
use diesel::prelude::*;

pub async fn find_all(db: Db) -> Vec<Booking> {
    let booking: Vec<Booking> = db
        .run(move |conn| bookings::table.load::<Booking>(conn))
        .await
        .unwrap();

    booking
}

pub async fn find_one(db: Db, id: i32) -> Booking {
    let booking: Booking = db
        .run(move |conn| bookings::table.find(id).get_result(conn))
        .await
        .unwrap();

    booking
}

pub async fn save(db: Db, data: NewBookings) -> Booking {
    let booking: Booking = db
        .run(move |conn| {
            diesel::insert_into(bookings::table)
                .values(data)
                .get_result::<Booking>(conn)
        })
        .await
        .unwrap();

    booking
}

pub async fn remove(db: Db, id: i32) -> Booking {
    let booking: Booking = db
        .run(move |conn| diesel::delete(bookings::table.find(id)).get_result(conn))
        .await
        .unwrap();

    booking
}

pub async fn update(db: Db, id: i32, data: NewBookings) -> Booking {
    let updated_booking: Booking = db
        .run(move |conn| {
            let booking = diesel::update(bookings::table.find(id));

            booking
                .set((
                    bookings::columns::tables_id.eq(data.tables_id),
                    bookings::columns::username.eq(data.username),
                    bookings::columns::people.eq(data.people),
                    bookings::columns::date_book.eq(data.date_book),
                ))
                .get_result::<Booking>(conn)
        })
        .await
        .unwrap();

    updated_booking
}

pub async fn occuped_tables(db: Db, date: String) -> Vec<i32> {
    let occuped_tables_ids: Vec<i32> = db
        .run(move |conn| {
            bookings::table
                .select(bookings::columns::tables_id)
                .filter(bookings::columns::date_book.eq(date))
                .get_results(conn)
        })
        .await
        .unwrap();

    occuped_tables_ids
}
