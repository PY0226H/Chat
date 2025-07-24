use axum::{Extension, response::IntoResponse};
use tracing::info;

use crate::User;
pub(crate) async fn list_chat_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    // Handler logic for listing chats
    info!("user: {:?}", user);
    "List of chats"
}

pub(crate) async fn create_chat_handler() -> impl IntoResponse {
    // Handler logic for creating a chat
}

pub(crate) async fn update_handler() -> impl IntoResponse {
    // Handler logic for updating a chat
}

pub(crate) async fn delete_chat_handler() -> impl IntoResponse {
    // Handler logic for deleting a chat
}
