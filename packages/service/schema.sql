-- Schema for rmcs-actions SQLite database.
-- Used to initialize an empty database so that sqlx compile-time query
-- checking can validate SQL expressions.

CREATE TABLE IF NOT EXISTS robots (
    uuid TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    mac  TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS network_info (
    robot_uuid   TEXT PRIMARY KEY NOT NULL,
    info         TEXT NOT NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
