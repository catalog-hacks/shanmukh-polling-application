use crate::models::vote::Vote;
use crate::repositories::Repository;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use std::sync::Arc;

use super::websocket::{broadcast_poll_update, PollUpdate, VoteResult};

#[derive(Deserialize)]
pub struct VoteData {
    pub poll_id: String,
    pub option_ids: Vec<String>,
}

// Get Voted Polls Handler
pub async fn get_voted_polls(
    repo: web::Data<Arc<dyn Repository>>,
    auth: BearerAuth,
) -> impl Responder {
    let user_id = match crate::utils::jwt::_verify_jwt(auth.token()) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    let votes = match repo.find_votes_by_user(&user_id).await {
        Ok(votes) => votes,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to retrieve votes"),
    };

    let poll_ids: Vec<ObjectId> = votes.iter().map(|vote| vote.poll_id).collect();

    let voted_polls = match repo.find_polls_by_ids(poll_ids).await {
        Ok(polls) => polls,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to retrieve polls"),
    };

    HttpResponse::Ok().json(voted_polls)
}

// Get Vote By Poll and User Handler
pub async fn get_vote_by_poll_and_user(
    repo: web::Data<Arc<dyn Repository>>,
    params: web::Path<(String, String)>,
    auth: BearerAuth,
) -> impl Responder {
    let (poll_id, user_id) = params.into_inner();
    let token_user_id = match crate::utils::jwt::_verify_jwt(auth.token()) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    if token_user_id != user_id {
        return HttpResponse::Unauthorized().body("Unauthorized access");
    }

    let poll_object_id = match ObjectId::parse_str(&poll_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid poll ID format"),
    };

    match repo.find_vote(poll_object_id, &user_id).await {
        Ok(Some(vote)) => HttpResponse::Ok().json(vote),
        Ok(None) => HttpResponse::NotFound().body("No vote found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve vote"),
    }
}

// Submit or Update Vote Handler
pub async fn submit_or_update_vote(
    repo: web::Data<Arc<dyn Repository>>,
    vote_data: web::Json<VoteData>,
    auth: BearerAuth,
) -> impl Responder {
    let user_id = match crate::utils::jwt::_verify_jwt(auth.token()) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    let poll_object_id = match ObjectId::parse_str(&vote_data.poll_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid poll ID format"),
    };

    let option_ids: Vec<ObjectId> = match vote_data
        .option_ids
        .iter()
        .map(|id| ObjectId::parse_str(id))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(ids) => ids,
        Err(_) => return HttpResponse::BadRequest().body("Invalid option ID format"),
    };

    let vote = Vote {
        id: None,
        poll_id: poll_object_id,
        user_id: user_id.clone(),
        option_ids,
    };



    match repo.submit_or_update_vote(vote).await {
        Ok(_) => {
            if let Ok(results) = repo.get_poll_results(poll_object_id).await {
                let results_vec: Vec<VoteResult> = results
                    .into_iter()
                    .map(|(id, count)| VoteResult {
                        _id: id.to_hex(),
                        count,
                    })
                    .collect();

                broadcast_poll_update(PollUpdate::VoteUpdate {
                    poll_id: vote_data.poll_id.clone(),
                    results: results_vec,
                })
                .await;
            }
            HttpResponse::Ok().body("Vote submitted successfully")
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to submit vote"),
    }
}

// Reset Votes Handler
pub async fn reset_votes(
    repo: web::Data<Arc<dyn Repository>>,
    poll_id: web::Path<String>,
    auth: BearerAuth,
) -> impl Responder {
    let user_id = match crate::utils::jwt::_verify_jwt(auth.token()) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    let poll_id_str = poll_id.to_string();

    let poll_object_id = match ObjectId::parse_str(&poll_id_str) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid poll ID format"),
    };

    match repo.get_poll_by_id(poll_object_id).await {
        Ok(Some(poll)) if poll.created_by == user_id => {
            match repo.reset_votes_for_poll(poll_object_id).await {
                Ok(_) => {
                    broadcast_poll_update(PollUpdate::Reset {
                        poll_id: poll_id_str,
                    })
                    .await;
                    HttpResponse::Ok().body("Votes reset successfully")
                }
                Err(_) => HttpResponse::InternalServerError().body("Failed to reset votes"),
            }
        }
        Ok(Some(_)) => HttpResponse::Unauthorized().json(serde_json::json!({
            "message": "You are not authorized to reset votes for this poll."
        })),
        Ok(None) => HttpResponse::NotFound().body("Poll not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve poll"),
    }
}
