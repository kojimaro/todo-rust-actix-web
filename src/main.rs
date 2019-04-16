pub mod domain;
pub mod api;
pub mod mysql;
mod schema;

extern crate serde;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;

use actix_web::{server, App, http};
use api::task::{create};

fn main() {
    let app = move || {
        App::new().resource("/tasks", |r| r.method(http::Method::POST).with(create))
    };

    server::new(app)
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}