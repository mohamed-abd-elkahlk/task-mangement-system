use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct UserCredential {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}
