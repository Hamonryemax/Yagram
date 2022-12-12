extern crate core;

mod app_settings;
mod app_state;
mod auth;
mod errors;

use actix_session::{storage::RedisActorSessionStore, Session, SessionMiddleware};
use actix_settings::{ApplySettings as _, Mode, Settings};
use actix_web::cookie::{Key, SameSite};
use actix_web::middleware::Logger;
use actix_web::{
    dev::ServiceRequest, get, post, web, web::scope, App, Error, HttpResponse, HttpServer,
    Responder,
};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use app_state::AppState;
use dotenv::dotenv;
use entity::prelude::User;
use entity::user;
use migration::{Migrator, MigratorTrait};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use sea_orm::entity::prelude::*;
use std::env;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/users/{user_id}")]
async fn get_user(
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

#[actix_web::main]
pub async fn start() -> Result<(), std::io::Error> {
    let settings = app_settings::SettingsInitializer::init();
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let app_state = AppState::new(&settings.application)
        .await
        .expect("AppState create failed");

    Migrator::up(&app_state.db, None)
        .await
        .expect("Migration failed");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&settings.actix.tls.private_key, SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(&settings.actix.tls.certificate)
        .unwrap();

    let state = web::Data::new(app_state);
    let settings_to_move = settings.clone();

    HttpServer::new(move || {
        let settings = settings_to_move.clone();
        App::new()
            .wrap(Logger::new("%a %s %{User-Agent}i"))
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(settings.application.redis_connection.clone()),
                    Key::generate(),
                )
                .cookie_http_only(false)
                .cookie_same_site(SameSite::Strict)
                .build(),
            )
            .app_data(state.clone())
            .service(ping)
            .service(scope("/auth").service(auth::login).service(auth::auth))
            .service(
                scope("/api")
                    .wrap(HttpAuthentication::bearer(auth::jwt_validation::validator))
                    .service(get_user),
            )
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .apply_settings(&settings)
    .run()
    .await
}
