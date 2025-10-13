CREATE TYPE action_permission as ENUM ('read', 'write', 'delete', 'create', 'all')

CREATE TABLE "user"(
    -- User
    id uuid DEFAULT uuidv7() PRIMARY KEY,
    username varchar(128) NOT NULL UNIQUE,
    email varchar(256) NOT NULL UNIQUE,

    -- Auth
    pwd varchar(256),

    -- Timestamps
    cid uuid NOT NULL,
    ctime timestamp with time zone NOT NULL,
    mid uuid NOT NULL,
    mtime timestamp with time zone NOT NULL
);

CREATE TABLE "permission" (
    id uuid uuidv7() PRIMARY KEY,

    -- Resource and coupled action e.g. products:read
    resource varchar(50) NOT NULL,
    "action" "action_permission" NOT NULL,

    UNIQUE(resource, "action")

);

CREATE TABLE "role" (
    id uuid DEFAULT uuidv7() PRIMARY KEY,
    organisation_id uuid REFERENCES "organisation"(id) ON DELETE cascade,
    name varchar(50) NOT NULL,

    UNIQUE(organisation_id, name)
);

CREATE TABLE "role_permission" (
    role_id uuid NOT NULL REFERENCES "role"(id) ON DELETE cascade,
    permission_id uuid NOT NULL REFERENCES "permission"(id) ON DELETE cascade,

    PRIMARY KEY(role_id, permission_id)
)

CREATE TABLE "organisation_membership" (
    user_id uuid NOT NULL REFERENCES "user"(id) ON DELETE cascade,
    role_id uuid NOT NULL REFERENCES "role"(id) ON DELETE cascade,

    PRIMARY KEY(user_id, role_id)
)

CREATE TABLE "organisation" (
    id uuid DEFAULT uuidv7() PRIMARY KEY,
    name varchar(128) NOT NULL,

    -- Timestamps
    cid uuid NOT NULL,
    ctime timestamp with time zone NOT NULL,
    mid uuid NOT NULL,
    mtime timestamp with time zone NOT NULL
);


CREATE TABLE "product"(
    id uuid DEFAULT uuidv7() PRIMARY KEY,
    organisation_id REFERENCES "organisation"(id) ON DELETE cascade
);

CREATE TABLE "oauth_client" (
    id uuid DEFAULT uuidv7() PRIMARY KEY,
    name TEXT NOT NULL,
    secret TEXT,
    organisation_id REFERENCES "organisation"(id) ON DELETE cascade,

    -- Timestamps
    cid uuid NOT NULL,
    ctime timestamp with time zone NOT NULL,
    mid uuid NOT NULL,
    mtime timestamp with time zone NOT NULL
);
