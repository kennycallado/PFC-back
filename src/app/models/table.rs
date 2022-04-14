use crate::app::models::schema::tables;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Table {
    pub id: i32,
    pub description: String,
    pub max_people: i32,
    pub min_people: i32,
}

#[derive(Debug, Clone, FromForm, Deserialize, Serialize, Insertable)]
#[table_name = "tables"]
pub struct NewTable {
    pub description: String,
    pub max_people: i32,
    pub min_people: i32,
}
