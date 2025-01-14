use rocket::{get, State};
use rocket::serde::json::Json;
use crate::adapters::presenters::api_error_presenter::ApiErrorPresenter;
use crate::adapters::presenters::presenter::Presenter;
use crate::adapters::presenters::user_presenter::UserPresenter;
use crate::application::usecases::get_one_user_by_id::GetOneUserByIdUseCase;
use crate::application::usecases::usecase::Usecase;
use crate::infrastructure::app_state::AppState;

#[get("/users/<id>")]
pub async fn get_user_by_id(id: i64, state: &State<AppState>) -> Result<Json<UserPresenter>, ApiErrorPresenter> {
    let repo = state.get_user_repository();
    let user = GetOneUserByIdUseCase::new(
        id,
        repo,
    )
    .execute()
    .await;
    
    match user {
        Ok(user) => Ok(Json(UserPresenter::from_entity(user))),
        Err(e) => Err(ApiErrorPresenter::from(e))
    }
}