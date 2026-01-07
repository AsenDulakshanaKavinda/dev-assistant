
//! Application composition root.
//!
//! Responsible for:
//! - Building the main Axum router
//! - Attaching routes, middleware and shared state
//!
//! This module does NOT contain
//! - Business logic
//! - Request handling code

use axum::{
    Router
};

use crate::routes::chat::create_chat_router;

/// Create and returns the main application router.
///
/// This function acts as the composition root for the HTTP layer.
/// ad wires together all routes and middleware

pub fn create_app() -> Router {
    Router::new()
        .nest("/chat", create_chat_router())

}
