#[macro_use]
extern crate rocket;
use db::{db_connection, DB};
use dotenv::dotenv;

mod db;
mod handlers;
mod models;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let db_pool: DB = db_connection().await;
    rocket::build().manage(db_pool).mount("/", routes![index])
}
