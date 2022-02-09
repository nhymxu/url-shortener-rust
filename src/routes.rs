use std::str;

/* Diesel query builder */
use diesel::prelude::*;
use crate::diesel::RunQueryDsl;

/* Database macros */
use crate::schema::shorten_urls;

use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel;
use serde::Deserialize;
use rocket::response::Redirect;
use models::NewShortenUrl;

use crate::DbConn;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct NewShortUrlBody {
    url: String,
}

#[get("/")]
pub fn index() -> &'static str {
    "Application successfully started!"
}

#[get("/go/<hash_id>")]
pub fn get_url(hash_id: i32, conn: DbConn) -> Redirect {
    let result = shorten_urls::table
        .select(shorten_urls::url)
        .filter(shorten_urls::id.eq(hash_id))
        .load::<String>(&*conn)
        .expect("Something happened while retrieving full url of this id");

    let final_url = result.get(0);
    /*
    thread '<unnamed>' panicked at 'called `Option::unwrap()` on a `None` value', src/routes.rs:38:34
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */
    Redirect::to(format!("{}", final_url.unwrap()))
}

#[post("/short", format = "application/json", data = "<input>")]
pub fn new_short_url(input: Json<NewShortUrlBody>, conn: DbConn) -> String {
    let inserted_url = diesel::insert_into(shorten_urls::table)
        .values(NewShortenUrl {url: input.url.to_string()})
        .execute(&*conn);

    format!("print test {:?}", inserted_url)
}
