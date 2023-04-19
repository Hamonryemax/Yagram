mod message_processor;

use actix::prelude::*;
use actix_rt::System;
use actix_web;
use log::info;
use message_processor::MessageProcessor;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    let mut sys = System::new();
    let _addr = sys.block_on(async {
        let message_processor = MessageProcessor;
        message_processor.start()
    });
    sys.run();
}
