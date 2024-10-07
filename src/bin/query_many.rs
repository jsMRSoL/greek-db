use clap::Parser;
use clap::ArgGroup;
use greek_db::query_asvocab;
use greek_db::query_gcse_greek;
use greek_db::QueryFunc;
use greek_db::transliterate::transliterate;
use std::thread;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
            ArgGroup::new("level")
                .required(true)
                .args(&["gcse", "asvocab"])))]
struct Args {
    #[clap(required = true)]
    terms: Vec<String>,
    #[clap(long)]
    gcse: bool,
    #[clap(long)]
    asvocab: bool,
}

fn main() {
    let args = Args::parse();
    // println!{"{:#?}", args};
    let f: QueryFunc = match args.gcse {
        true => query_gcse_greek,
        false => query_asvocab,
    };
    run_query(args.terms, f);
}

fn run_query(terms: Vec<String>, f: QueryFunc) {
    let pool = greek_db::get_connection_pool();
    let mut threads = vec![];

    for term in terms {
        let pool1 = pool.clone();
        let term1 = transliterate(&term);
        threads.push(thread::spawn({
            move || {
                let conn = &mut pool1.get().expect("Could not get connection from pool");
                let res = f(&term1, conn).expect("Database did not return result.");
                (term, res)
            }
        }))
    }

    let mut results: Vec<String> = Vec::new();
    for handle in threads {
        let (term, entry) = handle.join().unwrap();
        let (_, tail) = entry.split_once(": ").expect("Data returned from query was malformed");
        let res = format!("\"{term}\": {tail}");
        results.push(res);
    }

    let res_str = results.join(", ");
    println!("{{{}}}", res_str);

}
