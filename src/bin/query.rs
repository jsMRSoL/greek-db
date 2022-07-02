use clap::Parser;
use greek_db::get_lsj_key;
use greek_db::query_asvocab;
use greek_db::query_gcse_greek;
use greek_db::transliterate::transliterate;
use greek_db::QueryFunc;
use std::sync::Arc;
use std::thread;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    term: String,
}


fn main() {
    let args = Args::parse();
    let term = transliterate(&args.term);
    run_query(Arc::new(term));
}

fn run_query(term: Arc<String>) {
    let pool = greek_db::get_connection_pool();
    let mut threads = vec![];
    let queries: [QueryFunc; 3] = [query_gcse_greek, query_asvocab, get_lsj_key];

    for f in queries {
        let pool1 = pool.clone();
        let term1 = term.clone();
        threads.push(thread::spawn({
            move || {
                let conn = &mut pool1.get().expect("Could not get connection from pool");
                f(&term1, conn).expect("Database did not return result.")
            }
        }))
    }

    let mut results: Vec<String> = Vec::new();
    for handle in threads {
        let res = handle.join().unwrap();
        results.push(res);
    }

    let res_str = results.join(", ");
    println!("{{{}}}", res_str);

}
