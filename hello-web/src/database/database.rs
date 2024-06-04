use std::env;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn get_row() -> Result<String, sqlx::Error> {
    println!("trying to get row");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", database_url);
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // TEST CODE
    // Set up the database schema
    setup_database(&pool).await?;
    // Insert test data
    insert_test_data(&pool).await?;

    // Use the connection pool to interact with the database here
    // For example, let's run a simple query
    let row: (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await?;

    println!("Query result: {}", row.0);
    Ok(row.0.to_string())
}

async fn setup_database(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // Create table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS postgres (
            id SERIAL PRIMARY KEY,
            value INTEGER NOT NULL
        )
        "#,
    )
        .execute(pool)
        .await?;

    Ok(())
}

async fn insert_test_data(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // Insert data
    sqlx::query(
        r#"
        INSERT INTO postgres (value) VALUES (1), (2), (3)
        "#,
    )
        .execute(pool)
        .await?;

    Ok(())
}