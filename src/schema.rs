table! {
    asvocab (id) {
        id -> Int8,
        greek -> Varchar,
        headword -> Varchar,
        dict_form -> Varchar,
        part_of_speech -> Varchar,
        meaning -> Varchar,
    }
}

table! {
    gcse_greek (id) {
        id -> Int8,
        greek -> Varchar,
        headword -> Varchar,
        dict_form -> Varchar,
        part_of_speech -> Varchar,
        meaning -> Varchar,
    }
}

table! {
    lsj_entry_keys (id) {
        id -> Int8,
        xml_id -> Varchar,
        key -> Varchar,
        simple_key -> Varchar,
        head -> Varchar,
    }
}

table! {
    lsj_lemmata (id) {
        id -> Int8,
        headword -> Varchar,
        suffixed -> Varchar,
        form -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    asvocab,
    gcse_greek,
    lsj_entry_keys,
    lsj_lemmata,
);
