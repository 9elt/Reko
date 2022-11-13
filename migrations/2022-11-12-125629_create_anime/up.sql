-- Your SQL goes here
CREATE TABLE anime (
    id               SERIAL     NOT NULL PRIMARY KEY,
    title            VARCHAR    NOT NULL,
    picture          VARCHAR    NULL,
    mean             SMALLINT   NULL,
    airing_date      DATE       NULL,
    airing_status    SMALLINT   NULL,
    num_episodes     SMALLINT   NULL,
    rating           SMALLINT   NULL,
    genres           SMALLINT[] NULL,
    related          JSONB      NULL
);