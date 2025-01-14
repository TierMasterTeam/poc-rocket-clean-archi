use async_trait::async_trait;
use crate::domain::entities::api_error::ApiError;

#[async_trait]
pub trait Usecase<T>: Send + Sync {
    async fn execute(&self) -> Result<T, ApiError>;
}
