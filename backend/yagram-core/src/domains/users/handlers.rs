use crate::app_state::AppState;
use crate::errors;
use crate::users::queries::find_user_by_id;
use actix_web::{get, web, HttpResponse};
use entity::user;

#[get("/users/{user_id}")]
pub async fn get_user(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, errors::ServiceError> {
    let id = path.into_inner();
    let user_o: Option<user::Model> = find_user_by_id(&data.db, id).await.unwrap();

    match user_o {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(errors::ServiceError::NotFound),
    }
}
