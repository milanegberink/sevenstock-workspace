CREATE TABLE IF NOT EXISTS users(
    id UUID DEFAULT uuidv7() PRIMARY KEY,
    username varchar(128) NOT NULL UNIQUE,

    -- Auth
    pwd varchar(256),

    -- Timestamps
    cid bigint NOT NULL,
    ctime timestamp with time zone NOT NULL,
    mid bigint NOT NULL,
    mtime timestamp with time zone NOT NULL
)

CREATE TABLE IF NOT EXISTS api_keys(
    id UUID DEFAULT uuidv7() PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    key VARCHAR(256) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
