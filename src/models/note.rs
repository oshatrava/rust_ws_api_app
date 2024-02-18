use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct NoteModel {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub published: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNoteSchema {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNoteSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub published: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub page: usize,
    pub limit: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { 
            page: 1,
            limit: 10,
        }
    }
}