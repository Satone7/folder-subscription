use std::error::Error;
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;
use config::ConfigPeer;
use url::Url;
use log;

mod config;
mod downloader;
mod scanner;
mod webdav_client;

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

fn run_for_each_config_peer(config_peer: &ConfigPeer) -> Result<(), Box<dyn Error>> {

    log::debug!("Remote: {}", &config_peer.remote);

    let files: Vec<String> = scanner::scan_webdav(&config_peer.remote)?;

    let missing_files = compare_directories(&files, &config_peer.local)?;

    let mut base_url = Url::parse(&config_peer.remote)?;

    for missing_file in missing_files {
        // println!("missing file: {}.", missing_file);
        let tmp_path = config_peer.local.clone().add("/.").add(&missing_file);
        log::info!("Try to download to {} ...", tmp_path);
        base_url.set_path(&missing_file);
        downloader::download_file_with_resume(&base_url, Path::new(&tmp_path))?;
        // println!("missing file: {} download finish.", missing_file);
    }

    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = config::Config::new("config/config.json")?;

    for config_peer in config.into_iter() {
        run_for_each_config_peer(&config_peer)?;
    }

    Ok(())
}


#[cfg(test)]
mod lib_test {

    use std::path::Path;
    use std::fs;

    use crate::config::ConfigPeer;
    use crate::run_for_each_config_peer;


    #[test]
    fn sync_many_unexists_files() {

        let test_file = Path::new("target/手机备份/照片备份");
        if test_file.exists() {
            fs::remove_dir_all(test_file).unwrap();
        }

        let test_cp: ConfigPeer = ConfigPeer {
            remote: "http://softrouter.me:18080/手机备份/照片备份".to_string(),
            local: "target/手机备份/照片备份".to_string(),
        };

        assert!(run_for_each_config_peer(&test_cp).is_ok());
    }

}