use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Tasks {
    pub id: i64,
    pub user_id: i64,
    pub project_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}
#[derive(Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
}
#[derive(Deserialize)]
pub struct UpdatedTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub project_id: Option<i64>,
}
