use mongodb::{
    Collection, Cursor,
    bson::{
        doc,
        oid::ObjectId
    }
};
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;
use axum::response::Json;

use crate::connection::get_location_collection;
use crate::result::ApiResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct LatLng {
    lat: f32,
    lng: f32
}

#[derive(Serialize, Deserialize)]
pub struct LocationToCreate {
    pub name: String,
    pub lat: f32,
    pub lng: f32
}

#[derive(Serialize, Deserialize)]
pub struct LocationToUpdate {
    pub name: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub _id: Option<ObjectId>,
    pub name: String,
    pub latlng: LatLng
}

/*
impl Location {
    pub fn print_location(&self) -> () {
        println!("Name: {}, lat: {}, lng: {}", self.name, self.latlng.lat, self.latlng.lng);
    }
}
*/

fn parse_create_location(location_to_create: LocationToCreate) -> Location {
    Location {
        _id: None,
        latlng: LatLng { 
            lat: location_to_create.lat, 
            lng: location_to_create.lng 
        },
        name: location_to_create.name,
    }
}

pub async fn find_all_locations() -> ApiResult<Vec<Location>> {
    let location_collection: Collection<Location>  = get_location_collection().await.unwrap();
    let locations_cursor: Cursor<Location> = location_collection.find(None, None).await.unwrap();
    let locations: Vec<Location> = locations_cursor
        .map(|res| { return res.unwrap(); })
        .collect()
        .await;

    println!("Locations found: {}", locations.len());

    Ok(locations)
}

pub async fn find_one_location(_id: ObjectId) -> ApiResult<Option<Location>> {
    let location_collection: Collection<Location>  = get_location_collection().await.unwrap();
    let location = location_collection.find_one(
        doc! {
            "_id": &_id
        },
        None
    ).await.unwrap();
    Ok(location)
}

pub async fn create_location(location_to_create: LocationToCreate) -> ApiResult<Json<Location>> {
    let location_collection: Collection<Location>  = get_location_collection().await.unwrap();
    let location = parse_create_location(location_to_create);
    let _ = location_collection.insert_one(&location, None).await;

    Ok(Json(location))
}

pub async fn delete_location(_id: ObjectId) -> ApiResult<u64> {
    let location_collection: Collection<Location>  = get_location_collection().await.unwrap();
    let delete_result = location_collection.delete_one(
        doc! {
            "_id": &_id
        },
        None
    ).await.unwrap();
    Ok(delete_result.deleted_count)
}
