// @generated automatically by Diesel CLI.

diesel::table! {
    geo_provider (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        api_key -> Text,
        counter_limit -> Integer,
        counter -> Integer,
    }
}
