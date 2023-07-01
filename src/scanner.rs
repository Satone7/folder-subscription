/// This is a scanner to get the target webdev files tree.
///
use percent_encoding::percent_decode_str;
use std::error::Error;
use std::io::Cursor;
use std::str::FromStr;
use url::Url;
use xmltree::Element;


fn traverse_files(element: &Element) -> Vec<String> {
    let mut ret = Vec::new();

    let mut flag = true;

    for child in &element.children {
        let child = match child.as_element() {
            Some(x) => x,
            None => continue,
        };
        if flag {
            flag = false;
            continue;
        }
        if child.name == "response" {
            let href = child
                .get_child("href")
                .and_then(|href| href.get_text().clone())
                .unwrap()
                .to_string();
            ret.push(href);
        }

        ret.extend(traverse_files(child));
    }

    ret
}

pub fn scan_webdav(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut parsed_url = Url::parse(url)?;
    let mut files = Vec::new();

    let response = crate::webdav_client::WEBDAV_CLIENT.list(url, "1")?.text()?;

    let xml = Element::parse(Cursor::new(response))?;

    let results = traverse_files(&xml);

    for result in results {
        if result.ends_with("/") {
            parsed_url.set_path(&result);
            files.extend(scan_webdav(parsed_url.as_str())?);
        } else {
            let file_name =
                String::from_str(&percent_decode_str(&result).decode_utf8().unwrap()).unwrap();
            // println!("Find file: {}", file_name);
            files.push(file_name);
        }
    }

    Ok(files)
}
