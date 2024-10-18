use std::env;
use std::process;

use rust_mini_grep::GrepConfig;
use rust_mini_grep::SearchResults;

fn main() {
    let command_line_args: Vec<String> = env::args().collect();
    let config: GrepConfig = GrepConfig::new(&command_line_args[1..]).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });

    let search_results: SearchResults = SearchResults::grep(&config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(2);
    });

    search_results.print_grep_output(&config);
}
