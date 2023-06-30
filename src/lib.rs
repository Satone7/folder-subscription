use std::error::Error;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use url::Url;

mod config;
mod downloader;
mod scanner;

fn compare_directories(
    webdav_files: &[String],
    local_directory: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut missing_files = Vec::new();

    for file in webdav_files {
        let tmp_path = String::from_str(local_directory)?.add("/.").add(file);
        let local_file_path = Path::new(&tmp_path);
        // println!("local_file_path: {}", local_file_path.to_str().unwrap());
        if !local_file_path.exists() {
            missing_files.push(file.clone());
        }
    }

    Ok(missing_files)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = config::Config::new("/home/pren/resource_synchronizer/config/config.json")?;

    for config_peer in config.into_iter() {
        println!("Remote: {}", &config_peer.remote);

        let files: Vec<String> = scanner::scan_webdav(&config_peer.remote)?;

        let missing_files = compare_directories(&files, &config_peer.local)?;

        let mut base_url = Url::parse(&config_peer.remote)?;

        for missing_file in missing_files {
            println!("missing file: {}.", missing_file);
            let tmp_path = config_peer.local.clone().add("/.").add(&missing_file);
            println!("Try to download to {} ...", tmp_path);
            base_url.set_path(&missing_file);
            downloader::download(&base_url, Path::new(&tmp_path))?;
            println!("missing file: {} download finish.", missing_file);
        }
    }

    Ok(())
}
