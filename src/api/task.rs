use super::domain::models::Task;
use super::mysql;
use actix_web::{Json, Result};

pub fn create(request: Json<Task>) -> Result<String> {
    let connection = mysql::establish_connection();

    let title = request.title.to_string();
    let is_complete = request.is_complete;

    mysql::create_task(&connection, &title, &is_complete);

    Ok(request.title.to_string())
}