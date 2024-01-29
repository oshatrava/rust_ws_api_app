use std::sync::Arc;

use tower_http::services::ServeDir;
use axum::{
    routing::{get, post}, 
    Router,
};

use crate::AppState;
use crate::handlers::notes_handler::{
    create_note_handler, delete_note_handler, edit_note_handler, 
    get_note_handler, list_note_handler, health_check_handler,
};
use crate::handlers::wsocket_handler::wsocket_handler;
use crate::handlers::rooms_handler::list_rooms_handler;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_router = Router::new()
        .route("/health-check", get(health_check_handler))
        .route("/notes/", post(create_note_handler))
        .route("/notes", get(list_note_handler))
        .route(
            "/notes/:id",
            get(get_note_handler)
                .patch(edit_note_handler)
                .delete(delete_note_handler),
        );

    let api_rooms = Router::new()
        .route("/rooms", get(list_rooms_handler));

    Router::new()
        .fallback_service(
            ServeDir::new("public")
                .append_index_html_on_directories(true)
        )
        .route("/ws", get(wsocket_handler))
        .nest("/api", api_router)
        .nest("/api", api_rooms)
        .with_state(app_state)
}

