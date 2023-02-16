// use serde::{Deserialize, Serialize};
use crate::types::Photo;
use std::{env, fs::File, io, path::PathBuf};

pub mod types;

const URL: &str = "https://api.unsplash.com/";

pub async fn download_photo(path: &PathBuf) {
    let photo = get_wallpaper().await.unwrap();
    get_photo(photo.urls.full.unwrap(), path).await;
}

pub async fn download_random(path: &PathBuf) {
    let photo = get_random().await.unwrap();
    get_photo(photo.urls.full.unwrap(), path).await;
}

async fn get_wallpaper() -> reqwest::Result<Photo> {
    let access_key = env::var("AccessKey").unwrap();
    let client = gen_client(&access_key);
    let r_url = String::from(URL) + "topics/wallpapers/photos?per_page=1&orientation=landscape";
    let res = match client.get(r_url).send().await {
        Ok(res) => res,
        Err(err) => {
            println!("RESPONSE ERROR: {}", err);
            return Err(err);
        }
    };

    let photos: [Photo; 1] = match res.json().await {
        Ok(photos) => photos,
        Err(err) => {
            println!("JSON ERROR: {}", err);
            return Err(err);
        }
    };

    return Ok(photos[0].clone());
}

async fn get_random() -> reqwest::Result<Photo> {
    let access_key = env::var("AccessKey").unwrap();
    let client = gen_client(&access_key);
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

fn gen_client(access_key: &str) -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept-Version",
        reqwest::header::HeaderValue::from_static("V1"),
    );

    let access_key = String::from("Client-ID ") + access_key;

    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&access_key).unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    return client;
}

async fn get_photo(url: String, path: &PathBuf) {
    let access_key = env::var("AccessKey").unwrap();
    let client = gen_client(&access_key);
    let resp = client.get(url).send().await.unwrap();
    let mut content = io::Cursor::new(resp.bytes().await.unwrap());
    // println!("creating file");

    let mut file = File::create(path).unwrap();
    io::copy(&mut content, &mut file).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::get_wallpaper;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_wallpaper() {
        dotenv().ok();
        let res = get_wallpaper().await;
        let photo = match res {
            Ok(photo) => photo,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        println!("{}", photo.id);
    }
}

