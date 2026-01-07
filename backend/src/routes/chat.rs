
use axum::{Router, routing::post};


use crate::handlers::chat_handler::chat_handler;


pub fn create_chat_router() -> Router {
    let chat_router = Router::new()
        .route("/", post(chat_handler));
    
    
    chat_router
}


