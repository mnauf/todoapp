pub mod schema;
pub mod models;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Task, NewTask};

pub fn create_task<'a>(conn: &PgConnection, task: &'a str) -> Task {
    use schema::tasks;

    let new_task = NewTask {
        task: task
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .get_result(conn)
        .expect("Error saving new post")
}