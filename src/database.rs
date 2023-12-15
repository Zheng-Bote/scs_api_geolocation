use sqlx::{Pool, Sqlite};

pub mod requests;
pub mod responses;

use responses::GeoProvider;

pub type DBResult<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

pub async fn create_geo_provider(
    pool: &Pool<Sqlite>,
    name: &String,
    description: &String,
    api_key: &String,
    counter_limit: &i64,
    counter: &i64,
) -> DBResult<i64> {
    let mut connection = pool.acquire().await?;
    let id = sqlx::query_as!(
            GeoProvider,
            r#"
        INSERT INTO geo_provider (name, description, api_key, counter_limit, counter) VALUES (?, ?, ?, ?, ?);
        "#,
            name,
            description,
            api_key,
            counter_limit,
            counter
    )
        .execute(&mut connection)
        .await?
        .last_insert_rowid();
    Ok(id)
}

pub async fn get_geo_provider(pool: &Pool<Sqlite>, id: i64) -> DBResult<GeoProvider> {
    let mut connection = pool.acquire().await?;
    let task = sqlx::query_as!(
        GeoProvider,
        r#"
        SELECT id, name, description, api_key, counter_limit, counter from geo_provider
        WHERE id = ?;
        "#,
        id,
    )
    .fetch_one(&mut connection)
    .await?;
    Ok(task)
}

pub async fn get_geo_providers(pool: &Pool<Sqlite>) -> DBResult<Vec<GeoProvider>> {
    let mut connection = pool.acquire().await.unwrap();
    let tasks = sqlx::query_as::<_, GeoProvider>(
        r#"
        select id, name, description, api_key, counter_limit, counter from geo_provider;
        "#,
    )
    .fetch_all(&mut connection)
    .await?;
    Ok(tasks)
}
