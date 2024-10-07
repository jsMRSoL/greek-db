use diesel::insert_into;
use diesel::prelude::*;
use diesel::Insertable;
use greek_db::establish_connection;
use greek_db::schema::lsj_lemmata;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lemma_objs = get_lemma_objects2()?;
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
    insert_into(lsj_lemmata).values(values_vec).execute(conn)
}

fn get_lemma_objects() -> Result<Vec<Lemma>, Box<dyn std::error::Error>> {
    // let file = File::open("./greek-lemmata.txt")?;
    let file = File::open("./data/capitals3.txt")?;
    let reader = BufReader::new(file);
    println!("Got to here 1");
    let id_re = Regex::new(r"([a-z()\+/=\\\*|-]+?)(%*\d*)\s+(\d+)(.*)")?;
    println!("Got to here 2");
    // let lemmata_re = Regex::new(r"([a-z()/=\\]+?)\s+\((.+?)\)\s*")?;
    let lemmata_re = Regex::new(r"\t([a-z/=\\'()|)]+)[^\t(]+")?;
    // ([a-z()\/=\\]+) matches 	a(/d'
    // (\(.*?\){1,2})+ matches (aor ind act 3rd sg (epic doric ionic aeolic)) (aor imperat act 2nd sg)
    let mut lemmata_vec: Vec<Lemma> = Vec::new();
    for line in reader.lines() {
        let line = &line?;
        // println!("{}", line);
        // let captures = id_re.captures(line).unwrap();
        let captures = id_re.captures(line);
        if let None = captures {
            continue;
        }
        let captures = captures.unwrap();
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

fn get_lemma_objects2() -> Result<Vec<Lemma>, Box<dyn std::error::Error>> {
    let file = File::open("./data/capitals2.txt")?;
    let reader = BufReader::new(file);
    let id_re = Regex::new(r"([a-z!()\+/=\\\*\|'-]+?)\s+\{[\s\d]+([a-z\*\(\)_/,\|=]+).*\}")?;
    let mut failures: Vec<String> = vec![];
    let mut lemmata_vec: Vec<Lemma> = Vec::new();
    for line in reader.lines() {
        let line = &line?;
        let captures = id_re.captures(line);
        if let None = captures {
            failures.push(line.to_owned());
            continue;
        }
        let captures = captures.unwrap();
        let form = captures.get(1).map_or("", |s| s.as_str());
        let mut headword = captures.get(2).map_or("", |s| s.as_str());
        let mut parts = headword.split(",").skip(1);
        if let Some(p)= parts.next() {
            headword = p;
        }
        let lemma = Lemma {
            headword: headword.to_owned(),
            suffixed: headword.to_owned(),
            form: form.to_owned(),
        };
        lemmata_vec.push(lemma);
    }
    std::fs::write("data/failures.txt", failures.join("\n"))?;
    Ok(lemmata_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lemma_objects2() {
        let res = get_lemma_objects2();
        match res {
            Ok(v) => println!("{:#?}", v),
            Err(e) => eprintln!("{}", e),
        }
    }
    
    #[test]
    fn count_get_lemma_objects2() {
        let res = get_lemma_objects2();
        match res {
            Ok(v) => println!("{:#?}", v.len()),
            Err(e) => eprintln!("{}", e),
        }
    }
    
}
