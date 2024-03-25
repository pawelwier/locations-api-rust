use result::ApiResult;
use tokio::net::TcpListener;

use axum::{routing::get, Router};
use controller::{add_location, get_all_locations};

mod connection;
mod controller;
mod location;
mod result;

#[tokio::main]
async fn main() -> ApiResult<()> {
    let url = "127.0.0.1:5050".to_string();
    let router: Router<()> = Router::new()
        .route("/locations", get(get_all_locations).post(add_location));

    match TcpListener::bind(url.clone()).await {
        Ok(listener) => {
            println!("Listening on: {:15?}", listener.local_addr().unwrap());
            axum::serve(listener, router).await.unwrap();
        },
        Err(_) => { panic!("Unable to connect: {:?}", &url) }
    }

    Ok(())
}
