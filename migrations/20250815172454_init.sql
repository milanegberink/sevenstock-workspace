CREATE TABLE IF NOT EXISTS user(
    -- User
    id UUID DEFAULT uuidv7() PRIMARY KEY,
    username varchar(128) NOT NULL UNIQUE,
    email varchar(256) NOT NULL UNIQUE,

    -- Auth
    pwd varchar(256),

    -- Timestamps
    cid UUID NOT NULL,
    ctime timestamp with time zone NOT NULL,
    mid UUID NOT NULL,
    mtime timestamp with time zone NOT NULL
);


CREATE TABLE IF NOT EXISTS product(
    id UUID DEFAULT uuidv7() PRIMARY KEY,

)
