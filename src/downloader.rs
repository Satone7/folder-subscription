//! This mod is a downloader to get the files from web
use std::{fs, path};
use std::path::Path;
use url::Url;
use reqwest::header::{HeaderMap, HeaderValue, RANGE};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use curl::easy::{Easy, WriteError};
use indicatif::{ProgressBar, ProgressStyle};
use log;


pub fn download_wget(url: &Url, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // 创建父目录（如果不存在）
    if let Some(parent) = path.parent() {
        log::debug!("Try to create dir all: {}", parent.to_str().unwrap());
        fs::create_dir_all(parent)?;
    }

    // 使用 wget 下载文件
    let output = std::process::Command::new("wget")
        .arg("-O")
        .arg(path)
        .arg(String::from(url.clone()))
        .output()?;

    if output.status.success() {
        log::debug!("Downloaded file: {:?}", path);
        Ok(())
    } else {
        Err("Failed to download file.".into())
    }
}

pub fn download_file_with_resume(url: &Url, path: &Path) -> Result<(), Box<dyn std::error::Error>> {

    // 创建父目录（如果不存在）
    if let Some(parent) = path.parent() {
        log::debug!("Try to create dir all: {}", parent.to_str().unwrap());
        fs::create_dir_all(parent)?;
    }

    // 检查文件是否已存在
    // let path = Path::new(save_path);
    let mut file = if path.exists() {
        OpenOptions::new().read(true).write(true).open(path)?
    } else {
        File::create(path)?
    };

    // 获取已下载的文件大小
    let downloaded_size = file.metadata()?.len();
    let mut curl = Easy::new();

    let filename = path.file_name().unwrap();
    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {bytes_per_sec}").unwrap()
            .progress_chars("##-"),
    );
    progress_bar.set_message(format!("{:?}", filename));
    curl.url(&String::from(url.clone())).unwrap();
    curl.fetch_filetime(true).unwrap();
    curl.progress(true).unwrap();
    curl.progress_function( move |total_download_bytes, curr_download_bytes, _total_upload_bytes, _curr_upload_bytes| {
        progress_bar.set_length(total_download_bytes as u64);
        if total_download_bytes > 0.0 {
            progress_bar.set_position((curr_download_bytes) as u64);
        }
        true
    }).unwrap();
    if downloaded_size > 0 {
        curl.range(&format!("{}-", downloaded_size)).unwrap();
    }
    curl.write_function(move |data| {
        if let Err(e) = file.write_all(data) {
            log::warn!("CURL下载时发生错误: {}", e.to_string());
            return Err(WriteError::Pause);
        }
        Ok(data.len())
    }).unwrap();
    curl.perform().unwrap();

    log::debug!("Download completed");
    Ok(())
}


#[cfg(test)]
mod downloader_test {

    use std::{fs, path::Path, str::FromStr};
    use url::Url;
    use env_logger;
    use std::env;

    use crate::downloader::download_file_with_resume;

    #[test]
    fn download_an_unexists_file() {

        env_logger::init();

        let test_file = Path::new("target/手机备份/视频备份/VID_20210321_110850.mp4");
        if test_file.exists() {
            fs::remove_file(test_file).unwrap();
        }

        let url = Url::from_str("http://softrouter.me:18080/手机备份/视频备份/VID_20210321_110850.mp4").unwrap();

        assert!(download_file_with_resume(&url, test_file).is_ok());
    }

}