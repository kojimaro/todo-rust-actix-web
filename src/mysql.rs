use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::mysql::MysqlConnection;
use super::domain::models::{Task, NewTask};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_task(conn: &MysqlConnection, title: &String, is_complete: bool) -> Task {
    use super::schema::tasks::dsl::{id, tasks};

    let new_task = NewTask { title, is_complete };

    diesel::insert_into(tasks)
        .values(&new_task)
        .execute(conn)
        .expect("Error saving new post");

    tasks.order(id.desc()).first(conn).unwrap()
}