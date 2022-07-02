use diesel::insert_into;
use diesel::prelude::*;
use diesel::Insertable;
use greek_db::schema::*;
use greek_db::establish_connection;
use greek_db::transliterate;
use std::fs::File;
use std::io::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = get_values_from_file("./Greek-Summer-2021-word-list.csv")?;
    let mut connection = establish_connection();
    let res = insert_vec(data, &mut connection);
    println!("{:#?}", res);
    Ok(())
}

#[derive(Insertable)]
#[table_name = "asvocab"]
struct NewEntry {
    greek: String,
    headword: String,
    dict_form: String,
    part_of_speech: String,
    meaning: String,
}

fn insert_vec(values_vec: Vec<NewEntry>, conn: &mut PgConnection) -> QueryResult<usize> {
    use greek_db::schema::asvocab::dsl::*;
    insert_into(asvocab).values(&values_vec).execute(conn)
}

fn get_values_from_file(path: &str) -> Result<Vec<NewEntry>, Error> {
    let file = File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut data_vec: Vec<NewEntry> = Vec::new();
    for result in reader.records() {
        let record = result?;
        let greek = &record[0];
        let headword = transliterate::transliterate(&greek);
        let dict_form = &record[1];
        let part_of_speech = &record[2];
        let meaning = &record[3];
        data_vec.push(NewEntry {
            greek: greek.to_string(),
            headword,
            dict_form: dict_form.to_string(),
            part_of_speech: part_of_speech.to_string(),
            meaning: meaning.to_string(),
        });
    }
    Ok(data_vec)
}
