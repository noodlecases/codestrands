CREATE TABLE chats (
    id serial NOT NULL,
    title text,
    created_at timestamptz DEFAULT NOW() NOT NULL,
    updated_at timestamptz DEFAULT NOW() NOT NULL,

    PRIMARY KEY (id)
);
