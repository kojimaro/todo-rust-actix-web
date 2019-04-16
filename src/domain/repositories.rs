use super::models::NewTask;
use actix_web::Error as AWError;

pub struct CreateNewTask {
    pub title: String,
}