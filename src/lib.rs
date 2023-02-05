// use serde::{Deserialize, Serialize};
use crate::types::Photo;
use std::{env, fs::File, io, path::PathBuf};

pub mod types;

const URL: &str = "https://api.unsplash.com/";

pub async fn download_photo(path: &PathBuf) {
    let photo = get_random().await.unwrap();
    get_photo(photo.urls.full.unwrap(), path).await;
}

async fn get_random() -> reqwest::Result<Photo> {
    let client = gen_client();
    let r_url = String::from(URL) + "photos/random/";
    let res = match client.get(r_url).send().await {
        Ok(res) => res,
        Err(err) => {
            println!("RESPONSE ERROR: {}", err);
            return Err(err);
        }
    };

    // println!("DEBUG response: {:?}", res.text().await.unwrap());
    // return Ok(Photo::default());

    // TODO: check and handle error codes in response.

    let photo: Photo = match res.json().await {
        Ok(photo) => {
            // println!("JSON: {:?}", photo);
            photo
        }
        Err(err) => {
            println!("JSON ERROR: {}", err);
            return Err(err);
        }
    };

    Ok(photo)
}

fn gen_client() -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept-Version",
        reqwest::header::HeaderValue::from_static("V1"),
    );

    let api_key = env::var("AccessKey").unwrap();
    let api_key = String::from("Client-ID ") + &api_key;

    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&api_key).unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    client
}

async fn get_photo(url: String, path: &PathBuf) {
    let client = gen_client();
    let resp = client.get(url).send().await.unwrap();
    let mut content = io::Cursor::new(resp.bytes().await.unwrap());
    // println!("creating file");

    let mut file = File::create(path).unwrap();
    io::copy(&mut content, &mut file).unwrap();
}
