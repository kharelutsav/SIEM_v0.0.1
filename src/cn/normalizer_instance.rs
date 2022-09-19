use std::collections::HashMap;
use json::{JsonValue, object};

#[derive(Debug)]
pub struct Normalizer { // Matches the regex objects and returns the match
    pub taxonomy_map: HashMap<String, HashMap<String, String>>,
    pub type_map: HashMap<String, HashMap<String, String>>
}

impl Normalizer { 
    // fn _add_type(&self, taxonomized_log: &mut JsonValue, type_mapping: &HashMap<String, String>) {
    //     let mut typemapper: HashMap<&String, Vec<&str>> = HashMap::new();
    //     let iter_obj = taxonomized_log.clone();
    //     for (field, _value) in iter_obj.entries() {
    //         if let Some(data_type) = type_mapping.get(field) {
    //             if let Some(value) = typemapper.get_mut(data_type) {
    //                 value.push(field)
    //             }
    //             else {
    //                 typemapper.insert(data_type, vec![field]);
    //             }
    //         }
    //     }
    //     for (key, value) in typemapper {
    //         taxonomized_log[key] = value.join(" ").into();
    //     }
    // }

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
            if let Some(replace_str) = taxonomy_mapping.get(&taxonomy) {
                taxonomy = replace_str.to_string();
            }
            normalized_log[taxonomy] = value.to_owned();
        }
        normalized_log
    }

    pub fn normalize(&self, str_log: JsonValue, id: &str) -> JsonValue {
        let tax_map = &self.taxonomy_map[id];
        let type_map = &self.type_map[id];
        let mut taxonomized_log = self._add_taxonomy(tax_map, str_log); 
        self._add_type(&mut taxonomized_log, type_map);
        taxonomized_log
    }

}
