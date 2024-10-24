use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn _get_database_client() -> mongodb::error::Result<Client> {
    dotenv::dotenv().ok();

    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    let client_options = ClientOptions::parse(&mongo_uri).await?;

    Client::with_options(client_options)
}
