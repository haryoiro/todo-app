-- Your SQL goes here
CREATE TABLE todos (
    id serial PRIMARY KEY,
    title varchar(255) NOT NULL,
    completed boolean NOT NULL DEFAULT false,
);