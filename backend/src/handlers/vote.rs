use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::doc, Client};
use serde::Deserialize;
use crate::models::vote::Vote;

#[derive(Deserialize)]
pub struct VoteData {
    pub poll_id: String,
    pub option_index: usize,
}

pub async fn submit_vote(
    mongo_client: web::Data<Client>,
    user_id: String,
    vote_data: web::Json<VoteData>,
) -> impl Responder {
    let collection = mongo_client
        .database("polling_app")
        .collection::<Vote>("votes");

    let new_vote = Vote::_new(user_id, vote_data.poll_id.clone(), vote_data.option_index);

    match collection.insert_one(new_vote).await {
        Ok(_) => HttpResponse::Ok().body("Vote submitted"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to submit vote"),
    }
}
