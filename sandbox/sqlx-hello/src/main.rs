use sqlx::postgres::PgPoolOptions;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error>{
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test").await?;

    let row: (i64, _) = sql::query_as("SELECT $1").bind(150_i64).fetch_one(&pool).await?;

    assert!(row.0, 150);

    Ok(())
}
