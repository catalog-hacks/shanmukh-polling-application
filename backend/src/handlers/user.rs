use crate::models::user::User;
use crate::repositories::Repository;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct LoginData {
    pub user_id: String,
    pub name: String,
}

// Store or Login User Handler
pub async fn login_handler(
    repo: web::Data<Arc<dyn Repository>>,
    web::Json(login_data): web::Json<LoginData>,
) -> impl Responder {
    let user = User {
        id: None,
        user_id: login_data.user_id.clone(),
        name: login_data.name.clone(),
    };

    match repo.store_user(user).await {
        Ok(_) => match crate::utils::jwt::_create_jwt(&login_data.user_id) {
            Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "token": token })),
            Err(_) => HttpResponse::InternalServerError().body("Failed to create JWT"),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Get User ID from JWT
pub async fn get_user_id(auth: BearerAuth) -> impl Responder {
    match crate::utils::jwt::_verify_jwt(auth.token()) {
        Ok(user_id) => HttpResponse::Ok().json(serde_json::json!({ "user_id": user_id })),
        Err(_) => HttpResponse::Unauthorized().body("Invalid or expired token"),
    }
}
