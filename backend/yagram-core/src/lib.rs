mod errors;
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use entity::prelude::User;
use entity::user;
use migration::{Migrator, MigratorTrait};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DatabaseConnection;
use sea_orm::Database;
use std::env;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
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

// #[post("/users")]
// async fn add_user(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
//     let id = path.into_inner();
//     let user: Result<Option<User>, DbErr> = User::find_by_id(id).one(&data.db).await;
//     println!("{:?}", user);
//     HttpResponse::Ok().body("OK")
// }

struct AppState {
    db: DatabaseConnection,
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let db: DatabaseConnection = Database::connect(
        env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable"),
    )
    .await
    .unwrap();

    Migrator::up(&db, None).await.expect("Migration failed");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(".crt/key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(".crt/cert.pem").unwrap();
    let state = web::Data::new(AppState { db });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(ping)
            .service(echo)
            .service(get_user)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %s %{User-Agent}i"))
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}
