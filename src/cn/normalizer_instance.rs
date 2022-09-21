use std::collections::HashMap;
use json::JsonValue;

#[derive(Debug)]
pub struct Normalizer { // Matches the regex objects and returns the match
    pub taxonomy_map: HashMap<String, HashMap<String, String>>,
    pub type_map: HashMap<String, HashMap<String, String>>
}

impl Normalizer { 
    // fn _add_type(&self, taxonomized_log: &mut JsonValue, type_mapping: &HashMap<String, String>) {
    //     let mut typemapper: HashMap<&str, Vec<&str>> = HashMap::new();
    //     let xyz = taxonomized_log.clone();
    //     for (field, _value) in xyz.entries() {
    //         if let Some(data_type) = type_mapping.get(field) {
    //             if let Some(value) = typemapper.get_mut(data_type.as_str()) {
    //                 value.push(field)
    //             }
    //             else {
    //                 typemapper.insert(data_type, vec![field]);
    //             }
    //         }
    //     }
    //     for (key, value) in typemapper {
    //         taxonomized_log[key] = value.join(" ").as_str().into();
    //     }
    // }

    pub fn _add_type(&self, taxonomized_log: &mut JsonValue, typemapper: &mut JsonValue,id: &str) {
        let type_map = self.type_map.get(id).unwrap();
        for (field, _value) in taxonomized_log.entries() {
            if let Some(data_type) = type_map.get(field) {
                if typemapper.has_key(&data_type) {
                    let _ = typemapper[data_type].push(field);
                }
                else {
                    typemapper[data_type] = json::array![field];
                }
            }
        }
        for (key, value) in typemapper.entries() {
            taxonomized_log.insert(&key, value.clone()).unwrap();
        }
    }

    pub fn _add_taxonomy(&self, parsed_log: &JsonValue, id: &str, normalized_log: &mut JsonValue) {
        let tax_map = self.taxonomy_map.get(id).unwrap();
        for (key, value) in parsed_log.entries() {
            let mut taxonomy = key.to_lowercase();
            if let Some(replace_str) = tax_map.get(&taxonomy) {
                taxonomy = replace_str.to_string();
            }
            normalized_log.insert(&taxonomy, value.to_owned()).unwrap();
        }
    }
}
