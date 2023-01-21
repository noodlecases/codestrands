CREATE TABLE users (
    id serial NOT NULL,
    first_name text NOT NULL,
    last_name text NOT NULL,
    email text UNIQUE NOT NULL,
    username text UNIQUE NOT NULL,
    bio text,
    image text,
    created_at timestamptz DEFAULT NOW() NOT NULL,
    updated_at timestamptz DEFAULT NOW() NOT NULL,

    PRIMARY KEY (id)
);

CREATE TABLE accounts (
    id serial NOT NULL,
    user_id integer NOT NULL,
    provider text NOT NULL,
    provider_account_id text NOT NULL,
    refresh_token text NOT NULL,
    access_token text NOT NULL,
    expires_at timestamptz NOT NULL,

    PRIMARY KEY (id),
    CONSTRAINT valid_provider CHECK (provider IN ('KEYCLOAK')),
    UNIQUE (user_id, provider),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
