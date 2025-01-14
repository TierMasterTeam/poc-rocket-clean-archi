use async_trait::async_trait;
use derive_new::new;
use sqlx::{Pool, Sqlite};
use crate::application::utils::error_handling::ErrorHandler;
use crate::domain::entities::api_error::ApiError;
use crate::domain::entities::user_entity::UserEntity;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::models::model::Model;
use crate::infrastructure::models::user_model::UserModel;

#[derive(new)]
pub struct SqliteUserRepository  {
    pool: Pool<Sqlite>,
}

#[async_trait]
impl<'a>  UserRepository for SqliteUserRepository  {
    async fn get_user_by_id(&self, id: i64) -> Result<UserEntity, ApiError> {
        let user = sqlx::query_as::<Sqlite, UserModel>("select * from users where id=$1")
            .bind(id)
            .fetch_optional(&self.pool).await;

        match user {
            Ok(Some(user)) => Ok(user.to_entity()),
            Ok(None) => Err(ErrorHandler::not_found("User")),
            Err(e) => Err(ErrorHandler::internal_server_error(&e.to_string()))
        }
    }
}