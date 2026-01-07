mod routes;
mod handlers;

use axum::{
    Router
};

use crate::routes::chat::create_chat_router;


#[tokio::main]
async fn main() {
    // make single router
    let app = Router::new().nest("/chat", create_chat_router());


    // run our app with hyper, listening globally on port 4000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


