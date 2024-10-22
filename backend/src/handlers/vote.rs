use crate::utils;
use crate::models::vote::Vote;
use crate::models::poll::Poll;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::TryStreamExt;
use mongodb::Collection;
use mongodb::{
    bson::doc,
    Client,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VoteData {
    pub poll_id: String,
    pub option_ids: Vec<String>,
}

pub async fn get_voted_polls(
    mongo_client: web::Data<Client>,
    auth: BearerAuth,
) -> impl Responder {
    let user_id = match utils::jwt::_verify_jwt(auth.token()) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    let votes_collection = mongo_client
        .database("polling_app")
        .collection::<Vote>("votes");

    let poll_collection = mongo_client
        .database("polling_app")
        .collection::<Poll>("polls");

    let cursor = match votes_collection
        .find(doc! { "user_id": &user_id })
        .await
    {
        Ok(cursor) => cursor,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to retrieve votes"),
    };

    let votes: Vec<Vote> = match cursor.try_collect().await {
        Ok(votes) => votes,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to collect votes"),
    };

    let poll_ids: Vec<_> = votes.iter().map(|vote| vote.poll_id).collect();

    let cursor = match poll_collection
        .find(doc! { "_id": { "$in": poll_ids } })
        .await
    {
        Ok(cursor) => cursor,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to retrieve polls"),
    };

    let voted_polls: Vec<Poll> = match cursor.try_collect().await {
        Ok(polls) => polls,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to collect polls"),
    };

    HttpResponse::Ok().json(voted_polls)
}

pub async fn get_vote_by_poll_and_user(
    mongo_client: web::Data<Client>,
    params: web::Path<(String, String)>,
    auth: BearerAuth,
) -> impl Responder {
    let (poll_id, user_id) = params.into_inner();

    let token_user_id = match utils::jwt::_verify_jwt(auth.token()) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    if token_user_id != user_id {
        return HttpResponse::Unauthorized().body("Unauthorized user");
    }

    let poll_object_id = match mongodb::bson::oid::ObjectId::parse_str(&poll_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid poll ID format"),
    };

    let vote_collection = mongo_client.database("polling_app").collection::<Vote>("votes");

    match vote_collection
        .find_one(doc! { "poll_id": poll_object_id, "user_id": &user_id })
        .await
    {
        Ok(Some(vote)) => HttpResponse::Ok().json(vote),
        Ok(None) => HttpResponse::NotFound().body("No vote found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve vote"),
    }
}


pub async fn submit_or_update_vote(
    mongo_client: web::Data<Client>,
    vote_data: web::Json<VoteData>,
    auth: BearerAuth,
) -> impl Responder {
    let user_id = match utils::jwt::_verify_jwt(auth.token()) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    let poll_id = match mongodb::bson::oid::ObjectId::parse_str(&vote_data.poll_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid poll ID format"),
    };

    let option_ids: Vec<_> = vote_data
        .option_ids
        .iter()
        .filter_map(|id| mongodb::bson::oid::ObjectId::parse_str(id).ok())
        .collect();

    let vote_collection = mongo_client.database("polling_app").collection::<Vote>("votes");

    let existing_vote = vote_collection
        .find_one(doc! { "poll_id": poll_id, "user_id": &user_id })
        .await
        .unwrap();

    if let Some(_) = existing_vote {
        let update_result = vote_collection
            .update_one(
                doc! { "poll_id": poll_id, "user_id": &user_id },
                doc! { "$set": { "option_ids": option_ids } },
            )
            .await;

        match update_result {
            Ok(_) => HttpResponse::Ok().body("Vote updated successfully"),
            Err(_) => HttpResponse::InternalServerError().body("Failed to update vote"),
        }
    } else {
        let new_vote = Vote::_new(poll_id, option_ids, user_id);

        match vote_collection.insert_one(new_vote).await {
            Ok(_) => HttpResponse::Ok().body("Vote submitted successfully"),
            Err(_) => HttpResponse::InternalServerError().body("Failed to submit vote"),
        }
    }
}

pub async fn reset_votes(
    mongo_client: web::Data<Client>,
    poll_id: web::Path<String>,
    auth: BearerAuth,
) -> impl Responder {
    let user_id = match utils::jwt::_verify_jwt(auth.token()) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    let poll_object_id = match mongodb::bson::oid::ObjectId::parse_str(&poll_id.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid poll ID"),
    };

    let poll_collection: Collection<Poll> = mongo_client.database("polling_app").collection("polls");

    let poll = poll_collection.find_one(doc! { "_id": poll_object_id }).await.unwrap();

    if let Some(poll) = poll {
        if poll.created_by != user_id {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "message": "You are not authorized to reset votes for this poll."
            }));
        }
    } else {
        return HttpResponse::NotFound().body("Poll not found");
    }

    let vote_collection: Collection<Vote> = mongo_client.database("polling_app").collection("votes");

    match vote_collection.delete_many(doc! { "poll_id": poll_object_id }).await {
        Ok(_) => HttpResponse::Ok().body("Votes reset successfully"),
        Err(err) => {
            eprintln!("Error resetting votes: {}", err);
            HttpResponse::InternalServerError().body("Failed to reset votes")
        }
    }
}