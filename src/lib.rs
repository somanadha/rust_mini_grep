use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
pub struct GrepConfig {
    pub search_string: String,
    pub search_file_name: String,
}
pub struct SearchResults {
    results: HashMap<i32, Vec<usize>>,
}

impl GrepConfig {
    pub fn build() -> Result<GrepConfig, &'static str> {
        let command_line_args: Vec<String> = env::args().collect();

        if command_line_args.len() < 3 {
            Err("Too few argments - Syntax: rust_mini_grep <SEARCH_STRING> <SEARCH_FILE_NAME>")
        } else {
            let config = GrepConfig {
                search_string: command_line_args[1].clone(),
                search_file_name: command_line_args[2].clone(),
            };

            Ok(config)
        }
    }

    pub fn grep(&self) -> Result<SearchResults, Box<dyn Error>> {
        let file_data = fs::read_to_string(self.search_file_name.as_str())?;

        println!("{file_data}");

        let mut results_map: HashMap<i32, Vec<usize>> = HashMap::new();

        for one_line in file_data.lines().zip(1..) {
            let mut indexes_vector = Vec::new();
            let mut currunet_str: &str = one_line.0;
            let mut current_index: usize = 0;
            loop {
                currunet_str = &currunet_str[current_index..];
                match currunet_str.find(one_line.0) {
                    Some(index) => {
                        indexes_vector.push(index);
                        current_index = index + 1;
                    }
                    None => break,
                };
            }
            results_map.insert(one_line.1, indexes_vector);
        }

        let search_results: SearchResults = SearchResults {
            results: results_map,
        };

        Ok(search_results)
    }
}

impl SearchResults {
    pub fn print_grep_output(&self, config: &GrepConfig) {
        for each_line in &self.results {
            println!(
                "Line {}: has \"{}\" string in the file <{}> at following indexes: {}",
                each_line.0,
                config.search_string,
                config.search_file_name,
                self.concatenate_indexes(each_line.1)
            );
        }
    }

    fn concatenate_indexes(&self, indexes_vector: &Vec<usize>) -> String {
        let mut concatenated_indexes_string: String = String::new();
        for each_index in indexes_vector {
            concatenated_indexes_string.push_str(&each_index.to_string());
        }
        concatenated_indexes_string
    }
}
