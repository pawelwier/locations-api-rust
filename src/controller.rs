use serde_json::{Value, json};
use axum::response::Json;

use crate::{location::{find_all_locations, Location}, result::{ApiError, ApiResult}};

async fn parse_location_results(locations: Vec<Location>) -> Json<Value> {
    Json(json!(locations))
}

pub async fn get_all_locations() -> ApiResult<Json<Value>> {
    let locations: ApiResult<Vec<Location>> = find_all_locations().await;

    match locations {
        Ok(value) => {
            let locations_data: Json<Value> = parse_location_results(value).await;
            Ok(locations_data)
        },
        Err(_) => { Err(ApiError::InvalidLocationData)},
    }
}