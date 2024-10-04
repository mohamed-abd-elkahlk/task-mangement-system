use serde::{Deserialize, Serialize};

use super::tasks::Tasks;

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct NewProject {
    pub name: String,
}

#[derive(Serialize)]
pub struct ProjectData {
    pub items: usize,
    pub data: Vec<Tasks>,
}
