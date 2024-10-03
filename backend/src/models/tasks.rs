use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::project::Project;

#[derive(Deserialize, Serialize)]
pub struct Tasks {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub project: Project,
}
#[derive(Deserialize)]
pub struct NewTask {
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub project: Project,
}
