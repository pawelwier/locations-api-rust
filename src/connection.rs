use mongodb::{
    error::Error, options::ClientOptions, Client, Collection, Database
};
use dotenv::var;

use crate::location::Location;

fn get_mongo_address() -> String {
    format!("mongodb+srv://{}:{}@test.fdk5v31.mongodb.net/?retryWrites=true&w=majority",
        var("MONGO_USERNAME").unwrap(),
        var("MONGO_PASSWORD").unwrap()
    )
}

pub async fn get_mongo_client() -> Result<Client, Error> {
    let client_options: Result<ClientOptions, Error> = ClientOptions::parse(get_mongo_address()).await;
    match client_options {
        Ok(options) => {
            let client: Result<Client, Error> = Client::with_options(options);
            client
        },
        Err(e) => { 
            println!("Connection error: {:?}", e);
            Err(e)
        }
    }
}

pub async fn connect() -> Result<Database, Error> {
    match get_mongo_client().await {
        Ok(connection_result) => {
            let db: Database = connection_result.database(&var("MONGO_DB_NAME").unwrap());
            println!("Connected to: {}", db.name());
            Ok(db)
        },
        Err(e) => {
           println!("Database connection error");
           Err(e)
        }
    }
}

pub async fn get_location_collection() -> Result<Collection<Location> , Error> {
    match connect().await {
        Ok(db) => Ok(db.collection::<Location>("locations")),
        Err(e) => {
            println!("Couldn't find the collection");
            Err(e)
         }
    }
}