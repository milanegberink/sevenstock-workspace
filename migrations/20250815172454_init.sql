-- Add migration script here
CREATE TABLE IF NOT EXISTS users(
    id UUID DEFAULT uuidv7() PRIMARY KEY,
    username VARCHAR(128) NOT NULL UNIQUE
)
