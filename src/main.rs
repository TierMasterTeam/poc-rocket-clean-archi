use rocket::{launch, routes};
use poc_projet_annuel_b3::adapters::controllers::user_controller::get_user_by_id;
use poc_projet_annuel_b3::infrastructure::app_state::AppState;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(AppState::init().await)
        .mount("/", routes![get_user_by_id])
}