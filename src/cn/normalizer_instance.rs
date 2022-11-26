use std::collections::HashMap;
use json::JsonValue;

#[derive(Debug)]
pub struct Normalizer { // Matches the regex objects and returns the match
    pub taxonomy_map: HashMap<String, HashMap<String, String>>,
    pub type_map: HashMap<String, HashMap<String, String>>
}

impl Normalizer {

    pub fn _normalize(&self, parsed_log: &JsonValue, id: &str, normalized_log: &mut JsonValue, typemapper: &mut HashMap<String, String>) {
        let tax_map = self.taxonomy_map.get(id).unwrap();
        for (key, value) in parsed_log.entries() {
            let mut taxonomy = &key.to_lowercase();
            if let Some(replace_str) = tax_map.get(taxonomy) {
                taxonomy = replace_str;
            }
            normalized_log.insert(taxonomy, value.to_owned()).unwrap();
        }

        let type_map = self.type_map.get(id).unwrap();
        for (field, _value) in normalized_log.entries() {
            if let Some(data_type) = type_map.get(field) {
                if let Some(value) = typemapper.get_mut(data_type.as_str()) {
                    value.push_str(" ");
                    value.push_str(field);
                }
                else {
                    typemapper.insert(data_type.to_string(), field.to_string());
                }
            }
        }
        for (key, value) in typemapper {
            normalized_log.insert(key, value.clone()).unwrap();
        }
    }
}
