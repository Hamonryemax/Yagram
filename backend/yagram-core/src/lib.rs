use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use migration::{Migrator, MigratorTrait};
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

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    tracing_subscriber::fmt().init();
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:postgrespw@localhost:49153")
            .await
            .unwrap();
    Migrator::up(&db, None).await.expect("Migration failed");

    HttpServer::new(|| App::new().service(ping).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
