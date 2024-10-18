use indexmap::IndexMap;
use std::error::Error;
use std::{env, fs};

pub struct GrepConfig {
    search_string: String,
    search_file_name: String,
    ignore_case: bool,
}

impl GrepConfig {
    pub fn new(args: &[String]) -> Result<GrepConfig, &'static str> {
        if args.len() < 2 {
            Err("Too few argments - Syntax: rust_mini_grep <SEARCH_STRING> <SEARCH_FILE_NAME>")
        } else {
            let config = GrepConfig {
                search_string: args[0].clone(),
                search_file_name: args[1].clone(),
                ignore_case: match env::var("CASE_INSENSITIVE") {
                    Err(_) => false,
                    Ok(flag) => flag.parse().unwrap_or(false)
                },
            };
            Ok(config)
        }
    }

    pub fn get_search_string(&self) -> &str {
        self.search_string.as_str()
    }

    pub fn get_search_file_name(&self) -> &str {
        self.search_file_name.as_str()
    }

    pub fn is_ignore_case(&self) -> bool {
        self.ignore_case
    }
}

pub struct SearchResults {
    results: IndexMap<i32, (String, Vec<usize>)>,
}

impl SearchResults {
    pub fn grep(config: &GrepConfig) -> Result<SearchResults, Box<dyn Error>> {
        let mut search_string = config.get_search_string();
        let ignore_case = config.is_ignore_case();
        let search_file_name = config.get_search_file_name();

        let file_data = fs::read_to_string(search_file_name)?;

        let temp_search_string;
        if ignore_case {
            temp_search_string = search_string.to_lowercase();
            search_string = temp_search_string.as_str();
        }

        let mut results_map: IndexMap<i32, (String, Vec<usize>)> = IndexMap::new();

        for one_line in file_data.lines().zip(1..) {
            let orignal_one_line = one_line.0.to_string();
            let mut currunet_str: &str = one_line.0;
            let mut current_index: usize = 0;
            let mut previous_index: usize = 0;
            let mut indexes_vector = Vec::new();

            let temp_orignal_one_line;
            if ignore_case {
                temp_orignal_one_line = orignal_one_line.to_lowercase();
                currunet_str = temp_orignal_one_line.as_str();
            }
            loop {
                currunet_str = &currunet_str[current_index..];
                match currunet_str.find(search_string) {
                    Some(index) => {
                        indexes_vector.push(previous_index + index);
                        current_index = index + search_string.len();
                        previous_index += current_index;
                    }
                    None => break,
                };
            }
            if !indexes_vector.is_empty() {
                results_map.insert(one_line.1, (orignal_one_line, indexes_vector));
            }
        }

        let search_results: SearchResults = SearchResults {
            results: results_map,
        };

        Ok(search_results)
    }

    pub fn print_grep_output(&self, config: &GrepConfig) {
        println!(
            "The search string: \"{}\" found in the file <{}> at following indexes:",
            config.search_string, config.search_file_name,
        );
        println!("--------------------------------------------------------------------------");
        for each_line in &self.results {
            println!(
                "Line {}:\"{}\": {}",
                each_line.0,
                each_line.1 .0,
                self.concatenate_indexes(&each_line.1 .1),
            );
        }
    }

    fn concatenate_indexes(&self, indexes_vector: &Vec<usize>) -> String {
        let mut concatenated_indexes_string: String = String::new();
        if !indexes_vector.is_empty() {
            for each_index in indexes_vector {
                concatenated_indexes_string.push_str(&each_index.to_string());
                concatenated_indexes_string.push_str(", ");
            }
            concatenated_indexes_string = concatenated_indexes_string
                .trim_end_matches(", ")
                .to_string();
        }
        concatenated_indexes_string
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn get_config_instance() -> Result<GrepConfig, &'static str> {
        let args = vec![String::from("or"), String::from("test.txt")];
        GrepConfig::new(&args)
    }

    #[test]
    fn test_config_new() {
        let config = get_config_instance().expect("Unable to create input parameters");

        assert_eq!(config.get_search_string(), "or");
        assert_eq!(config.get_search_file_name(), "test.txt")
    }

    #[test]
    fn test_grep_function() {
        let config = get_config_instance().expect("Unable to create input parameters");
        let search_results = SearchResults::grep(&config).expect("Error in grep function");

        search_results.print_grep_output(&config);

        assert_eq!(5, search_results.results.len());

        assert_eq!(2, search_results.results.keys()[0]);
        assert_eq!(4, search_results.results.keys()[1]);
        assert_eq!(6, search_results.results.keys()[2]);
        assert_eq!(7, search_results.results.keys()[3]);
        assert_eq!(10, search_results.results.keys()[4]);

        assert_eq!(
            "3",
            search_results.concatenate_indexes(&search_results.results.get(&2).unwrap().1)
        );
        assert_eq!(
            "20",
            search_results.concatenate_indexes(&search_results.results.get(&4).unwrap().1)
        );
        assert_eq!(
            "12",
            search_results.concatenate_indexes(&search_results.results.get(&6).unwrap().1)
        );
        assert_eq!(
            "16, 27",
            search_results.concatenate_indexes(&search_results.results.get(&7).unwrap().1)
        );
        assert_eq!(
            "8",
            search_results.concatenate_indexes(&search_results.results.get(&10).unwrap().1)
        );
    }
}
