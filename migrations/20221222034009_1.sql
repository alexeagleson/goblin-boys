-- Add migration script here
CREATE TABLE moves (
    id INTEGER PRIMARY KEY NOT NULL,
    direction TEXT CHECK(direction IN ('up', 'down', 'left', 'right')) NOT NULL
);