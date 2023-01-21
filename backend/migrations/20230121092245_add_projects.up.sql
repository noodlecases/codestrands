CREATE TABLE projects (
    id serial NOT NULL,
    user_id integer NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    url text NOT NULL,
    image text,
    created_at timestamptz DEFAULT NOW() NOT NULL,
    updated_at timestamptz DEFAULT NOW() NOT NULL,

    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
