use diesel::insert_into;
use diesel::prelude::*;
use diesel::Insertable;
use greek_db::schema::lsj_lemmata;
use greek_db::establish_connection;
use std::io::{BufReader, BufRead};
use std::fs::File;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lemma_objs = get_lemma_objects()?;
    let mut connection = establish_connection();
    let chunks = lemma_objs.chunks(10000);
    for chunk in chunks {
        let res = insert_vec_lns_lem(chunk, &mut connection);
        println!("{:?}", res);
    }
    Ok(())
}

#[derive(Debug, Insertable)]
#[table_name = "lsj_lemmata"]
struct Lemma {
    headword: String,
    suffixed: String,
    form: String,
}

fn insert_vec_lns_lem(values_vec: &[Lemma], conn: &mut PgConnection) -> QueryResult<usize> {
    use greek_db::schema::lsj_lemmata::dsl::*;
    insert_into(lsj_lemmata)
        .values(values_vec)
        .execute(conn)
}

fn get_lemma_objects() -> Result<Vec<Lemma>, Box<dyn std::error::Error>> {
    let file = File::open("./greek-lemmata.txt")?; 
    let reader = BufReader::new(file);
    println!("Got to here 1");
    let id_re = Regex::new(r"([a-z()\+/=\\|-]+?)(%*\d*)\s+(\d+)(.*)")?;
    println!("Got to here 2");
    // let lemmata_re = Regex::new(r"([a-z()/=\\]+?)\s+\((.+?)\)\s*")?;
    let lemmata_re = Regex::new(r"\t([a-z/=\\'()|)]+)[^\t(]+")?;
// ([a-z()\/=\\]+) matches 	a(/d' 
    // (\(.*?\){1,2})+ matches (aor ind act 3rd sg (epic doric ionic aeolic)) (aor imperat act 2nd sg)
    let mut lemmata_vec: Vec<Lemma> = Vec::new();
    for line in reader.lines() {
        let line = &line?;
        let captures = id_re.captures(line).unwrap();
        let headword = captures.get(1).map_or("", |s| s.as_str());
        let suffix = captures.get(2);
        let suffixed = match suffix {
            Some(sfx) => match sfx.as_str() {
                "" => format!("{}", &headword),
                _ => format!("{}{}", &headword, sfx.as_str()),
            },
            None => format!("{}", &headword),
        };
        let rest = captures.get(4).map_or("", |s| s.as_str());
        for cap in lemmata_re.captures_iter(rest) {
            let lemma = Lemma {
                headword: headword.to_owned(),
                suffixed: suffixed.clone(),
                form: cap[1].to_owned(),
            };
            // println!("{:#?}", lemma);
            lemmata_vec.push(lemma);
        } 

    }
    Ok(lemmata_vec)
}
