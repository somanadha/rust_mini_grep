use std::process;

use rust_mini_grep::*;
fn main() {
    let config = GrepConfig::build().unwrap_or_else(|err| {
        println!("Application error: {err}");
        process::exit(1);
    });

    let _search_results = config.run().unwrap_or_else(|err|{
        println!("Application error: {err}");
        process::exit(2);
    });
}


