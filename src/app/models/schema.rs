table! {
    bookings (id) {
        id -> Int4,
        tables_id -> Int4,
        username -> Varchar,
        people -> Int4,
        date_book -> Varchar,
    }
}

table! {
    tables (id) {
        id -> Int4,
        description -> Varchar,
        max_people -> Int4,
        min_people -> Int4,
    }
}

joinable!(bookings -> tables (tables_id));

allow_tables_to_appear_in_same_query!(
    bookings,
    tables,
);
