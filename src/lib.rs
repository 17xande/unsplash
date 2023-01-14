use serde::{Deserialize, Serialize};
use std::env;
use std::time;

const URL: &str = "https://api.unsplash.com/";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Photo {
    id: String,
    // created_at: time::Instant,
    // updated_at: time::Instant,
    width: u16,
    height: u16,
    color: String,
    blur_hash: String,
    downloads: u64,
    likes: u32,
    liked_by_user: bool,
    description: String,
    // exif: Exif,
    // location: Location,
    // current_user_collections: Vec<Collection>,
    // urls: Urls,
    // links: Links,
    // user: User,
}

struct Exif {
    make: String,
    model: String,
    exposure_time: f64,
    aperture: f64,
    focal_length: f32,
    iso: u32,
}

struct Location {
    name: String,
    city: String,
    country: String,
    position: Position,
}

struct Position {
    latitude: f32,
    longitude: f32,
}

struct Collection {
    id: u64,
    title: String,
    published_at: time::Instant,
    last_collected_at: time::Instant,
    updated_at: time::Instant,
    cover_photo: String,
    user: String,
}

struct Urls {
    raw: String,
    full: String,
    regular: String,
    small: String,
    thumb: String,
}

struct Links {
    cself: String,
    html: String,
    download: String,
    download_location: String,
    photos: String,
    likes: String,
    portfolio: String,
}

struct User {
    id: String,
    updated_at: time::Instant,
    username: String,
    name: String,
    portfolio_url: String,
    bio: String,
    location: String,
    total_likes: u32,
    total_photos: u32,
    total_collections: u32,
    instagram_username: String,
    twitter_username: String,
    links: Links,
}

pub async fn get_random() -> reqwest::Result<Photo> {
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

    let r_url = String::from(URL) + "photos/random/";
    let res = match client.get(r_url).send().await {
        Ok(res) => res,
        Err(err) => {
            println!("ERROR: {}", err);
            return Err(err);
        }
    };

    println!("RESPONSE: {:?}", res.text().await);
    // TODO: check and handle error codes in response.

    // let photo: Photo = match res.json().await {
    //     Ok(photo) => {
    //         println!("JSON: {:?}", photo);
    //         photo
    //     }
    //     Err(err) => {
    //         println!("ERROR: {}", err);
    //         return Err(err);
    //     }
    // };

    // Ok(photo)
    let p = Photo::default();
    Ok(p)
}
