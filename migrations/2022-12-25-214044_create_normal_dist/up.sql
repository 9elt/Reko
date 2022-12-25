-- Your SQL goes here
CREATE TABLE analysis (
    users_count     SERIAL      NOT NULL PRIMARY KEY,
    mean            JSONB       NOT NULL,
    std_dev         JSONB       NOT NULL
);