-- Create Followers table
CREATE TABLE followers(
    follower TEXT NOT NULL,
    followed TEXT NOT NULL,
    CONSTRAINT fk_follower
        FOREIGN KEY(follower)
            REFERENCES users(username)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
    CONSTRAINT fk_followed
        FOREIGN KEY(followed)
            REFERENCES users(username)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
    CHECK (follower != followed),
    UNIQUE (follower, followed)
);
