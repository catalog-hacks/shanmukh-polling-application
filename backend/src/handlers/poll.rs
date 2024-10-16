#[path = "../models/mod.rs"] mod models;
use actix_web::{web, HttpResponse, Responder};
use futures::TryStreamExt;
use mongodb::{bson::doc, Client};
use serde::Deserialize;
use crate::models::poll::Poll;

#[derive(Deserialize, Debug)]
pub struct CreatePollData {
    pub question: String,
    pub options: Vec<String>,
}

pub async fn create_poll(
    mongo_client: web::Data<Client>,
    user_id: String,
    poll_data: web::Json<CreatePollData>,
) -> impl Responder {
    println!("Received poll data: {:?}", poll_data);
    let collection = mongo_client
        .database("polling_app")
        .collection::<Poll>("polls");

    let new_poll = Poll::_new(user_id, poll_data.question.clone(), poll_data.options.clone());

    match collection.insert_one(new_poll).await {
        Ok(_) => HttpResponse::Ok().body("Poll created"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create poll"),
    }
}

pub async fn _get_polls(
    mongo_client: web::Data<Client>,
    user_id: web::Path<String>,
) -> impl Responder {
    let collection = mongo_client
        .database("polling_app")
        .collection::<Poll>("polls");

    let cursor = collection
        .find(doc! { "user_id": user_id.as_str() })
        .await
        .unwrap();

    let polls: Vec<Poll> = cursor.try_collect().await.unwrap();

    HttpResponse::Ok().json(polls)
}