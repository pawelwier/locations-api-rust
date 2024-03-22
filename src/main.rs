use connection::connect;
use location::Location;

mod connection;
mod location;

#[tokio::main]
async fn main() {
    if let Ok(db) = connect().await {
        if let Ok(mut locations_cursor) = db.collection::<Location>("locations").find(None, None).await {
            while locations_cursor.advance().await.unwrap() {
                println!("{:?}", locations_cursor.deserialize_current());
            }
        }
    }
}
