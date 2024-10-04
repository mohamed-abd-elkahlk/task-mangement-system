#[macro_use]
extern crate rocket;
use db::{db_connection, DB};
use dotenv::dotenv;
use rocket::{Build, Rocket};
use routes::{
    auth_routes,
    project_routes::{self, project_routes},
    tasks_routes,
};
mod auth;
mod db;
mod guards;
mod handlers;
mod models;
mod routes;
mod utils;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let db_pool: DB = db_connection().await;
    rocket::build()
        .manage(db_pool)
        .mount("/auth", auth_routes::auth_routes())
        .mount("/task", tasks_routes::tasks_routes())
        .mount("/project", project_routes::project_routes())
}
