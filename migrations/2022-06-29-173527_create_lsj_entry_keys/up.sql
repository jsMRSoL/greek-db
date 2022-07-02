-- Your SQL goes here
CREATE TABLE lsj_entry_keys (
    id BIGSERIAL PRIMARY KEY,
    xml_id VARCHAR NOT NULL,
    key VARCHAR NOT NULL,
    simple_key VARCHAR NOT NULL,
    head VARCHAR NOT NULL
);
