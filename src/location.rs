use mongodb::{Collection, Cursor};
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;

use crate::connection::get_location_collection;
use crate::result::ApiResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct LatLng {
    lat: f32,
    lng: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    name: String,
    latlng: LatLng
}

impl Location {
    pub fn print_location(&self) -> () {
        println!("Name: {}, lat: {}, lng: {}", self.name, self.latlng.lat, self.latlng.lng);
    }
}

pub async fn find_all_locations() -> ApiResult<Vec<Location>> {
    let location_collection: Collection<Location>  = get_location_collection().await.unwrap();
    let locations_cursor: Cursor<Location> = location_collection.find(None, None).await.unwrap();
    let locations: Vec<Location> = locations_cursor
        .map(|res| { return res.unwrap(); })
        .collect()
        .await;

    Ok(locations)
}