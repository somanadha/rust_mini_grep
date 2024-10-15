use std::process;

use rust_mini_grep::*;
fn main() {
    let config = GrepConfig::build().unwrap_or_else(|err| {
        println!("Application error: {err}");
        process::exit(1);
    });

    let search_results = config.grep().unwrap_or_else(|err| {
        println!("Application error: {err}");
        process::exit(2);
    });

    search_results.print_grep_output(&config);
}
