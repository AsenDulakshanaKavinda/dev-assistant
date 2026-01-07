use axum::{
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
    // make single router
    let app = Router::new().route("/chat", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 4000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


