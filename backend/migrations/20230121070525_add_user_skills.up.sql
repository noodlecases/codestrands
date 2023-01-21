CREATE TABLE user_skills (
    id serial NOT NULL,
    user_id integer NOT NULL,
    skill_id integer NOT NULL,
    created_at timestamptz DEFAULT NOW() NOT NULL,

    PRIMARY KEY (id),
    UNIQUE (user_id, skill_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (skill_id) REFERENCES skills(id) ON DELETE CASCADE
);
