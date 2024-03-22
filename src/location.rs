use mongodb::{error::Error, Collection};
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;

use crate::connection::get_location_collection;

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

pub async fn find_all_locations() -> Result<Vec<Result<Location, Error>>, Error> {
    let location_collection: Collection<Location>  = get_location_collection().await?;
    let locations_cursor = location_collection.find(None, None).await?;
    let locations: Vec<Result<Location, Error>> = locations_cursor.collect().await;

    Ok(locations)
}