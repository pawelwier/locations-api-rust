use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub type ApiResult<T> = core::result::Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    InvalidLocationData,
    InvalidUpdateObject
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        println!("Error occurred: {:?}", self);

        (StatusCode::INTERNAL_SERVER_ERROR, "ERROR").into_response()        
    }
}