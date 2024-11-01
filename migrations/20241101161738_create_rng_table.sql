-- Add migration script here
CREATE TABLE rng (
    id INTEGER PRIMARY KEY,
    roll_result INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);