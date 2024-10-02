use std::env;

use sqlx::{MySql, Pool};

pub type DB = Pool<MySql>;
pub async fn db_connection() -> DB {
    let url = env::var("DATABASE_URL").expect("please provide a DATABASEURL in .env file");
    sqlx::MySqlPool::connect(&url).await.unwrap()
}
