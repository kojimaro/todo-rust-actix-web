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

use actix_web::{server, App, http::Method};
use api::task::{index, store};

fn main() {
    let app = move || {
        App::new()
            .resource("/tasks", |r| r.method(Method::GET).f(index))
            .resource("/task", |r| r.method(Method::POST).with(store))
    };

    server::new(app)
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}