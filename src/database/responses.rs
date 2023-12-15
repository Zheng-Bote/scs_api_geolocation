use rocket::serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GeoProvider {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub api_key: String,
    pub counter_limit: i64,
    pub counter: i64,
}
