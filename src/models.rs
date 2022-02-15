use crate::schema::shorten_urls;

use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct ShortenUrls {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name="shorten_urls"]
pub struct NewShortenUrl {
    pub id: String,
    pub url: String,
}
