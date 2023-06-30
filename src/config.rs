use serde::{Deserialize, Serialize};
use serde_json::{Result as JsonResult, Value};
use std::error::Error;
use std::fs::{self, File};
use std::option::Iter;
use std::path::Path;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
struct Webdav {
    host: String,

    #[serde(skip)]
    parsed_host: Option<Url>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Folder {
    remote: String,
    local: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    webdav: Webdav,
    folders: Vec<Folder>,
}

pub struct ConfigPeer {
    pub remote: String,
    pub local: String,
}

impl Config {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let mut config: Config = serde_json::from_reader(file)?;
        config.webdav.parsed_host = Some(Url::parse(&config.webdav.host)?);
        Ok(config)
    }
}

impl Iterator for Config {
    type Item = ConfigPeer;

    fn next(&mut self) -> Option<Self::Item> {
        let mut parsed_url = self.webdav.parsed_host.clone()?;
        self.folders.pop().map(|folder| ConfigPeer {
            remote: {
                parsed_url.set_path(&folder.remote);
                parsed_url.into()
            },
            local: folder.local,
        })
    }
}
