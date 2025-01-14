use rocket::Responder;
use crate::adapters::presenters::api_error_presenter::ApiErrorPresenter::{BadRequest, InternalServerError, NotFound};
use crate::domain::entities::api_error::ApiError;

#[derive(Responder)]
#[response(content_type = "json")]
pub enum ApiErrorPresenter {
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    InternalServerError(String),
    #[response(status = 300)]
    BadRequest(String),
}

impl ApiErrorPresenter {
    pub fn from(err: ApiError) -> ApiErrorPresenter {
        match err {
            ApiError::NotFound(s) => {NotFound(s)}
            ApiError::InternalError(s) => InternalServerError(s),
            ApiError::BadRequest(s) => {BadRequest(s)}
        }
    }
}