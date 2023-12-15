#[macro_use]
extern crate rocket;
mod database;

use database::requests::GeoRequest;
use database::responses::GeoProvider;
use database::{create_geo_provider, get_geo_provider, get_geo_providers, DBResult};

use rocket::serde::json::Json;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use sqlx::{Pool, Sqlite, SqlitePool};

#[post("/create", format = "json", data = "<GeoProvider>")]
async fn create(
    GeoProvider: Json<GeoRequest>,
    pool: &State<Pool<Sqlite>>,
) -> DBResult<Json<GeoProvider>> {
    let id = create_geo_provider(
        pool,
        &GeoProvider.name,
        &GeoProvider.description,
        &GeoProvider.api_key,
        &GeoProvider.counter_limit,
        &GeoProvider.counter,
    )
    .await?;
    let task = get_geo_provider(pool, id).await?;
    Ok(Json(task))
}

#[get("/providers")]
async fn getall(pool: &State<Pool<Sqlite>>) -> DBResult<Json<Vec<GeoProvider>>> {
    let tasks = get_geo_providers(pool).await?;
    Ok(Json(tasks))
}

#[get("/provider/<id>")]
async fn detail(id: i64, pool: &State<Pool<Sqlite>>) -> DBResult<Json<GeoProvider>> {
    let task = get_geo_provider(pool, id).await?;
    Ok(Json(task))
}

#[get("/")]
async fn index() -> Template {
    Template::render("index", context! {field:"Hello", person:"ZHENG Bote"})
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let pool = SqlitePool::connect("sqlite://data.db")
        .await
        .expect("Couldn't connect to sqlite database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Couldn't migrate the database tables");

    let _rocket = rocket::build()
        .mount("/", routes![index, getall, create, detail])
        .attach(Template::fairing())
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}
