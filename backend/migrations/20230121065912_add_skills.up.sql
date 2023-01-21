CREATE TABLE skills (
    id serial NOT NULL,
    name text UNIQUE NOT NULL,
    created_at timestamptz DEFAULT NOW() NOT NULL,

    PRIMARY KEY (id)
);
