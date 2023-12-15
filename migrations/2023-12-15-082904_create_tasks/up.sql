CREATE TABLE IF NOT EXISTS geo_provider
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    api_key TEXT NOT NULL,
    counter_limit integer not NULL,
    counter integer not NULL
);
