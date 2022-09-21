use std::collections::HashMap;
use json::JsonValue;

#[derive(Debug)]
pub struct Normalizer { // Matches the regex objects and returns the match
    pub taxonomy_map: HashMap<String, HashMap<String, String>>,
    pub type_map: HashMap<String, HashMap<String, String>>
}

impl Normalizer { 
    pub fn _add_type(&self, taxonomized_log: &mut JsonValue, typemapper: &mut HashMap<String, Vec<String>>, id: &str) {
        let type_map = self.type_map.get(id).unwrap();
        for (field, _value) in taxonomized_log.entries() {
            if let Some(data_type) = type_map.get(field) {
                if let Some(value) = typemapper.get_mut(data_type.as_str()) {
                    value.push(field.to_string())
                }
                else {
                    typemapper.insert(data_type.to_string(), vec![field.to_string()]);
                }
            }
        }
        for (key, value) in typemapper {
            let value: JsonValue = value.join(" ").into();
            taxonomized_log.insert(key, value).unwrap();
        }
    }

    // pub fn _add_type(&self, taxonomized_log: &mut JsonValue, typemapper: &mut JsonValue,id: &str) {
    //     let type_map = self.type_map.get(id).unwrap();
    //     for (field, _value) in taxonomized_log.entries() {
    //         if let Some(data_type) = type_map.get(field) {
    //             if typemapper.has_key(&data_type) {
    //                 let _ = typemapper[data_type].push(field);
    //             }
    //             else {
    //                 typemapper[data_type] = json::array![field];
    //             }
    //         }
    //     }
    //     for (key, value) in typemapper.entries() {
    //         taxonomized_log.insert(&key, value.clone()).unwrap();
    //     }
    // }

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
