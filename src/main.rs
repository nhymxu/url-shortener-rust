#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use rocket_contrib::database;

pub mod models;
pub mod routes;
pub mod schema;

#[database("shorten_urls")]
pub struct DbConn(diesel::SqliteConnection);

fn main() {
    rocket::ignite()
        .mount("/", routes![
            routes::index,
            routes::get_url,
            routes::new_short_url,
        ])
        .attach(DbConn::fairing())
        .launch();
}
