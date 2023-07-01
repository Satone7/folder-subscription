use serde::{Deserialize, Serialize};
use serde_json::{Result as JsonResult, Value};
use std::error::Error;
use std::fs::{self, File};
use std::option::Iter;
use std::path::Path;
use std::str::FromStr;
use url::Url;
use percent_encoding::percent_decode_str;

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
                percent_decode_str(&String::from(parsed_url)).decode_utf8().unwrap().into()
            },
            local: folder.local,
        })
    }
}

// wget -O /root/folder-subscription/target/./手机备份/视频备份/VID_20210321_110850.mp4 http://softrouter.me:18080/%E6%89%8B%E6%9C%BA%E5%A4%87%E4%BB%BD/%E8%A7%86%E9%A2%91%E5%A4%87%E4%BB%BD/VID_20210321_110850.mp4