use actix_cors::Cors;
use actix_web::{web::{self}, App, HttpServer};
use repositories::Repository;
use std::sync::Arc;
use handlers::websocket::ws_handler;

mod handlers;
mod models;
mod utils;
mod repositories;
mod mongodb_repository;

use mongodb_repository::MongoDBRepository;
use utils::db::_get_database_client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = _get_database_client()
        .await
        .expect("Failed to create MongoDB client");

    let mongo_repo: Arc<dyn Repository> = Arc::new(MongoDBRepository::new(&client));

    let repo_data = web::Data::new(mongo_repo);

    HttpServer::new(move || {
        App::new()
            .app_data(repo_data.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![
                        actix_web::http::header::CONTENT_TYPE,
                        actix_web::http::header::AUTHORIZATION,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .route("/ws/{poll_id}", web::get().to(ws_handler))

            .route("/api/login", web::post().to(handlers::user::login_handler))
            .route("/api/get_user_id", web::get().to(handlers::user::get_user_id))
            .route(
                "/api/all_polls_summary",
                web::get().to(handlers::poll::get_all_polls_summary),
            )
            .route(
                "/api/polls/{poll_id}",
                web::get().to(handlers::poll::get_poll_by_id),
            )
            .route(
                "/api/polls/user/{user_id}",
                web::get().to(handlers::poll::get_polls_by_user),
            )
            .route(
                "/api/create_polls",
                web::post().to(handlers::poll::create_poll),
            )
            .route(
                "/api/vote",
                web::post().to(handlers::vote::submit_or_update_vote),
            )
            .route(
                "/api/my_votes",
                web::get().to(handlers::vote::get_voted_polls),
            )
            .route(
                "/api/reset_votes/{poll_id}",
                web::delete().to(handlers::vote::reset_votes),
            )
            .route(
                "/api/toggle_poll_status/{poll_id}",
                web::put().to(handlers::poll::toggle_poll_status),
            )
            .route(
                "/api/votes/{poll_id}/{user_id}",
                web::get().to(handlers::vote::get_vote_by_poll_and_user),
            )
            .route(
                "/api/poll_results/{poll_id}",
                web::get().to(handlers::poll::get_poll_results),
            )
    })
    .bind(("0.0.0.0", 3030))?
    .run()
    .await
}
