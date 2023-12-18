CREATE TABLE IF NOT EXISTS geo_provider
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    api_key TEXT NOT NULL,
    counter_limit TEXT NOT NULL,
    counter TEXT NOT NULL,
    date_time integer not NULL
);
