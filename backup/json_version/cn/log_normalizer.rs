use std::{collections::{HashMap}};
use json::{JsonValue, object};

pub struct Normalizer {
    pub parsed_log: JsonValue,
    pub taxonomy_mapping: HashMap<String, String>,
    pub type_mapping: HashMap<String, String>
}

impl Normalizer {
    pub fn normalize(&self) -> JsonValue {
        let mut taxonomized_log = self.taxonomize(); 
        self._add_type(&mut taxonomized_log);
        taxonomized_log
    }

    fn _add_type(&self, taxonomized_log: &mut JsonValue) {
        let mut typemapper = json::object! {};
        for (field, _value) in taxonomized_log.entries() {
            if self.type_mapping.contains_key(field) {
                let data_type = &self.type_mapping[field];
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

    fn taxonomize(&self) -> JsonValue {
        let mut normalized_log = object! {};
        for (key, value) in self.parsed_log.entries() {
            let mut taxonomy = key.to_lowercase();
            if self.taxonomy_mapping.contains_key(&taxonomy) {
                taxonomy = self.taxonomy_mapping[&taxonomy].clone();
            }
            normalized_log[taxonomy] = value.to_owned();
        }
        normalized_log
    }
}