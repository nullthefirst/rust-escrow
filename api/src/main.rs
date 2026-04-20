use dotenv::dotenv;
use axum::{Router, routing::{get, post}};

mod config;
mod db;
mod routes;
mod handlers;
mod models;
mod services;

#[tokio::main]
async fn main() {
    // setup app config
    dotenv().ok();

    let config = config::Config::from_env();
    let db = db::init_db(&config.mongodb_uri, &config.db_name).await;

    // build application
    let app = routes::create_routes(db, config);

    // run application
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
