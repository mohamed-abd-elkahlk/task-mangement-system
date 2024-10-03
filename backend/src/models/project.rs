use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct NewProject {
    pub name: String,
}
