use clap::Parser;
use greek_db::get_lsj_key_headword;
use greek_db::query_asvocab_headword;
use greek_db::query_gcse_greek_headword;
use greek_db::transliterate::transliterate;
// use std::env;
use std::sync::Arc;
use std::thread;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    term: String,
    #[clap(short, long, action)]
    greek: bool,
}

fn main() {
    let args = Args::parse();
    let term = match args.greek {
        true => transliterate(&args.term),
        false => args.term,
    };
    run_query(Arc::new(term));
}

fn run_query(term: Arc<String>) {
    let pool = greek_db::get_connection_pool();
    let mut threads = vec![];

    let pool0 = pool.clone();
    let term0 = term.clone();
    threads.push(thread::spawn({
        move || {
            let conn = &mut pool0.get().unwrap();
            get_lsj_key_headword(&term0, conn).unwrap()
        }
    }));

    let pool1 = pool.clone();
    let term1 = term.clone();
    threads.push(thread::spawn({
        move || {
            let conn = &mut pool1.get().unwrap();
            query_gcse_greek_headword(&term1, conn).unwrap()
        }
    }));

    let pool2 = pool.clone();
    let term2 = term.clone();
    threads.push(thread::spawn({
        move || {
            let conn = &mut pool2.get().unwrap();
            query_asvocab_headword(&term2, conn).unwrap()
        }
    }));

    let mut results: Vec<String> = Vec::new();
    for handle in threads {
        let res = handle.join().unwrap();
        results.push(res);
    }

    let res_str = results.join(", ");
    println!("{{{}}}", res_str);
}
