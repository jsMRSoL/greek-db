#![allow(dead_code)]
#[macro_use]
extern crate diesel;
// use serde::Serialize;
// use serde_json;

// pub mod models;
pub mod schema;
pub mod transliterate;
pub mod search;
pub mod parsing;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::prelude::*;

use dotenv;
use serde::Serialize;
use serde_json;
use std::env;

pub type QueryFunc = fn(&str, &PgConnection) -> Result<String, Box<dyn std::error::Error>>;

pub fn establish_connection() -> PgConnection {
    let database_url =
        env::var("LSJ_PG_DATABASE_URL_LOCAL").expect("LSJ_PG_DATABASE_URL_LOCAL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to datbase.")
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv::from_path("/home/simon/.api_keys").expect("api_keys not accessible");
    let url = dotenv::var("LSJ_PG_DATABASE_URL_LOCAL").expect("LSJ_PG_DATABASE_URL_LOCAL must be set");

    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .max_size(25)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

#[derive(Queryable, Debug, PartialEq, Clone, Serialize)]
struct QueryResult {
    dict_form: String,
    part_of_speech: String,
    meaning: String,
}

pub fn query_gcse_greek(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::gcse_greek::dsl::dict_form as g_dict_form;
    use self::schema::gcse_greek::dsl::gcse_greek;
    use self::schema::gcse_greek::dsl::headword as g_headword;
    use self::schema::gcse_greek::dsl::meaning as g_meaning;
    use self::schema::gcse_greek::dsl::part_of_speech as g_part_of_speech;
    use self::schema::lsj_lemmata::dsl::*;

    let data: Result<Vec<(String, String, String)>, _> = gcse_greek
        .inner_join(lsj_lemmata.on(g_headword.eq(headword)))
        .filter(form.eq(term))
        .select((g_dict_form, g_part_of_speech, g_meaning))
        .order(g_dict_form.asc())
        .load(connection);

    match data {
        Ok(results) => match serde_json::to_string(&results) {
            Ok(json) => Ok(format!("\"gcse\": {}", json)),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn query_asvocab(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::asvocab::dsl::asvocab;
    use self::schema::asvocab::dsl::dict_form as a_dict_form;
    use self::schema::asvocab::dsl::headword as a_headword;
    use self::schema::asvocab::dsl::meaning as a_meaning;
    use self::schema::asvocab::dsl::part_of_speech as a_part_of_speech;
    use self::schema::lsj_lemmata::dsl::*;

    let data: Result<Vec<(String, String, String)>, _> = asvocab
        .inner_join(lsj_lemmata.on(a_headword.eq(headword)))
        .filter(form.eq(term))
        .select((a_dict_form, a_part_of_speech, a_meaning))
        .order(a_dict_form.asc())
        .load(connection);

    match data {
        Ok(results) => match serde_json::to_string(&results) {
            Ok(json) => Ok(format!("\"asvocab\": {}", json)),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn get_lsj_key(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::lsj_lemmata::dsl::*;
    use self::schema::lsj_entry_keys::dsl::*;

    let data: Result<Vec<String>, _> = lsj_entry_keys
        .inner_join(lsj_lemmata.on(headword.eq(simple_key)))
        .filter(form.eq(term))
        .select(key)
        .load(connection);

    match data {
        Ok(results) => {
            match search::query_lsj_vec(results) {
                Ok(parsed_entries) => Ok(format!("\"lsj\": {}", parsed_entries)),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn query_gcse_greek_headword(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::gcse_greek::dsl::dict_form as g_dict_form;
    use self::schema::gcse_greek::dsl::gcse_greek;
    use self::schema::gcse_greek::dsl::headword as g_headword;
    use self::schema::gcse_greek::dsl::meaning as g_meaning;
    use self::schema::gcse_greek::dsl::part_of_speech as g_part_of_speech;

    let data: Result<Vec<(String, String, String)>, _> = gcse_greek
        .filter(g_headword.eq(term))
        .select((g_dict_form, g_part_of_speech, g_meaning))
        .order(g_dict_form.asc())
        .load(connection);

    match data {
        Ok(results) => match serde_json::to_string(&results) {
            Ok(json) => Ok(format!("\"gcse\": {}", json)),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn query_asvocab_headword(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::asvocab::dsl::asvocab;
    use self::schema::asvocab::dsl::dict_form as a_dict_form;
    use self::schema::asvocab::dsl::headword as a_headword;
    use self::schema::asvocab::dsl::meaning as a_meaning;
    use self::schema::asvocab::dsl::part_of_speech as a_part_of_speech;

    let data: Result<Vec<(String, String, String)>, _> = asvocab
        .filter(a_headword.eq(term))
        .select((a_dict_form, a_part_of_speech, a_meaning))
        .order(a_dict_form.asc())
        .load(connection);

    match data {
        Ok(results) => match serde_json::to_string(&results) {
            Ok(json) => Ok(format!("\"asvocab\": {}", json)),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn get_lsj_key_headword(
    term: &str,
    connection: &PgConnection,
) -> Result<String, Box<dyn std::error::Error>> {
    use self::schema::lsj_entry_keys::dsl::*;

    let data: Result<Vec<String>, _> = lsj_entry_keys
        .filter(head.eq(term))
        .select(key)
        .load(connection);

    match data {
        Ok(results) => {
            match search::query_lsj_vec(results) {
                Ok(parsed_entries) => Ok(format!("\"lns\": {}", parsed_entries)),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let _ = establish_connection();
    }
}
