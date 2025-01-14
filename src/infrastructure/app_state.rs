use std::sync::{Arc};
use sqlx::{Pool, Postgres, Sqlite};
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::db::connection::{init_postgres_pool, init_sqlite_pool, DatabaseType};
use crate::infrastructure::repositories::pg_user_repository::PgUserRepository;
use crate::infrastructure::repositories::sqlite_user_repository::SqliteUserRepository;

pub struct AppState {
    pub current_db: DatabaseType,
    postgres_pool: Option<Pool<Postgres>>,
    sqlite_pool: Option<Pool<Sqlite>>,
}

impl AppState {
    pub async fn init() -> AppState {
        let postgres_pool = init_postgres_pool("postgres://user:password@localhost/db").await;
        let sqlite_pool = init_sqlite_pool("sqlite://:memory:").await;
        
        AppState {
            current_db: DatabaseType::Postgres,
            postgres_pool: Some(postgres_pool),
            sqlite_pool: Some(sqlite_pool),
        }
    }
    pub fn get_user_repository(&self) -> Arc<dyn UserRepository> {
        match self.current_db {
            DatabaseType::Postgres => Arc::new(PgUserRepository::new(
                self.postgres_pool.clone().unwrap()
            )),
            DatabaseType::Sqlite => Arc::new(SqliteUserRepository::new(
                self.sqlite_pool.clone().unwrap()
            ))
        }
    }
    
    pub fn set_current_db(&mut self, db_type: DatabaseType) {
        self.current_db = db_type;
    }
}