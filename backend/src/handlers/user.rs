#[path = "../models/mod.rs"] mod models;
use models::user::User;
use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::doc, Client};
use serde::Deserialize;
use mongodb::error::Error;

#[derive(Deserialize)]
pub struct LoginData {
    pub user_id: String,
    pub name: String,
}

pub async fn store_user(
    mongo_client: web::Data<Client>,
    user_id: String,
    name: String,
) -> Result<(), Error> {
    let collection = mongo_client
        .database("polling_app")
        .collection::<User>("users");

    let user_exists = collection
        .find_one(doc! { "user_id": &user_id })
        .await?;

    if user_exists.is_none() {
        let new_user = User::_new(user_id, name);
        collection.insert_one(new_user).await?;
    }

    Ok(())
}

pub async fn login_handler(
    mongo_client: web::Data<Client>,
    login_data: web::Json<LoginData>,
) -> impl Responder {
    match store_user(mongo_client, login_data.user_id.clone(), login_data.name.clone()).await {
        Ok(_) => HttpResponse::Ok().body("User stored or already exists"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to store user"),
    }
}
