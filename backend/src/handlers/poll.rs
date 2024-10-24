use crate::models::poll::Poll;
use crate::repositories::Repository;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::websocket::{broadcast_poll_update, PollUpdate};

#[derive(Deserialize)]
pub struct CreatePollData {
    pub question: String,
    pub options: Vec<String>,
    pub created_by: String,
    pub is_multiple_choice: bool,
}

// Create Poll Handler
pub async fn create_poll(
    repo: web::Data<Arc<dyn Repository>>,
    web::Json(poll_data): web::Json<CreatePollData>,
) -> impl Responder {
    let new_poll = Poll::_new(
        poll_data.question,
        poll_data.options,
        poll_data.created_by,
        poll_data.is_multiple_choice,
    );

    match repo.create_poll(new_poll).await {
        Ok(_) => HttpResponse::Ok().body("Poll created successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create poll"),
    }
}

// Get All Polls Summary Handler
pub async fn get_all_polls_summary(
    repo: web::Data<Arc<dyn Repository>>,
) -> impl Responder {
    match repo.get_all_polls().await {
        Ok(polls) => {
            let mut poll_summaries = Vec::new();

            for poll in polls {
                let user_name = match repo.find_user_by_id(&poll.created_by).await {
                    Ok(Some(user)) => user.name,
                    Ok(None) => "Unknown".to_string(),
                    Err(_) => "Unknown".to_string(),
                };

                poll_summaries.push(serde_json::json!({
                    "id": poll.id.unwrap().to_hex(),
                    "question": poll.question,
                    "created_by": user_name,
                    "created_at": poll.created_at.to_rfc3339(),
                    "isactive": poll.isactive,
                }));
            }

            HttpResponse::Ok().json(poll_summaries)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch polls"),
    }
}


// Get Poll By ID Handler
pub async fn get_poll_by_id(
    repo: web::Data<Arc<dyn Repository>>,
    poll_id: web::Path<String>,
) -> impl Responder {
    let object_id = match ObjectId::parse_str(&poll_id.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid poll ID format"),
    };

    match repo.get_poll_by_id(object_id).await {
        Ok(Some(poll)) => HttpResponse::Ok().json(poll),
        Ok(None) => HttpResponse::NotFound().body("Poll not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve poll"),
    }
}

// Get Polls by User Handler
pub async fn get_polls_by_user(
    repo: web::Data<Arc<dyn Repository>>,
    user_id: web::Path<String>,
) -> impl Responder {
    match repo.get_polls_by_user(user_id.as_str()).await {
        Ok(polls) => HttpResponse::Ok().json(polls),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve polls"),
    }
}

#[derive(Deserialize)]
pub struct ToggleStatusRequest {
    pub isactive: bool,
}

// Toggle Poll Status Handler
pub async fn toggle_poll_status(
    repo: web::Data<Arc<dyn Repository>>,
    poll_id: web::Path<String>,
    body: web::Json<ToggleStatusRequest>,
    credentials: BearerAuth,
) -> impl Responder {
    let user_id = match crate::utils::jwt::_verify_jwt(credentials.token()) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };
    let poll_id_str = poll_id.to_string();

    let poll_object_id = match ObjectId::parse_str(&poll_id_str) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid poll ID"),
    };


    match repo.get_poll_by_id(poll_object_id).await {
        Ok(Some(poll)) if poll.created_by == user_id => {
            match repo.update_poll_status(poll_object_id, body.isactive).await {
                Ok(_) => {
                    broadcast_poll_update(PollUpdate::StatusUpdate {
                        poll_id: poll_id.to_string(),
                        is_active: body.isactive,
                    })
                    .await;
                    HttpResponse::Ok().body("Poll status updated successfully")
                }
                Err(_) => HttpResponse::InternalServerError().body("Failed to update poll status"),
            }
        }
        Ok(Some(_)) => HttpResponse::Unauthorized().json(serde_json::json!({
            "message": "You are not authorized to change the status of this poll."
        })),
        Ok(None) => HttpResponse::NotFound().body("Poll not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve poll"),
    }
}

// Get Poll Results Handler
#[derive(Serialize)]
struct PollResult {
    _id: String,
    count: i32,
}

pub async fn get_poll_results(
    repo: web::Data<Arc<dyn Repository>>,
    poll_id: web::Path<String>,
) -> impl Responder {
    let poll_object_id = match ObjectId::parse_str(&poll_id.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid poll ID format"),
    };

    match repo.get_poll_results(poll_object_id).await {
        Ok(results) => {
            let poll_results: Vec<PollResult> = results
                .into_iter()
                .map(|(option_id, count)| PollResult {
                    _id: option_id.to_hex(),
                    count,
                })
                .collect();

            HttpResponse::Ok().json(poll_results)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve poll results"),
    }
}
