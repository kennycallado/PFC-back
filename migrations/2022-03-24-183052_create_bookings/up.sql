-- Your SQL goes here
CREATE TABLE bookings (
  id SERIAL PRIMARY KEY,
  tables_id SERIAL,
  username VARCHAR NOT NULL,
  people INTEGER NOT NULL,
  date_book VARCHAR NOT NULL
)
