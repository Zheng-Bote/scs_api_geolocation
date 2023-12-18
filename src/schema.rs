// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Nullable<BigInt>,
        description -> Text,
        installed_on -> Timestamp,
        success -> Bool,
        checksum -> Binary,
        execution_time -> BigInt,
    }
}

diesel::table! {
    geo_provider (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        api_key -> Text,
        counter_limit -> Text,
        counter -> Text,
        date_time -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    geo_provider,
);
