use std::{collections::{HashMap}};
use serde_json::{json, Value, Map};

pub struct Normalizer {
    pub parsed_log: Value,
    pub taxonomy_mapping: HashMap<String, String>,
    pub type_mapping: HashMap<String, String>
}

impl Normalizer {
    pub fn normalize(&self) -> Value {
        let mut taxonomized_log = self.taxonomize(); 
        self._add_type(&mut taxonomized_log);
        taxonomized_log
    }

    fn _add_type(&self, taxonomized_log: &mut Value) {
        let mut typemapper = Map::new();
        for (field, _value) in taxonomized_log.as_object().unwrap() {
            if self.type_mapping.contains_key(field) {
                let data_type = &self.type_mapping[field];
                if typemapper.contains_key(data_type) {
                    typemapper[data_type].as_array_mut().unwrap().push(json!(field));
                }
                else {
                    typemapper.insert(data_type.to_string(), json!([field]));
                }
            }
        }
        for (key, value) in Value::Object(typemapper).as_object().unwrap() {
            taxonomized_log[key] = value.clone();
        }
    }

    fn taxonomize(&self) -> Value {
        let mut normalized_log = json!({});
        for (key, value) in self.parsed_log.as_object().unwrap() { 
            let mut taxonomy = key.to_lowercase();
            if self.taxonomy_mapping.contains_key(&taxonomy) {
                taxonomy = self.taxonomy_mapping[&taxonomy].clone();
            }
            normalized_log[taxonomy] = value.to_owned();
        }
        normalized_log
    }
}