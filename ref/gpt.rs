use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::fs::{self, File};
use std::io::copy;
use std::path::{Path, PathBuf};
use std::time::Duration;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Deserialize)]
struct Config {
    webdav_directory: String,
    local_directory: String,
}

impl Config {
    fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let config = serde_json::from_reader(file)?;
        Ok(config)
    }
}

#[tokio::main]
async fn main() {
    // 读取配置文件
    let config = Config::from_file("config.json").expect("Failed to read config file.");

    // 扫描 WebDAV 目录
    let files =
        scan_webdav_directory(&config.webdav_directory).expect("Failed to scan WebDAV directory.");

    // 比对本地目录
    let missing_files = compare_directories(&files, &config.local_directory)
        .expect("Failed to compare directories.");

    // 下载缺失的文件
    for file in missing_files {
        let webdav_file_path = Path::new(&config.webdav_directory).join(&file);
        let local_file_path = Path::new(&config.local_directory).join(&file);

        download_file(&webdav_file_path, &local_file_path).expect("Failed to download file.");
    }
}

fn scan_webdav_directory(directory: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut files = Vec::new();

    // 发送 PROPFIND 请求获取 WebDAV 目录下的文件列表
    let res = client
        .request(reqwest::Method::PROPFIND, directory)
        .send()?
        .text()?;

    // 解析 XML 结果，提取文件名
    // 这里使用你选择的 XML 解析库进行解析
    // 以下示例仅作为伪代码
    let parsed_files = parse_xml(&res)?;

    for file in parsed_files {
        files.push(file);
    }

    Ok(files)
}

fn compare_directories(
    webdav_files: &[String],
    local_directory: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut missing_files = Vec::new();

    for file in webdav_files {
        let local_file_path = Path::new(local_directory).join(file);
        if !local_file_path.exists() {
            missing_files.push(file.clone());
        }
    }

    Ok(missing_files)
}

fn download_file(webdav_file: &Path, local_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // 创建父目录（如果不存在）
    if let Some(parent) = local_file.parent() {
        fs::create_dir_all(parent)?;
    }

    // 使用 wget 下载文件
    let output = std::process::Command::new("wget")
        .arg("-O")
        .arg(local_file)
        .arg(webdav_file)
        .output()?;

    if output.status.success() {
        println!("Downloaded file: {:?}", local_file);
        Ok(())
    } else {
        Err("Failed to download file.".into())
    }
}

fn parse_xml(xml: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    let parser = EventReader::new(xml.as_bytes());

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == "D:href" {
                    if let Ok(XmlEvent::Characters(href)) = parser.next().unwrap() {
                        // 假设 href 是完整的文件路径，需要根据具体情况进行适当的处理
                        files.push(href);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(files)
}
