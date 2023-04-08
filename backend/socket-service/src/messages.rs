use actix::prelude::*;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Connect {}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Disconnect {}
