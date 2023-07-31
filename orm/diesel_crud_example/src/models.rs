use diesel::{Insertable, Queryable};
use serde::Deserialize;
use serde::Serialize;

use crate::schema::sample_table;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "sample_table"]
pub struct Sample {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "sample_table"]
pub struct NewSample<'a> {
    pub name: &'a str,
    pub age: i32,
}
