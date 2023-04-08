mod messages;
mod server;
mod session;

use crate::messages::{Connect, Disconnect};
use actix::{Actor, System};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use crate::server::WsServer;
use crate::session::WsSession;

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WsSession {}, &req, stream);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    WsServer::start(WsServer {});

    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
