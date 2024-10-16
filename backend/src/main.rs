use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
mod handlers;
mod models;

use crate::handlers::user::login_handler;
use crate::handlers::poll::{create_poll, _get_polls};
use crate::handlers::vote::submit_vote;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client_options = mongodb::options::ClientOptions::parse(&mongo_uri).await.unwrap();
    let client = mongodb::Client::with_options(client_options).unwrap();

    let app_state = web::Data::new(client);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(
                Cors::permissive()
            )
            .route("/api/login", web::post().to(login_handler))
            .route("/api/polls", web::post().to(create_poll))
            .route("/api/polls/{user_id}", web::get().to(_get_polls))
            .route("/api/votes", web::post().to(submit_vote))
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
