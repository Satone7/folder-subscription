//! This mod is a downloader to get the files from web
use std::fs;
use std::path::Path;

use url::Url;

// static let download_command = "wget";

pub fn download(url: &Url, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // 创建父目录（如果不存在）
    if let Some(parent) = path.parent() {
        println!("Try to create dir all: {}", parent.to_str().unwrap());
        fs::create_dir_all(parent)?;
    }

    // 使用 wget 下载文件
    println!(
        "wget -o {} {}",
        path.to_str().unwrap(),
        String::from(url.clone())
    );
    let output = std::process::Command::new("wget")
        .arg("-o")
        .arg(path)
        .arg(String::from(url.clone()))
        .output()?;

    if output.status.success() {
        println!("Downloaded file: {:?}", path);
        Ok(())
    } else {
        Err("Failed to download file.".into())
    }
}
