#![allow(dead_code, unused_variables)]
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
// use std::env;
use std::error::Error;
use crate::parsing;

lazy_static! {
    static ref XML_FILES: HashMap<char, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert(
            'a',
            vec![
                "a01.xml", "a02.xml", "a03.xml", "a04.xml", "a05.xml", "a06.xml", "a07.xml",
                "a08.xml", "a09.xml", "a10.xml",
            ],
        );
        m.insert('b', vec!["b01.xml", "b02.xml"]);
        m.insert('c', vec!["c01.xml"]);
        m.insert('d', vec!["d01.xml", "d02.xml", "d03.xml", "d04.xml"]);
        m.insert(
            'e',
            vec![
                "e01.xml", "e02.xml", "e03.xml", "e04.xml", "e05.xml", "e06.xml", "e07.xml",
                "e08.xml", "e09.xml", "e10.xml",
            ],
        );
        m.insert('f', vec!["f01.xml", "f02.xml", "f03.xml"]);
        m.insert('g', vec!["g01.xml", "g02.xml"]);
        m.insert('h', vec!["h01.xml"]);
        m.insert('i', vec!["i01.xml", "i02.xml"]);
        m.insert(
            'k',
            vec![
                "k01.xml", "k02.xml", "k03.xml", "k04.xml", "k05.xml", "k06.xml",
            ],
        );
        m.insert('l', vec!["l01.xml", "l02.xml", "l03.xml"]);
        m.insert('m', vec!["m01.xml", "m02.xml", "m03.xml"]);
        m.insert('n', vec!["n01.xml", "n02.xml"]);
        m.insert('o', vec!["o01.xml", "o02.xml", "o03.xml", "o04.xml"]);
        m.insert(
            'p',
            vec![
                "p01.xml", "p02.xml", "p03.xml", "p04.xml", "p05.xml", "p06.xml", "p07.xml",
                "p08.xml",
            ],
        );
        m.insert('q', vec!["q01.xml", "q02.xml"]);
        m.insert('r', vec!["r01.xml", "r02.xml"]);
        m.insert(
            's',
            vec![
                "s01.xml", "s02.xml", "s03.xml", "s04.xml", "s05.xml", "s06.xml",
            ],
        );
        m.insert('t', vec!["t01.xml", "t02.xml", "t03.xml"]);
        m.insert('u', vec!["u01.xml", "u02.xml"]);
        m.insert('w', vec!["w01.xml", "w02.xml"]);
        m.insert('x', vec!["x01.xml", "x02.xml"]);
        m.insert('y', vec!["y01.xml"]);
        m.insert('z', vec!["z01.xml"]);
        m
    };
}

// pub fn lookup(mut term: &str) -> Result<Vec<String>, Box<dyn Error>> {
//     let mut initial: char = term.chars().next().unwrap();
//     if initial == '*' {
//         initial = term.chars().next().unwrap();
//         term = &term[1..];
//     }
//
//     // println!("Initial: {}\nTerm: {}", initial, term);
//     let files = XML_FILES.get(&initial).unwrap();
//     let xml_lines = search(&term.to_lowercase(), files)?;
//     // println!("xml_lines: {:#?}", xml_lines);
//     Ok(xml_lines)
// }

pub fn search(term: &str, file: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new("/home/simon/Projects/python/greeklexicon/data/LSJLogeion/");

    let mut found = String::new();
    let full_path = path.join(file);
    // println!("path: {:?}", full_path);
    let file_text = File::open(full_path)?;
    let buffered_text = BufReader::new(file_text);
    let ptn = format!(r#"key="{}""#, regex::escape(term));
    let rgx = Regex::new(&ptn)?;
    for line in buffered_text.lines() {
        let line = line?;
        if rgx.is_match(&line) {
            found = line;
            break;
        }
    }
    Ok(found)
}

pub fn query_lsj_vec(term_vec: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let mut lines_found = Vec::new();
    let mut entries_found = Vec::new();
    for mut term in term_vec {
        // let initial: char = term.chars().next().unwrap().to_lowercase().next().unwrap();
        let mut initial: char = term.chars().next().unwrap();
        if initial == '*' {
            initial = term.chars().next().unwrap();
            term = term[1..].to_string();
        }
        let files = XML_FILES.get(&initial).unwrap();
        for file in files {
            let query_result = search(&term, &file);
            match query_result {
                Ok(line) if &line != "" => {
                    lines_found.push(line);
                }
                Ok(_) => {}
                // Err(e) => eprintln!("{e}"),
                Err(_) => {}
            }
        }
    }

    for line in lines_found.iter() {
        let parsed_entry = parsing::parse_entry(line);
        match parsed_entry {
            Ok((_, entry)) => entries_found.push(entry),
            // Err(e) => eprintln!("{e}"),
            Err(e) => {}
        }
    }

    match serde_json::to_string(&entries_found) {
        Ok(json) => Ok(json),
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]
    // fn test_lookup() {
    //     let term = "teleuth/";
    //     let xml_results = lookup(term);
    //
    //     println!("XML results:\n{:#?}", xml_results);
    // }
}
