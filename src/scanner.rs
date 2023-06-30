//! This is a scanner to get the target webdev files tree.
//!
use percent_encoding::percent_decode_str;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use reqwest::Method;
use std::error::Error;
use std::io::Cursor;
use std::str::FromStr;
use url::Url;
use xmltree::Element;

pub fn scan_webdav(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut parsed_url = Url::parse(url)?;
    let mut files = Vec::new();

    // 创建一个 HTTP 客户端
    let client = Client::new();

    // 设置 Depth 头部，这对于 PROPFIND 请求通常是必要的
    let mut headers = HeaderMap::new();
    headers.insert("Depth", "1".parse().unwrap());

    // 创建一个 PROPFIND 请求
    let response = client
        .request(Method::from_bytes(b"PROPFIND").unwrap(), url)
        .headers(headers)
        .body("") // PROPFIND 请求通常包含一个 XML 主体，但在这个例子中我们不使用它
        .send()?
        .text()?;

    let xml = Element::parse(Cursor::new(response))?;

    let results = traverse_files(&xml);

    for result in results {
        if result.ends_with("/") {
            parsed_url.set_path(&result);
            files.extend(scan_webdav(parsed_url.as_str())?);
        } else {
            let file_name =
                String::from_str(&percent_decode_str(&result).decode_utf8().unwrap()).unwrap();
            println!("Find file: {}", file_name);
            files.push(file_name);
        }
    }

    Ok(files)
}

fn traverse_files(element: &Element) -> Vec<String> {
    let mut ret = Vec::new();

    let mut flag = true;

    for child in &element.children {
        if flag {
            flag = false;
            continue;
        }
        if child.name == "response" {
            let href = child
                .get_child("href")
                .and_then(|href| href.text.clone())
                .unwrap();
            ret.push(href);
        }

        ret.extend(traverse_files(child));
    }

    ret
}
