use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i64,
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UserCredential {
    email: String,
    password: String,
}
