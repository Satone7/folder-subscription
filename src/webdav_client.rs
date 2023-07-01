use rustydav::client;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WEBDAV_CLIENT: client::Client = client::Client::init("", "");
}