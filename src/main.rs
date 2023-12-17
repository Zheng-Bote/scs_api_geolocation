#[macro_use]
extern crate rocket;

mod database;
use database::requests::GeoRequest;
use database::responses::GeoProvider;
use database::{create_geo_provider, get_geo_provider, get_geo_providers, DBResult};

mod contentloader;
use contentloader::{get_filecontent, get_filecontent2};

use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use std::path::Path;
use std::path::PathBuf;

// CORS
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

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

#[get("/css/<file..>")]
async fn get_css(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/css/").join(file))
        .await
        .ok()
}
#[get("/js/<file..>")]
async fn get_js(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/js/").join(file))
        .await
        .ok()
}

#[get("/")]
async fn index() -> Template {
    //let html_data: String = get_filecontent(String::from("index")).await;
    let html_data: [String; 3] = get_filecontent2(String::from("index")).await;
    Template::render(
        "html/index",
        context! {field:"Hello", css:html_data[1].to_owned(), html:html_data[0].to_owned(), js:html_data[2].to_owned()},
    )
}

// CORS
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
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
        .mount("/", routes![index, getall, create, detail, get_css, get_js])
        .attach(Template::fairing())
        .attach(CORS)
        .manage(CORS)
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}
