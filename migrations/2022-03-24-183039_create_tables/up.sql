-- Your SQL goes here
CREATE TABLE tables (
  id SERIAL PRIMARY KEY,
  description VARCHAR NOT NULL,
  max_people INTEGER NOT NULL,
  min_people INTEGER NOT NULL
)

