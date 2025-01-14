use crate::domain::entities::api_error::ApiError;

pub struct ErrorHandler;

impl ErrorHandler {
    pub fn not_found(entity: &str) -> ApiError {
        ApiError::NotFound(format!("{} not found", entity))
    }

    pub fn internal_server_error(details: &str) -> ApiError {
        ApiError::InternalError(details.to_string())
    }

    pub fn bad_request(details: &str) -> ApiError {
        ApiError::BadRequest(details.to_string())
    }
}