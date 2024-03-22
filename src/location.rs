use serde::{Deserialize, Serialize};

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