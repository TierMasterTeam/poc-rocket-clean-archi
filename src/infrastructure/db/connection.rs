use sqlx::{Pool, Postgres, Sqlite};

pub enum DatabaseType {
    Postgres,
    Sqlite,
}

pub async fn init_postgres_pool(database_url: &str) -> Pool<Postgres> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to Postgres database")
}

pub async fn init_sqlite_pool(database_url: &str) -> Pool<Sqlite> {
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to SQLite database")
}