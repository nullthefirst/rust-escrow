use dotenv::dotenv;

use crate::handlers::AppState;

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

    // single state object for dependencies
    let state = AppState {
        db: db,
        config: config,
    };

    // build application
    let app = routes::create_routes(state);

    // run application
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
