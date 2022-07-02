-- Your SQL goes here
CREATE TABLE lsj_lemmata (
    id BIGSERIAL PRIMARY KEY,
    headword VARCHAR NOT NULL,
    suffixed VARCHAR NOT NULL,
    form VARCHAR NOT NULL
);
