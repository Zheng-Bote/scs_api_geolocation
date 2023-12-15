use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GeoRequest {
    pub name: String,
    pub description: String,
    pub api_key: String,
    pub counter_limit: i64,
    pub counter: i64,
}
