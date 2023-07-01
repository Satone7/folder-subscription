// use percent_encoding::{utf8_percent_decode, DEFAULT_ENCODE_SET};
use resource_synchronizer::run;
use std::error::Error;

mod scanner;
mod webdav_client;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    run()
}
