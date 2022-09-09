use std::collections::HashMap;
use json::{JsonValue, object};
use regex::Regex;
use super::normalizer_instance::Normalizer;


impl Normalizer {
    fn _capture_regex_match (&self, obj: &mut JsonValue, regex: &Regex, log: &str) {
        let header = regex.captures(log).unwrap();
        for name in regex.capture_names() {
            if !name.is_none() {
                obj.insert(name.unwrap(), header[name.unwrap()].to_string()).unwrap();
            }
        };
    }

    fn _post_processor(&self, id: &String, log: &str, str_log: &mut JsonValue) {
        let _internal_regex = &self.active_normalizers[id]._internal_regex_objects;
        if !_internal_regex.is_empty() {
            for regex in _internal_regex {
                if regex.is_match(log) {
                    self._capture_regex_match(str_log, regex, log);
                }
            }
        }
    } 

    fn _add_type(&self, taxonomized_log: &mut JsonValue, type_mapping: &HashMap<String, String>) {
        let mut typemapper = json::object! {};
        for (field, _value) in taxonomized_log.entries() {
            if type_mapping.contains_key(field) {
                let data_type = &type_mapping[field];
                if typemapper.has_key(&data_type) {
                    let _ = typemapper[data_type].push(field);
                }
                else {
                    typemapper[data_type] = json::array![field];
                }
            }
        }
        for (key, value) in typemapper.entries() {
            taxonomized_log[key] = value.clone();
        }
    }

    fn _add_taxonomy(&self, taxonomy_mapping: &HashMap<String, String>, parsed_log: JsonValue) -> JsonValue {
        let mut normalized_log = object! {};
        for (key, value) in parsed_log.entries() {
            let mut taxonomy = key.to_lowercase();
            if taxonomy_mapping.contains_key(&taxonomy) {
                taxonomy = taxonomy_mapping[&taxonomy].clone();
            }
            normalized_log[taxonomy] = value.to_owned();
        }
        normalized_log
    }

    fn _add_taxonomy_and_type_map(&self, str_log: JsonValue, id: String) -> JsonValue {
        let tax_map = &self.taxonomy_map[&id];
        let type_map = &self.type_map[&id];
        let mut taxonomized_log = self._add_taxonomy(tax_map, str_log); 
        self._add_type(&mut taxonomized_log, type_map);
        taxonomized_log
    }

    pub fn normalize_log(&self, log: &str) -> Option<JsonValue> {
        let mut str_log = object! {};
        for (regex, id) in &self.regex_objects {
            if regex.is_match(log) {
                let log_type = &self.log_type[id];
                match log_type.as_str() {
                    "JSON" => {
                        let index = self.regex.find(log).unwrap();
                        self._json_parser(&json::parse(&log[index.start()..index.end()]).unwrap(), &mut str_log, "");
                    },
                    "LEEF" => {
                        self._capture_regex_match(&mut str_log, regex, log);
                        self._leef_parser(str_log.remove("attributes").to_string(), &mut str_log)
                    },
                    _ => todo!()
                }
                self._post_processor(id, log, &mut str_log);
                return Some(self._add_taxonomy_and_type_map(str_log, id.to_string()));
            }
        }
        return None;
    }
}
