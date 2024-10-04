use crate::{
    models::project::ProjectData,
    utils::{parse_date, parse_user_id},
};
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
        project::{NewProject, Project},
        tasks::Tasks,
    },
};

#[post("/", data = "<project>")]
pub async fn create_project(
    db_pool: &rocket::State<DB>,
    user: JwtAuth,
    project: Json<NewProject>,
) -> Result<Json<Project>, status::Custom<Json<ErrorResponse>>> {
    let user_id = parse_user_id(user)?;

    let result = sqlx::query!(
        "INSERT INTO projects (name,user_id) VALUES (?, ?)",
        project.name,
        user_id
    )
    .execute(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::NotFound,
            Json(ErrorResponse {
                error: "Database error",
            }),
        )
    })?;

    let project = Project {
        id: result.last_insert_id() as i64,
        user_id,
        name: project.name.clone(),
    };
    Ok(Json(project))
}

#[get("/<id>/tasks")]
pub async fn get_project_tasks(
    db_pool: &rocket::State<DB>,
    user: JwtAuth,
    id: i64,
) -> Result<Json<ProjectData>, status::Custom<Json<ErrorResponse>>> {
    let user_id = parse_user_id(user)?;
    sqlx::query!(
        "SELECT id FROM projects WHERE id = ? AND user_id = ?",
        id,
        user_id
    )
    .fetch_one(db_pool.inner())
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => status::Custom(
            Status::NotFound,
            Json(ErrorResponse {
                error: "Project not found or accesse denied.",
            }),
        ),
        _ => status::Custom(
            Status::NotFound,
            Json(ErrorResponse {
                error: "Database error",
            }),
        ),
    })?;
    let records = sqlx::query!(
        "SELECT * FROM tasks WHERE project_id = ? AND user_id = ?",
        id,
        user_id
    )
    .fetch_all(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::NotFound,
            Json(ErrorResponse {
                error: "Database error",
            }),
        )
    })?;

    let tasks: Vec<Tasks> = records
        .iter()
        .map(|row| {
            let created_at = parse_date(row.created_at).expect("fiald to parse time");
            Tasks {
                id: row.id as i64,
                project_id: id,
                title: row.title.clone(),
                user_id: row.user_id as i64,
                description: row.description.clone(),
                created_at,
            }
        })
        .collect();
    let project = ProjectData {
        items: tasks.len(),
        data: tasks,
    };
    Ok(Json(project))
}

#[get("/<id>")]
pub async fn get_project(
    db_pool: &rocket::State<DB>,
    user: JwtAuth,
    id: i64,
) -> Result<Json<Project>, status::Custom<Json<ErrorResponse>>> {
    let user_id = parse_user_id(user)?;

    let record = sqlx::query!(
        "SELECT * FROM projects WHERE id = ? AND user_id = ?",
        id,
        user_id
    )
    .fetch_one(db_pool.inner())
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => status::Custom(
            Status::NotFound,
            Json(ErrorResponse {
                error: "Project not found or accesse denied.",
            }),
        ),
        _ => status::Custom(
            Status::NotFound,
            Json(ErrorResponse {
                error: "Database error.",
            }),
        ),
    })?;
    let project = Project {
        id: record.id as i64,
        user_id,
        name: record.name,
    };

    Ok(Json(project))
}

#[put("/<id>", data = "<project>")]
pub async fn update_project(
    db_pool: &rocket::State<DB>,
    user: JwtAuth,
    project: Json<NewProject>,
    id: i64,
) -> Result<Json<Project>, status::Custom<Json<ErrorResponse>>> {
    // Parse the user_id from the JWT token
    let user_id = parse_user_id(user)?;

    // Check if the project exists and belongs to the user
    sqlx::query!(
        "SELECT * FROM projects WHERE id = ? AND user_id = ?",
        id,
        user_id
    )
    .fetch_one(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::NotFound,
            Json(ErrorResponse {
                error: "Project not found or access denied",
            }),
        )
    })?;

    // Update the project details in the database
    sqlx::query!(
        "UPDATE projects SET name = ? WHERE id = ? ",
        project.name,
        id
    )
    .execute(db_pool.inner())
    .await
    .map_err(|e| {
        println!("{:?}", e);
        status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: "Failed to update the project",
            }),
        )
    })?;

    let project = Project {
        id,
        user_id,
        name: project.name.clone(),
    };
    Ok(Json(project))
}

#[delete("/<project_id>")]
pub async fn delete_project(
    db_pool: &rocket::State<DB>,
    user: JwtAuth,
    project_id: i64,
) -> Result<NoContent, status::Custom<Json<ErrorResponse>>> {
    let user_id = parse_user_id(user)?;

    // Execute the deletion query and check the result
    let result = sqlx::query!(
        "DELETE FROM projects WHERE id = ? AND user_id = ?",
        project_id,
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
                        error: "Project not found of acesses denied",
                    }),
                ))
            } else {
                Ok(NoContent) // Return 204 No Content for successful deletion
            }
        }
        Err(e) => {
            println!("{:?}", e);
            Err(status::Custom(
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: "Database Error.",
                }),
            ))
        } // Handle database error
    }
}
