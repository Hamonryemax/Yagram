use crate::messages::{Connect, Disconnect};
use actix::prelude::*;
use actix_broker::{BrokerSubscribe, SystemBroker};

#[derive(Default)]
pub struct WsServer {}

impl Actor for WsServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<Connect>(ctx);
        self.subscribe_system_async::<Disconnect>(ctx);
    }
}

impl Handler<Connect> for WsServer {
    type Result = ();
    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        println!("Accepted connect message");
    }
}

impl Handler<Disconnect> for WsServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        println!("Accepted disconnect message");
    }
}

impl SystemService for WsServer {}
impl Supervised for WsServer {}
