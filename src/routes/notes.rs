use std::sync::Arc;
use serde_json::json;
use axum::{
    Router,
    routing::{get, post, patch, delete},
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::app::AppState;
use crate::models::note::{NoteModel, CreateNoteSchema, UpdateNoteSchema, Pagination};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/notes/", post(create_notes))
        .route("/notes", get(get_notes))
        .route("/notes/:id", get(get_note_by_id))
        .route("/notes/:id", patch(update_note_by_id))
        .route("/notes/:id", delete(delete_note_by_id))
}

async fn get_notes(
    opts: Option<Query<Pagination>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit;
    let offset = (opts.page - 1) * limit;

    let notes = sqlx::query_as!(
        NoteModel,
        "SELECT * FROM notes ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32,
    )
    .fetch_all(&data.db)
    .await;

    match notes {
        Ok(notes) => {
            let response = json!({
                "status": "success",
                "data": notes,
            });
        
            Ok(Json(response))
        }
        Err(_) => {
            let error = json!({
                "status": "error",
                "message": "Something bad happened while fetching all note items",
            });

            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
        }
    }
}

async fn create_notes(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        NoteModel,
        "INSERT INTO notes (title, content, category) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".to_string()),
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let response = json!({
                "status": "success",
                "data": note,
            });

            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(err) => {
            if err
                .to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = json!({
                    "status": "error",
                    "message": "Note with that title already exists",
                });

                return Err((StatusCode::CONFLICT, Json(error_response)));
            }

            let error_response = json!({
                "status": "error",
                "message": format!("{:?}", err),
            });

            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

async fn get_note_by_id(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let note = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id,)
        .fetch_one(&data.db)
        .await
        .map_err(|err| {
            let error_response = json!({
                "status": "error",
                "message": format!("Database error: {}", err),
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        });

    match note {
        Ok(note) => {
            let response = json!({
                "status": "success",
                "data": note,
            });

            Ok(Json(response))
        }
        Err(_) => {
            let error = json!({
                "status": "error",
                "message": format!("Note with ID: {:?} not found", id),
            });

            Err((StatusCode::NOT_FOUND, Json(error)))
        }
    }
}

async fn update_note_by_id(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let note = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id,)
        .fetch_one(&data.db)
        .await;
    
    if note.is_err() {
        let error = json!({
            "status": "error",
            "messsage": format!("Note with ID: {} not found", id),
        });

        return Err((StatusCode::NOT_FOUND, Json(error)));
    }

    let note = note.unwrap();
    let now = chrono::Utc::now();

    let note = sqlx::query_as!(
        NoteModel,
        "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *",
        body.title.to_owned().unwrap_or(note.title),
        body.content.to_owned().unwrap_or(note.content),
        body.category.to_owned().unwrap_or(note.category.unwrap()),
        body.published.unwrap_or(note.published.unwrap()),
        now,
        id,
    )
    .fetch_one(&data.db)
    .await;

    match note {
        Ok(note) => {
            let response = json!({
                "status": "success",
                "data": note,
            });

            Ok(Json(response))
        }
        Err(err) => {
            let error = json!({
                "status": "error",
                "messsage": format!("{:?}", err)
            });

            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
        }
    }
}

async fn delete_note_by_id(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query_as!(NoteModel, "DELETE FROM notes WHERE id = $1", id,)
        .execute(&data.db)
        .await;

    let rows_affected = result.unwrap().rows_affected();
    if rows_affected == 0 {
        let error = json!({
            "status": "error",
            "message": format!("Note with ID: {} not found", id),
        });

        return Err((StatusCode::NOT_FOUND, Json(error)));
    }

    let response = json!({
        "status": "success",
    });

    Ok(Json(response))
}
