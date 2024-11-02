-- Add migration script here
DROP TABLE IF EXISTS rng;

CREATE TABLE rng (
    record_id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    roll_result INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);