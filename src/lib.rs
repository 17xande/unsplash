use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io;
use std::path::PathBuf;

const URL: &str = "https://api.unsplash.com/";

#[derive(Serialize, Deserialize, Debug, Default)]
struct Photo {
    id: String,
    created_at: Option<String>,
    updated_at: Option<String>,
    promoted_at: Option<String>,
    width: u16,
    height: u16,
    color: Option<String>,
    blur_hash: Option<String>,
    description: Option<String>,
    alt_description: Option<String>,
    downloads: u64,
    views: u64,
    likes: u32,
    liked_by_user: bool,
    exif: Exif,
    location: Location,
    current_user_collections: Vec<Collection>,
    urls: Urls,
    links: Links,
    user: User,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Exif {
    make: Option<String>,
    model: Option<String>,
    exposure_time: Option<String>,
    aperture: Option<String>,
    focal_length: Option<String>,
    iso: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Location {
    name: Option<String>,
    city: Option<String>,
    country: Option<String>,
    position: Position,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Position {
    latitude: f32,
    longitude: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Collection {
    id: u64,
    title: Option<String>,
    published_at: Option<String>,
    last_collected_at: Option<String>,
    updated_at: Option<String>,
    cover_photo: Option<String>,
    user: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct Urls {
    raw: Option<String>,
    full: Option<String>,
    regular: Option<String>,
    small: Option<String>,
    thumb: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct Links {
    #[serde(rename = "self")]
    this: Option<String>,
    html: Option<String>,
    download: Option<String>,
    // download_location: Option<String>,
    // photos: Option<String>,
    // likes: Option<String>,
    // portfolio: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct User {
    id: String,
    updated_at: Option<String>,
    username: Option<String>,
    name: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    bio: Option<String>,
    location: Option<String>,
    total_likes: u32,
    total_photos: u32,
    total_collections: u32,
    instagram_username: Option<String>,
    twitter_username: Option<String>,
    portfolio_url: Option<String>,
    links: Links,
}

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
            println!("JSON: {:?}", photo);
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
    println!("creating file");

    let mut file = File::create(path).unwrap();
    io::copy(&mut content, &mut file).unwrap();
}
