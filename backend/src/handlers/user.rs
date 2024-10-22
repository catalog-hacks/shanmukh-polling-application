use crate::utils;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::{bson::doc, Client};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginData {
    pub user_id: String,
    pub name: String,
}

async fn store_user(
    mongo_client: web::Data<Client>,
    user_id: &str,
    name: &str,
) -> mongodb::error::Result<()> {
    let collection = mongo_client.database("polling_app").collection("users");

    if collection.find_one(doc! { "user_id": user_id }).await?.is_none() {
        let new_user = doc! { "user_id": user_id, "name": name };
        collection.insert_one(new_user).await?;
    }
    Ok(())
}

pub async fn login_handler(
    mongo_client: web::Data<Client>,
    web::Json(login_data): web::Json<LoginData>,
) -> impl Responder {
    match store_user(mongo_client, &login_data.user_id, &login_data.name).await {
        Ok(_) => {
            match utils::jwt::_create_jwt(&login_data.user_id) {
                Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "token": token })),
                Err(_) => HttpResponse::InternalServerError().body("Failed to create JWT"),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_user_id(auth: BearerAuth) -> impl Responder {
    match utils::jwt::_verify_jwt(auth.token()) {
        Ok(user_id) => HttpResponse::Ok().json(serde_json::json!({ "user_id": user_id })),
        Err(_) => HttpResponse::Unauthorized().body("Invalid or expired token"),
    }
}