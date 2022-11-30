use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use entity::prelude::User;
use entity::user;
use migration::{Migrator, MigratorTrait};
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DatabaseConnection;
use sea_orm::Database;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/users/{user_id}")]
async fn get_user(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let user_o: Option<user::Model> = User::find_by_id(id)
        .one(&data.db)
        .await
        .expect("Can't find user with given ID");

    match user_o {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
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
    tracing_subscriber::fmt().init();
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:postgrespw@localhost:49153")
            .await
            .unwrap();

    Migrator::up(&db, None).await.expect("Migration failed");
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
