CREATE TABLE interests (
    id serial NOT NULL,
    name text NOT NULL,
    created_at timestamptz DEFAULT NOW() NOT NULL,

    PRIMARY KEY (id)
);
