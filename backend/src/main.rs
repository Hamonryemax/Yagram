use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ping).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
