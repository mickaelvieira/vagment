extern crate vagment;

use std::env;
use vagment::logger;

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let command = env::args().nth(1);

    if command.is_none() {
        logger::error("Please provide a command");
    } else {
        if command.unwrap() == "version" {
            logger::info(version);
        }
    }
}
