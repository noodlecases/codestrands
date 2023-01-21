CREATE TABLE user_interests (
    id serial NOT NULL,
    user_id integer NOT NULL,
    interest_id integer NOT NULL,
    created_at timestamptz DEFAULT NOW() NOT NULL,

    PRIMARY KEY (id),
    UNIQUE (user_id, interest_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (interest_id) REFERENCES interests(id) ON DELETE CASCADE
);
