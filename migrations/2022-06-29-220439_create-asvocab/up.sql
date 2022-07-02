-- Your SQL goes here
CREATE TABLE asvocab (
    id BIGSERIAL PRIMARY KEY,
    greek VARCHAR NOT NULL,
    headword VARCHAR NOT NULL,
    dict_form VARCHAR NOT NULL,
    part_of_speech VARCHAR NOT NULL,
    meaning VARCHAR NOT NULL
);
