diesel::table! {
    shorten_urls (id) {
        id -> Text,
        url -> Text,
    }
}
