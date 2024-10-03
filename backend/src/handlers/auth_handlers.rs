use rocket::{
    http::{Cookie, CookieJar, Status},
    response::status,
    serde::json::Json,
};
use sqlx::MySqlPool;

use crate::{
    auth::{
        jwt::generate_jwt,
        password::{hash_password, verify_password},
    },
    db::DB,
    models::{
        error::ErrorResponse,
        user::{NewUser, User, UserCredential},
    },
};

#[post("/sign-up", data = "<new_user>")]
pub async fn sign_up<'a>(
    db_pool: &rocket::State<DB>,
    cookies: &CookieJar<'_>,
    new_user: Json<NewUser>,
) -> Result<Json<User>, status::Custom<Json<ErrorResponse<'a>>>> {
    let password = hash_password(&new_user.password).map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: "failed to hash the password",
            }),
        )
    })?;
    let result = sqlx::query!(
        "INSERT INTO users (email, username, password) VALUES ( ?, ?, ?)",
        new_user.email,
        new_user.username,
        password
    )
    .execute(db_pool.inner())
    .await
    .map_err(|e| {
        println!("{:?}", e);
        status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: "Database Error",
            }),
        )
    })?;

    let user = User {
        id: result.last_insert_id() as i64,
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        role: "user".to_string(),
    };

    let token = generate_jwt(&user.id.to_string(), &user.role).map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: "failed to create jwt token",
            }),
        )
    })?;

    cookies.add(Cookie::build(("auth_token", token)));

    Ok(Json(user))
}

#[post("/sign-in", data = "<user>")]
pub async fn sign_in<'a>(
    db_pool: &rocket::State<MySqlPool>,
    cookies: &CookieJar<'_>,
    user: Json<UserCredential>,
) -> Result<Json<User>, status::Custom<Json<ErrorResponse<'a>>>> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)",
        user.email
    )
    .fetch_one(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: "Database Error.",
            }),
        )
    })?;
    if exists == 0 {
        return Err(status::Custom(
            Status::Unauthorized,
            Json(ErrorResponse {
                error: "email or password are incrrocet",
            }),
        ));
    }

    let record = sqlx::query!("SELECT * FROM users WHERE email = ?", user.email)
        .fetch_one(db_pool.inner())
        .await
        .unwrap();
    let password_varifcation = verify_password(&user.password, &record.password);
    if !password_varifcation {
        return Err(status::Custom(
            Status::Unauthorized,
            Json(ErrorResponse {
                error: "email or password are incrrocet",
            }),
        ));
    }

    let user = User {
        id: record.id as i64,
        username: record.username.clone(),
        email: record.email.clone(),
        role: "user".to_string(),
    };

    let token = generate_jwt(&user.id.to_string(), &user.role).map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: "failed to create jwt token",
            }),
        )
    })?;

    cookies.add(Cookie::build(("auth_token", token)));

    Ok(Json(user))
}
