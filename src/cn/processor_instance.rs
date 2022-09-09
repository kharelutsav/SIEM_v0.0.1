use std::collections::HashMap;
use json::JsonValue;
use regex::Regex;

#[derive(Debug)]
pub struct Processor { // Matches the regex objects and returns the match
    pub internal_regex_objects: HashMap<String, Vec<Regex>>,
    pub add_fields: HashMap<String, HashMap<String, String>>,
    pub remove_fields: HashMap<String, HashMap<String, String>>,
    pub regex_objects: HashMap<String, Regex>
}

impl Processor {

    fn _capture_regex_match (&self, obj: &mut JsonValue, regex: &Regex, log: &str) {
        let header = regex.captures(log).unwrap();
        for name in regex.capture_names() {
            if !name.is_none() {
                obj.insert(name.unwrap(), header[name.unwrap()].to_string()).unwrap();
            }
        };
    }

    fn _post_processor(&self, id: &String, log: &str, str_log: &mut JsonValue) {
        let _internal_regex = &self.internal_regex_objects[id];
        if !_internal_regex.is_empty() {
            for regex in _internal_regex {
                if regex.is_match(log) {
                    self._capture_regex_match(str_log, regex, log);
                }
            }
        }
    }

    pub fn match_log(&self, log: &str) -> Option<String> {
        for (id, regex) in &self.regex_objects {
            if regex.is_match(log) {
                return Some(id.to_owned());
            }
        }
        return None;
    }
    
}