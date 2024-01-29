use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: uuid::Uuid,
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
