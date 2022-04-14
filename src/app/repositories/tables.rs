// desde rocket - mas o menos
use crate::config::database::Db;

// desde app
use crate::app::models::table::{NewTable, Table};

// desde diesel
use crate::app::models::schema::tables;
use diesel::prelude::*;

pub async fn find_all(db: &Db) -> Vec<Table> {
    let tables: Vec<Table> = db
        .run(move |conn| tables::table.load::<Table>(conn))
        .await
        .unwrap();

    tables
}

pub async fn find_one(db: Db, id: i32) -> Table {
    let table: Table = db
        .run(move |conn| tables::table.find(id).get_result(conn))
        .await
        .unwrap();

    table
}

pub async fn save(db: Db, data: NewTable) -> Table {
    let table: Table = db
        .run(move |conn| {
            diesel::insert_into(tables::table)
                .values(data)
                .get_result::<Table>(conn)
        })
        .await
        .unwrap();

    table
}

pub async fn remove(db: Db, id: i32) -> Table {
    let table: Table = db
        .run(move |conn| diesel::delete(tables::table.find(id)).get_result(conn))
        .await
        .unwrap();

    table
}

pub async fn update(db: Db, id: i32, data: NewTable) -> Table {
    let updated_table: Table = db
        .run(move |conn| {
            let table = diesel::update(tables::table.find(id));
            table
                .set((
                    tables::columns::description.eq(data.description),
                    tables::columns::min_people.eq(data.min_people),
                    tables::columns::max_people.eq(data.max_people),
                ))
                .get_result::<Table>(conn)
        })
        .await
        .unwrap();

    updated_table
}
