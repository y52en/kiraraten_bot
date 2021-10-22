// <S:AsRef<str>>(path:S) 
//     let STR = path.as_ref();
use reqwest;
use reqwest::Error;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub async fn http_get<S:AsRef<str>>(url: S) -> Result<String, Error> {
    let url = url.as_ref();
    let req = reqwest::get(url).await;
    match req {
        Ok(html) => Ok(html.text().await.unwrap()),
        Err(e) => Err(e),
    }
}

pub fn writefile<S:AsRef<str>,T:AsRef<str>>(path: S, inc: T) -> Result<(), Box<dyn std::error::Error>> {
    let mut write_to = inc.as_ref();
    let mut path = path.as_ref();
    let mut file = File::create(path)?;
    write!(file, "{}", write_to)?;
    file.flush()?;
    Ok(())
}

pub fn regex(reg: &str) -> Regex {
    Regex::new(reg).unwrap()
}

pub fn readfile<S:AsRef<str>>(path: S) -> Result<String, Box<dyn std::error::Error>> {
    let path = path.as_ref();
    if !is_path_exist(&path) {
        return Ok(String::new());
    }
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

pub fn is_path_exist(path: &str) -> bool {
    Path::new(path).exists()
}