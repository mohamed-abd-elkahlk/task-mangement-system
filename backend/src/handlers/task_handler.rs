use chrono::Utc;
use rocket::{
    http::Status,
    response::status::{self, NoContent},
    serde::json::Json,
};

use crate::{
    db::DB,
    guards::jwt_guard::JwtAuth,
    models::{
        error::ErrorResponse,
        tasks::{NewTask, Tasks, UpdatedTask},
    },
    utils::{parse_date, parse_user_id},
};

#[get("/<id>")]
pub async fn get_tasks(
    db_pool: &rocket::State<DB>,
    user: JwtAuth,
    id: i64,
) -> Result<Json<Tasks>, status::Custom<Json<ErrorResponse>>> {
    let user_id = parse_user_id(user)?;
    let record = sqlx::query!(
        "SELECT * FROM tasks WHERE id = ? AND user_id = ?",
        id,
        user_id
    )
    .fetch_one(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::NotFound,
            Json(ErrorResponse {
                error: "Task not found or access denied",
            }),
        )
    })?;
    let created_at = parse_date(record.created_at)?;
    let task = Tasks {
        id: record.id as i64,
        project_id: record.project_id.expect("msg") as i64,
        user_id,
        title: record.title,
        description: record.description,
        created_at,
    };
    Ok(Json(task))
}

#[post("/?<project_id>", data = "<task>")]
pub async fn create_task(
    db_pool: &rocket::State<DB>,
    user: JwtAuth,
    project_id: i64,
    task: Json<NewTask>,
) -> Result<Json<Tasks>, status::Custom<Json<ErrorResponse>>> {
    let user_id = parse_user_id(user)?;
    let result = sqlx::query!(
        "INSERT INTO tasks (user_id , project_id, title,description) VALUES (?,?,?,?)",
        user_id,
        project_id,
        task.title,
        Some(task.description.clone()),
    )
    .execute(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::NotFound,
            Json(ErrorResponse {
                error: "row not found",
            }),
        )
    })?;
    let new_task = Tasks {
        id: result.last_insert_id() as i64,
        user_id,
        project_id,
        title: task.title.clone(),
        description: task.description.clone(),
        created_at: Utc::now(),
    };
    Ok(Json(new_task))
}
#[put("/<task_id>", data = "<task>")]
pub async fn update_task(
    db_pool: &rocket::State<DB>,
    user: JwtAuth,
    task_id: i64,
    task: Json<UpdatedTask>,
) -> Result<Json<Tasks>, status::Custom<Json<ErrorResponse>>> {
    let user_id = parse_user_id(user)?;

    // Fetch the existing task
    let existing_task = sqlx::query!(
        "SELECT id, user_id, project_id, title, description, created_at 
         FROM tasks WHERE id = ? AND user_id = ?",
        task_id,
        user_id
    )
    .fetch_one(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::NotFound,
            Json(ErrorResponse {
                error: "Task not found or access denied",
            }),
        )
    })?;

    // Check if the new project_id exists if provided
    if let Some(project_id) = task.project_id {
        let project_exists = sqlx::query!("SELECT id FROM projects WHERE id = ?", project_id)
            .fetch_optional(db_pool.inner())
            .await
            .map_err(|_| {
                status::Custom(
                    Status::InternalServerError,
                    Json(ErrorResponse {
                        error: "Failed to check project existence",
                    }),
                )
            })?;

        if project_exists.is_none() {
            return Err(status::Custom(
                Status::NotFound,
                Json(ErrorResponse {
                    error: "Project not found",
                }),
            ));
        }
    }

    // Merge new values or keep the old ones
    let updated_title = task.title.clone().unwrap_or(existing_task.title);
    let updated_description = task.description.clone().or(existing_task.description);
    let updated_project_id = task
        .project_id
        .map(|pid| pid as i64)
        .or(existing_task.project_id.map(|pid| pid as i64));

    // Update the task
    sqlx::query!(
        "UPDATE tasks SET title = ?, description = ?, project_id = ? WHERE id = ? AND user_id = ?",
        updated_title,
        updated_description,
        updated_project_id,
        task_id,
        user_id
    )
    .execute(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: "Failed to update the task",
            }),
        )
    })?;

    let updated_task = Tasks {
        id: existing_task.id as i64,
        user_id: existing_task.user_id as i64,
        project_id: updated_project_id.unwrap_or(existing_task.project_id.unwrap() as i64),
        title: updated_title,
        description: updated_description,
        created_at: parse_date(existing_task.created_at)?,
    };

    Ok(Json(updated_task))
}

#[delete("/<task_id>")]
pub async fn delete_task(
    db_pool: &rocket::State<DB>,
    user: JwtAuth,
    task_id: i64,
) -> Result<NoContent, status::Custom<Json<ErrorResponse>>> {
    let user_id = parse_user_id(user)?;

    // Execute the deletion query and check the result
    let result = sqlx::query!(
        "DELETE FROM tasks WHERE id = ? AND user_id = ?",
        task_id,
        user_id
    )
    .execute(db_pool.inner())
    .await;

    match result {
        Ok(query_result) => {
            if query_result.rows_affected() == 0 {
                // No task was deleted, meaning task was not found
                Err(status::Custom(
                    Status::NotFound,
                    Json(ErrorResponse {
                        error: "Taks not found or acesses denied",
                    }),
                ))
            } else {
                Ok(NoContent) // Return 204 No Content for successful deletion
            }
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: "Database Error",
            }),
        )), // Handle database error
    }
}
