
use axum::{Json, http::StatusCode};
use serde::Deserialize;

use crate::services::mistral_service::send_to_mistral;

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

// send payload to the AI, and return the response
pub async fn chat_handler(Json(payload): Json<ChatRequest>) -> Result<Json<String>, StatusCode> {
    match send_to_mistral(payload.message).await {
        Ok(reply) => Ok(Json(reply)),
        Err(err) => {
            eprintln!("Mistral error: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }

    }
}