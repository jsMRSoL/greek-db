#[allow(dead_code)]
use diesel::insert_into;
use diesel::prelude::*;
use diesel::Insertable;
use greek_db::establish_connection;
use greek_db::schema::*;
// use greek_db::transliterate; use regex::Regex;
use std::sync::Arc;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::thread;

#[derive(Insertable, Debug)]
#[table_name = "lsj_entry_keys"]
struct NewEntryKey {
    xml_id: String,
    key: String,
    simple_key: String,
    head: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re: Arc<Regex> = Arc::new(Regex::new(r#"<div2 id="(.+?)".+?key="([^0-9]+?)(\d*)".+>(.+?)</head"#).unwrap());
    let path = Path::new("/home/simon/Projects/python/greeklexicon/data/LSJLogeion/");

    let files = [ 
        "a01.xml", "a02.xml", "a03.xml", "a04.xml", "a05.xml", "a06.xml", "a07.xml", "a08.xml",
        "a09.xml", "a10.xml", "b01.xml", "b02.xml", "c01.xml", "d01.xml", "d02.xml", "d03.xml",
        "d04.xml", "e01.xml", "e02.xml", "e03.xml", "e04.xml", "e05.xml", "e06.xml", "e07.xml",
        "e08.xml", "e09.xml", "e10.xml", "f01.xml", "f02.xml", "f03.xml", "g01.xml", "g02.xml",
        "h01.xml", "i01.xml", "i02.xml", "k01.xml", "k02.xml", "k03.xml", "k04.xml", "k05.xml",
        "k06.xml", "l01.xml", "l02.xml", "l03.xml", "m01.xml", "m02.xml", "m03.xml", "n01.xml",
        "n02.xml", "o01.xml", "o02.xml", "o03.xml", "o04.xml", "p01.xml", "p02.xml", "p03.xml",
        "p04.xml", "p05.xml", "p06.xml", "p07.xml", "p08.xml", "q01.xml", "q02.xml", "r01.xml",
        "r02.xml", "s01.xml", "s02.xml", "s03.xml", "s04.xml", "s05.xml", "s06.xml", "san.xml",
        "t01.xml", "t02.xml", "t03.xml", "u01.xml", "u02.xml", "w01.xml", "w02.xml", "x01.xml",
        "x02.xml", "y01.xml", "z01.xml",
    ];

    let mut handles = Vec::with_capacity(files.len());

    for file in files {
        let mut connection = establish_connection();
        let re_ = re.clone();
        let t = thread::spawn(move || {
            let mut found: Vec<NewEntryKey> = Vec::new();
            let full_path = path.join(file);
            println!("path: {:?}", full_path);
            let file_text = File::open(full_path).unwrap();
            let buffered_text = BufReader::new(file_text);
            for line in buffered_text.lines() {
                let line = line.expect("No line in buffer.");
                create_objs(&re_, &line, &mut found);
            }
            let res = insert_vec_lsj(found, &mut connection);
            println!("{:?}", res);
        });
        handles.push(t);
    }

    for h in handles {
        let _ = h.join();
    }

    Ok(())
}

fn insert_vec_lsj(values_vec: Vec<NewEntryKey>, conn: &mut PgConnection) -> QueryResult<usize> {
    use greek_db::schema::lsj_entry_keys::dsl::*;
    insert_into(lsj_entry_keys)
        .values(&values_vec)
        .execute(conn)
}

fn create_objs(re: &Regex, line: &str, found: &mut Vec<NewEntryKey>) {
    let captures = re.captures(line);
    captures.map(|c| {
        let xml_id = c.get(1).map_or(String::new(), |s| s.as_str().to_owned());
        let simple_key = c.get(2).map_or(String::new(), |s| s.as_str().to_owned());
        let key_suffix = c.get(3);
        // while simple_key still contains long and short marks...
        let key = match key_suffix {
            Some(sfx) => format!("{}{}", &simple_key, sfx.as_str()),
            None => format!("{}", &simple_key),
        };
        // now we've constructed 'key', let's remove the long and shorts from simple key
        // so that we can search on this field
        let simple_key = simple_key.replace("^", "").replace("_", "").to_owned();
        // let simple_key = key.replace("^", "").replace("_", "").to_owned();
        let head = c.get(4).map_or(String::new(), |s| s.as_str().to_owned());
        // let head = transliterate::transliterate(&gk_head);
        found.push(NewEntryKey {
            xml_id,
            key,
            simple_key,
            head,
        });
    });
}
