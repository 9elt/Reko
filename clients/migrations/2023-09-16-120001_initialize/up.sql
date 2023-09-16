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
    id                      INT                     NOT NULL PRIMARY KEY AUTO_INCREMENT,
    username                VARCHAR(64)             NOT NULL UNIQUE,
    hash                    BIGINT                  UNSIGNED NOT NULL,
    updated_at              TIMESTAMP               NOT NULL
);

CREATE TABLE entries (
    id                      INT                     NOT NULL PRIMARY KEY AUTO_INCREMENT,
    watched                 BOOLEAN                 NOT NULL,
    updated_at              TIMESTAMP               NOT NULL,

    anime                   INT                     NOT NULL,
    FOREIGN KEY (anime)     REFERENCES anime(id)    ON DELETE CASCADE,

    user                    INT                     NOT NULL,    
    FOREIGN KEY (user)      REFERENCES users(id)    ON DELETE CASCADE,

    UNIQUE unique_id (anime, user)
);
