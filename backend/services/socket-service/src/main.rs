mod server;
mod session;

use actix::Actor;
use actix_web::http::header;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use crate::server::WsServer;
use crate::session::WsSession;

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("Handle WS connection");
    let headers = req.headers();
    let user_id = headers
        .get("user-id")
        .expect("Expected user-id header")
        .to_str()
        .unwrap()
        .to_string();
    let resp = ws::start(WsSession::new(user_id), &req, stream);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("|| socket-service is starting ||");
    WsServer::start(WsServer::default());

    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
