use axum::{ routing::get, Router};

#[tokio::main]
async fn main() {
    // build application
    let app = Router::new().route("/", get(|| async { "Hello, Axum!" }));

    // run application
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
