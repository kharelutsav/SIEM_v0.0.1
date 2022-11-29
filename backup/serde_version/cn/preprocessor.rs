use std::{collections::HashMap, fs};
use json::{JsonValue};
use regex::{Regex};

#[derive(Clone)]
struct Helper { // Helper function
    _norm_id: String,
    _device_category: String,
    internal_regex_objects: Vec<Regex>
}

pub struct Preprocessor { // Matches the regex objects and returns the match
    active_regex: Vec<String>,
    regex_details: HashMap<String, Helper>,
    regex_objects: Vec<Regex>,
    taxonomy_map: HashMap<String, HashMap<String, String>>,
    type_map: HashMap<String, HashMap<String, String>>
}

impl Preprocessor {
    fn load_regex_details(&mut self, all: JsonValue) {
        for regex in &self.active_regex {

            let temp_object = &all[regex];

            self.regex_objects.push(Regex::new(regex).unwrap()); // Prepared regex objects for enhanced performance

            self.taxonomy_map.insert(regex.to_string(), self.csv_to_hashmap(&temp_object["config_taxonomy"].to_string())); // Load taxonomy config

            self.type_map.insert(regex.to_string(), self.csv_to_hashmap(&temp_object["config_type"].to_string())); // Load type config

            self.regex_details.insert(regex.to_string(), Helper{ // Create specific regex patterns for specific entries
                _norm_id: temp_object["norm_id"].to_string(), // Norm id of the log
                _device_category: temp_object["device_category"].to_string(), // Category of device
                internal_regex_objects: temp_object["possible_regex"].members().map(|x|Regex::new(&x.to_string()).unwrap()).collect:: <Vec<Regex>>()// Final level prepeare regex objects
            });
        }
    }

    pub fn match_regex(&self, log: &str) -> Result<(usize, usize, HashMap<String, String>, HashMap<String, String>), usize> {
        for regex in &self.regex_objects {
            if regex.is_match(log) {
                for internal_regex in &self.regex_details[&regex.to_string()].internal_regex_objects {
                    if internal_regex.is_match(log) {  
                        let one = internal_regex.find(log).unwrap();
                        return Ok((one.start(), one.end(), self.taxonomy_map[&regex.to_string()].clone(), self.type_map[&regex.to_string()].clone()));
                    }
                }
            }
        }
        Err(0)
    }

    fn csv_to_hashmap (&self, path_of_file: &str) -> HashMap<String, String> { // Csv to hashmap <String, String>
        let mut new_hashmap: HashMap<String, String> = HashMap::new();
        let read_file = fs::read_to_string(path_of_file).unwrap();
        for lines in read_file.clone().split_terminator("\n") {
            let pair = lines.split(",").collect:: <Vec<&str>> ();
            new_hashmap.insert(pair[0].to_string(), pair[1].to_string());
        };
        new_hashmap
    }
}

pub fn process() -> Preprocessor {
    let mut preprocessor = Preprocessor {
        active_regex: json::parse(&fs::read_to_string("src/data/json/active_regex.json").unwrap()).unwrap().members().map(|x| x.to_string()).collect:: <Vec<String>>(),
        regex_details: Default::default(),
        regex_objects: vec![],
        taxonomy_map: Default::default(),
        type_map: Default::default()
    };
    preprocessor.load_regex_details(json::parse(&fs::read_to_string("src/data/json/regex_details.json").unwrap()).unwrap());
    return preprocessor
}