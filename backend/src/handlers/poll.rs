use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};
use crate::models::poll::Poll;
use crate::models::user::User;
use crate::models::vote::Vote;
use crate::utils;

#[derive(Deserialize)]
pub struct CreatePollData {
    pub question: String,
    pub options: Vec<String>,
    pub created_by: String,
    pub is_multiple_choice: bool,
}

pub async fn create_poll(
    mongo_client: web::Data<Client>,
    web::Json(poll_data): web::Json<CreatePollData>,
) -> impl Responder {
    let poll_collection = mongo_client.database("polling_app").collection::<Poll>("polls");

    let new_poll = Poll::_new(
        poll_data.question,
        poll_data.options,
        poll_data.created_by,
        poll_data.is_multiple_choice,
    );

    match poll_collection.insert_one(new_poll).await {
        Ok(_) => HttpResponse::Ok().body("Poll created successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create poll"),
    }
}

pub async fn get_all_polls_summary(mongo_client: web::Data<Client>) -> impl Responder {
    let poll_collection = mongo_client.database("polling_app").collection::<Poll>("polls");
    let user_collection = mongo_client.database("polling_app").collection::<User>("users");

    let cursor = poll_collection.find(doc! {}).await.unwrap();
    let polls: Vec<Poll> = cursor.try_collect().await.unwrap();

    let mut poll_summaries = Vec::new();
    for poll in polls {
        let user_doc = user_collection
            .find_one(doc! { "user_id": &poll.created_by })
            .await
            .unwrap();

        let user_name = user_doc.map_or("Unknown".to_string(), |user| user.name);

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

pub async fn get_poll_by_id(
    mongo_client: web::Data<Client>,
    poll_id: web::Path<String>,
) -> impl Responder {
    let object_id = match ObjectId::parse_str(&poll_id.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid poll ID format"),
    };

    let poll_collection = mongo_client.database("polling_app").collection::<Poll>("polls");

    match poll_collection.find_one(doc! { "_id": object_id }).await {
        Ok(Some(poll)) => HttpResponse::Ok().json(poll),
        Ok(None) => HttpResponse::NotFound().body("Poll not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve poll"),
    }
}

pub async fn get_polls_by_user(
    mongo_client: web::Data<Client>,
    user_id: web::Path<String>,
) -> impl Responder {
    let poll_collection = mongo_client.database("polling_app").collection::<Poll>("polls");

    let cursor = poll_collection.find(doc! { "created_by": user_id.as_str() }).await.unwrap();
    let polls: Vec<Poll> = cursor.try_collect().await.unwrap();

    HttpResponse::Ok().json(polls)
}

#[derive(Deserialize)]
pub struct ToggleStatusRequest {
    pub isactive: bool,
}

pub async fn toggle_poll_status(
    mongo_client: web::Data<Client>,
    poll_id: web::Path<String>,
    body: web::Json<ToggleStatusRequest>,
    credentials: BearerAuth,
) -> impl Responder {
    let user_id = match utils::jwt::_verify_jwt(credentials.token()) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    let collection: Collection<Poll> = mongo_client.database("polling_app").collection("polls");

    let poll_object_id = match mongodb::bson::oid::ObjectId::parse_str(&poll_id.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid poll ID"),
    };

    let poll = collection.find_one(doc! { "_id": poll_object_id }).await.unwrap();

    if let Some(poll) = poll {
        if poll.created_by != user_id {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "message": "You are not authorized to change the status of this poll."
            }));
        }
    } else {
        return HttpResponse::NotFound().body("Poll not found");
    }

    match collection
        .update_one(
            doc! { "_id": poll_object_id },
            doc! { "$set": { "isactive": body.isactive } },
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Poll status updated successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update poll status"),
    }
}

#[derive(Serialize)]
struct PollResult {
    _id: String,
    count: i32,
}

pub async fn get_poll_results(
    mongo_client: web::Data<Client>,
    poll_id: web::Path<String>,
) -> impl Responder {
    let vote_collection: Collection<Vote> = mongo_client
        .database("polling_app")
        .collection("votes");

    let poll_object_id = match ObjectId::parse_str(&poll_id.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid poll ID"),
    };

    let pipeline = vec![
        doc! { "$match": { "poll_id": poll_object_id } },
        doc! { "$unwind": "$option_ids" },
        doc! { "$group": {
            "_id": "$option_ids",
            "count": { "$sum": 1 }
        }},
    ];

    let mut cursor = vote_collection.aggregate(pipeline).await.unwrap();
    let mut results = vec![];

    while let Some(doc) = cursor.try_next().await.unwrap() {
        let option_id = doc.get_object_id("_id").unwrap().to_hex();
        let count = doc.get_i32("count").unwrap_or(0);
        results.push(PollResult { _id: option_id, count });
    }

    HttpResponse::Ok().json(results)
}