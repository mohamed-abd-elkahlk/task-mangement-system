use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rocket::{http::Status, response::status, serde::json::Json, time::PrimitiveDateTime};

use crate::{guards::jwt_guard::JwtAuth, models::error::ErrorResponse};

pub fn parse_user_id<'a>(user: JwtAuth) -> Result<i64, status::Custom<Json<ErrorResponse<'a>>>> {
    user.claims.sub.parse::<i64>().map_err(|_| {
        status::Custom(
            Status::Unauthorized,
            Json(ErrorResponse {
                error: "Invalid user ID in token",
            }),
        )
    })
}

pub fn parse_date<'a>(
    primitive_dt: PrimitiveDateTime,
) -> Result<DateTime<Utc>, status::Custom<Json<ErrorResponse<'a>>>> {
    // Convert `rocket::time::Date` to `chrono::NaiveDate`
    let date = primitive_dt.date();
    let naive_date = NaiveDate::from_ymd_opt(date.year(), date.month() as u32, date.day() as u32)
        .ok_or_else(|| {
        status::Custom(
            Status::NotFound,
            Json(ErrorResponse {
                error: "Database error",
            }),
        )
    })?;

    // Convert `rocket::time::Time` to `chrono::NaiveTime`
    let time = primitive_dt.time();
    let naive_time = NaiveTime::from_hms_opt(
        time.hour() as u32,
        time.minute() as u32,
        time.second() as u32,
    )
    .ok_or_else(|| {
        status::Custom(
            Status::NotFound,
            Json(ErrorResponse {
                error: "Database error",
            }),
        )
    })?;

    // Combine `NaiveDate` and `NaiveTime` to create `NaiveDateTime`
    let naive_datetime = NaiveDateTime::new(naive_date, naive_time);

    // Convert `NaiveDateTime` to `DateTime<Utc>` using `from_naive_utc_and_offset`
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(
        naive_datetime,
        Utc,
    ))
}
