diesel::table! {
    shorten_urls (id) {
        id -> Integer,
        url -> Text,
    }
}
