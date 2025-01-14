use std::sync::{Arc};
use crate::application::{
    usecases::usecase::Usecase,
};
use async_trait::async_trait;
use derive_new::new;
use crate::application::utils::error_handling::ErrorHandler;
use crate::domain::entities::api_error::ApiError;
use crate::domain::entities::user_entity::UserEntity;
use crate::domain::repositories::user_repository::UserRepository;

#[derive(new)]
pub struct GetOneUserByIdUseCase {
    user_id: i64,
    repository: Arc<dyn UserRepository>,
}

#[async_trait]
impl<'a> Usecase<UserEntity> for GetOneUserByIdUseCase {
    async fn execute(&self) -> Result<UserEntity, ApiError> {
        if self.user_id < 0 {
            return Err(ErrorHandler::bad_request("Invalid user ID"));
        }
        
        self.repository.get_user_by_id(self.user_id).await
    }
}
