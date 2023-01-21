CREATE TABLE chat_users (
    id serial NOT NULL,
    chat_id integer NOT NULL,
    user_id integer NOT NULL,
    created_at timestamptz DEFAULT NOW() NOT NULL,

    PRIMARY KEY (id),
    UNIQUE (chat_id, user_id),
    FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
