#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;

    let result = sqlx::query!("SELECT id FROM test")
        .fetch_all(&db_pool)
        .await?;

    println!("result: {:?}", result);
    Ok(())
}
