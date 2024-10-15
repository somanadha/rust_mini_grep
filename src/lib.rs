use std::env;
use std::error::Error;
use std::fs;
use std::collections::HashMap;
pub struct GrepConfig {
    _search_string : String,
    search_file_name: String,
}
pub struct SearchResults {
    _results: HashMap<u32, Vec<u32>>,
}

impl GrepConfig {
    pub fn build() ->Result<GrepConfig, &'static str> {

        let command_line_args: Vec<String> = env::args().collect();

        if command_line_args.len() < 3 {
            Err("Too few argments - Syntax: rust_mini_grep <SEARCH_STRING> <SEARCH_FILE_NAME>")
        } 
        else {
            let config = GrepConfig {
                _search_string : command_line_args[1].clone(),
                search_file_name: command_line_args[2].clone(),
            };

            Ok(config)
        }
    }

    pub fn run(&self) -> Result<SearchResults, Box<dyn Error>> {

        let file_data = fs::read_to_string(self.search_file_name.as_str())?;

        println!("{file_data}");

        let results_map:HashMap<u32, Vec<u32>> = HashMap::new();

        let search_results: SearchResults = SearchResults{
            _results: results_map,
        };

        Ok(search_results)
    }
}

