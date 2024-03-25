use serde_json::{Value, json};
use axum::Json;

use crate::{
    location::{
        create_location, find_all_locations, Location, LocationToCreate
    }, 
    result::{
        ApiError, ApiResult
    }
};

async fn parse_location_results(locations: Vec<Location>) -> Json<Value> {
    Json(json!(locations))
}

// TODO: edit return value
pub async fn get_all_locations() -> ApiResult<Json<Value>> {
    let locations_result: ApiResult<Vec<Location>> = find_all_locations().await;

    match locations_result {
        Ok(value) => {
            let locations_data: Json<Value> = parse_location_results(value).await;
            Ok(locations_data)
        },
        Err(_) => { Err(ApiError::InvalidLocationData)},
    }
}

// TODO: edit return value
pub async fn add_location(Json(location_to_create): Json<LocationToCreate>) -> ApiResult<Json<Value>> {
    let location_result = create_location(location_to_create).await;

    match location_result {
        Ok(value) => {
            let locations_data: Json<Value> = Json(json!(*value));
            Ok(locations_data)
        },
        Err(_) => { Err(ApiError::InvalidLocationData)},
    }
}