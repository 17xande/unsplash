use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Photo {
    pub id: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub promoted_at: Option<String>,
    pub width: u16,
    pub height: u16,
    pub color: Option<String>,
    pub blur_hash: Option<String>,
    pub description: Option<String>,
    pub alt_description: Option<String>,
    pub downloads: u64,
    pub views: u64,
    pub likes: u32,
    pub liked_by_user: bool,
    pub exif: Exif,
    pub location: Location,
    pub current_user_collections: Vec<Collection>,
    pub urls: Urls,
    pub links: Links,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Exif {
    pub make: Option<String>,
    pub model: Option<String>,
    pub exposure_time: Option<String>,
    pub aperture: Option<String>,
    pub focal_length: Option<String>,
    pub iso: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Location {
    pub name: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Position {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Collection {
    pub id: u64,
    pub title: Option<String>,
    pub published_at: Option<String>,
    pub last_collected_at: Option<String>,
    pub updated_at: Option<String>,
    pub cover_photo: Option<String>,
    pub user: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Urls {
    pub raw: Option<String>,
    pub full: Option<String>,
    pub regular: Option<String>,
    pub small: Option<String>,
    pub thumb: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Links {
    #[serde(rename = "self")]
    pub this: Option<String>,
    pub html: Option<String>,
    pub download: Option<String>,
    // download_location: Option<String>,
    // photos: Option<String>,
    // likes: Option<String>,
    // portfolio: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub id: String,
    pub updated_at: Option<String>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub total_likes: u32,
    pub total_photos: u32,
    pub total_collections: u32,
    pub instagram_username: Option<String>,
    pub twitter_username: Option<String>,
    pub portfolio_url: Option<String>,
    pub links: Links,
}
