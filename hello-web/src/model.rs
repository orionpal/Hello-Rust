use serde::{Serialize, Deserialize};
use super::schema::items;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "items"]
pub struct NewItem {
    pub name: String,
    pub description: String,
}
