use crate::app_state::AppState;
use crate::errors;
use actix_web::{get, web, HttpResponse};
use entity::prelude::User;
use entity::user;
use sea_orm::entity::prelude::*;

#[get("/users/{user_id}")]
pub async fn get_user(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, errors::ServiceError> {
    let id = path.into_inner();
    let user_o: Option<user::Model> = User::find_by_id(id).one(&data.db).await.unwrap();

    match user_o {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(errors::ServiceError::NotFound),
    }
}
