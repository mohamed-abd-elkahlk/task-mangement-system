use crate::handlers::project_handler::{
    create_project, delete_project, get_project, get_project_tasks, update_project,
};
use rocket::Route;
pub fn project_routes() -> Vec<Route> {
    routes![
        create_project,
        get_project,
        get_project_tasks,
        update_project,
        delete_project
    ]
}
