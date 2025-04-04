-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(20) NOT NULL UNIQUE,
    email VARCHAR(50) NOT NULL UNIQUE,
    password TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT NOW()
);