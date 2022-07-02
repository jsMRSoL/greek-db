#![allow(dead_code)]
use diesel::insert_into;
use diesel::prelude::*;
use diesel::Insertable;
use greek_db::schema::*;
use greek_db::*;
use transliterate::transliterate;
use std::fs::File;
use std::io::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = get_values_from_file("./greek_for_db.csv")?;
    // let _ = get_values_from_file("./greek_for_db.csv")?;
    let mut connection = establish_connection();
    let res = insert_vec(data, &mut connection);
    println!("{:#?}", res);
    Ok(())
}

#[derive(Insertable, Debug)]
#[table_name = "gcse_greek"]
struct NewEntry {
    greek: String,
    headword: String,
    dict_form: String,
    part_of_speech: String,
    meaning: String,
}

fn insert_vec(values_vec: Vec<NewEntry>, conn: &mut PgConnection) -> QueryResult<usize> {
    use greek_db::schema::gcse_greek::dsl::*;
    insert_into(gcse_greek).values(&values_vec).execute(conn)
}

fn get_values_from_file(path: &str) -> Result<Vec<NewEntry>, Error> {
    let file = File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut data_vec: Vec<NewEntry> = Vec::new();
    for result in reader.records() {
        let record = result?;
        let greek = &record[0];
        let headword = transliterate(&greek);
        let dict_form = &record[1];
        let part_of_speech = &record[2];
        let meaning = &record[3];
        data_vec.push(NewEntry {
            greek: greek.to_string(),
            headword: headword.to_string(),
            dict_form: dict_form.to_string(),
            part_of_speech: part_of_speech.to_string(),
            meaning: meaning.to_string(),
        });
    }
    Ok(data_vec)
}
