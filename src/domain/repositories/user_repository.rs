use async_trait::async_trait;
use crate::domain::entities::api_error::ApiError;
use crate::domain::entities::user_entity::UserEntity;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user_by_id(&self, id: i64) -> Result<UserEntity, ApiError>;
}
