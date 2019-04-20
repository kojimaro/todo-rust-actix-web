use super::diesel::prelude::*;
use super::domain::models::Task;
use super::mysql;
use actix_web::{Json, Result, HttpRequest, HttpResponse};

pub fn index(request: &HttpRequest) -> HttpResponse {
    use super::schema::tasks::dsl::*;

    let connection = mysql::establish_connection();

    let results = tasks.load::<Task>(&connection).expect("Error loading tasks");

    HttpResponse::Ok().json(results)
}

pub fn store(request: Json<Task>) -> Result<String> {
    let _connection = mysql::establish_connection();

    let _title = request.title.to_string();
    let is_complete = request.is_complete;

    mysql::create_task(&_connection, &_title, &is_complete);

    Ok(is_complete.to_string())
}