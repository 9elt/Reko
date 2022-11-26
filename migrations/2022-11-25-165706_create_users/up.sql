-- Your SQL goes here
CREATE TABLE users (
    user_name       VARCHAR    NOT NULL PRIMARY KEY,
    list            JSONB      NOT NULL,
    model           JSONB      NULL,
    updated_at      TIMESTAMP  NOT NULL DEFAULT NOW()
);