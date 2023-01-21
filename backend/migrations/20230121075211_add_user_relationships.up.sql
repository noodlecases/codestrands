CREATE TABLE user_relationships (
    id serial NOT NULL,
    user1_id integer NOT NULL,
    user2_id integer NOT NULL,
    type integer NOT NULL,
    created_at timestamptz DEFAULT NOW() NOT NULL,
    updated_at timestamptz DEFAULT NOW() NOT NULL,

    PRIMARY KEY (id),
    UNIQUE (user1_id, user2_id),
    FOREIGN KEY (user1_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (user2_id) REFERENCES users(id) ON DELETE CASCADE
);
