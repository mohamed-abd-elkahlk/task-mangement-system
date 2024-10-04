use crate::handlers::task_handler::{create_task, delete_task, get_tasks, update_task};
use rocket::Route;
pub fn tasks_routes() -> Vec<Route> {
    routes![create_task, get_tasks, update_task, delete_task]
}
