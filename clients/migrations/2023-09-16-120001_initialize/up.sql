-- Your SQL goes here

CREATE TABLE anime (
    id                      INT                     NOT NULL PRIMARY KEY,
    title                   VARCHAR(256)            NOT NULL,
    airing_date             TIMESTAMP               NULL,
    length                  INT,
    mean                    FLOAT,
    rating                  VARCHAR(8),
    picture                 VARCHAR(64),
    aired                   BOOLEAN                 NOT NULL,
    stats                   JSON                    NOT NULL,
    updated_at              TIMESTAMP               NOT NULL,

    parent                  INT,
    FOREIGN KEY (parent)    REFERENCES anime(id)
);

CREATE TABLE users (
    id                      INT                     PRIMARY KEY AUTO_INCREMENT,
    username                VARCHAR(64)             NOT NULL UNIQUE,
    hash                    BIGINT                  UNSIGNED NOT NULL,
    updated_at              TIMESTAMP               NOT NULL
);

CREATE TABLE entries (
    id                      INT                     PRIMARY KEY AUTO_INCREMENT,
    score                   INT                     NOT NULL,
    watched                 BOOLEAN                 NOT NULL,
    updated_at              TIMESTAMP               NOT NULL,

    -- an anime may not exist at the time of entry creation
    -- so foreign key for the time being
    anime                   INT                     NOT NULL,

    -- anime                   INT,
    -- FOREIGN KEY (anime)     REFERENCES anime(id)    ON DELETE CASCADE,

    user                    INT                     NOT NULL,    
    FOREIGN KEY (user)      REFERENCES users(id)    ON DELETE CASCADE,

    UNIQUE unique_entry (anime, user)
);
