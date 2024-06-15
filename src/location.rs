use mongodb::{
    bson::{
        bson,
        doc,
        oid::ObjectId,
        to_bson,
        Document
    }, Collection, Cursor
};
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;
use axum::response::Json;

use crate::{connection::get_location_collection, result::ApiError};
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
    _id: Option<ObjectId>,
    name: String,
    latlng: LatLng
}

// TODO: separate files

/*
impl Location {
    pub fn print_location(&self) -> () {
        println!("Name: {}, lat: {}, lng: {}", self.name, self.latlng.lat, self.latlng.lng);
    }
}
*/

fn parse_create_location(location_to_create: LocationToCreate) -> Location {
    Location {
        _id: Some(ObjectId::new()),
        latlng: LatLng { 
            lat: location_to_create.lat, 
            lng: location_to_create.lng 
        },
        name: location_to_create.name
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
    let location_result = location_collection.insert_one(&location, None).await;

    match location_result {
        Ok(data) => {
            println!("Inserted: {}", data.inserted_id)
        },
        Err(e) => {
            println!("Insert error: {}", e)
        }
    }

    Ok(Json(location))
}

pub async fn delete_location(_id: ObjectId) -> ApiResult<u64> {
    let location_collection: Collection<Location>  = get_location_collection().await.unwrap();
    let query: Document = doc! { "_id": _id };
    let delete_result = location_collection.delete_one(
        query,
        None
    ).await;

    match delete_result {
        Ok(value) => {
            let count: u64 = value.deleted_count;
            println!("Records deleted: {}", count);

            Ok(count)
        },
        Err(_) => { Err(ApiError::InvalidLocationData)}
    }

}

pub async fn update_location(
    _id: ObjectId,
    location_to_update: LocationToUpdate
) -> ApiResult<ObjectId> {
    let location_collection: Collection<Location>  = get_location_collection().await.unwrap();
    let query = doc! { "_id": _id };
    let location_bson_result = to_bson(&location_to_update);

    match location_bson_result {
        Ok(location_bson) => {
            let update_object = doc! { "$set": bson!(location_bson) };
            let update_data = location_collection.update_one(query, update_object, None).await;
            
            match update_data {
                Ok(value) => {
                    let count: u64 = value.modified_count;
                    if value.modified_count == 1 {
                        println!("Records updated: {}", count);

                        Ok(_id) 
                    } else { Err(ApiError::InvalidLocationData) }
                },
                Err(_) => { Err(ApiError::InvalidLocationData) }
            }
        },
        Err(_) => { Err(ApiError::InvalidUpdateObject) }
    }
}