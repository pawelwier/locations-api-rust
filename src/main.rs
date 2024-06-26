use axum::{routing::get, Router};
use dotenv::var;
use result::ApiResult;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use controller::{
    add_location,
    get_all_locations,
    get_one_location,
    remove_location,
    edit_location
};

mod connection;
mod controller;
mod location;
mod result;

#[tokio::main]
async fn main() -> ApiResult<()> {
    let url = var("URL").unwrap();
    let router: Router<()> = Router::new()
        .route("/locations", 
            get(get_all_locations)
                .post(add_location)
        )
        .route("/locations/:id", 
            get(get_one_location)
                .delete(remove_location)
                .patch(edit_location)
        )
        .layer(CorsLayer::permissive());

    match TcpListener::bind(url.clone()).await {
        Ok(listener) => {
            println!("Listening on: {:15?}", listener.local_addr().unwrap());
            axum::serve(listener, router).await.unwrap();
        },
        Err(_) => { panic!("Unable to connect: {:?}", &url) }
    }

    Ok(())
}
