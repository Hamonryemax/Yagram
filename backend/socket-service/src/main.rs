mod messages;
mod server;
mod session;

use actix::Actor;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use crate::server::WsServer;
use crate::session::WsSession;

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("Handle WS connection");
    let resp = ws::start(WsSession {}, &req, stream);
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
