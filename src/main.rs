use mongodb::error::Error;
use location::{find_all_locations, Location};

mod connection;
mod location;

#[tokio::main]
async fn main() {
        let locations: Vec<Result<Location, Error>> = find_all_locations().await.unwrap();

        for location_result in locations {
            location_result.unwrap().print_location();
        }
}
