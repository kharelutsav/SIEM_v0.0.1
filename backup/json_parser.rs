use json::JsonValue;
use crate::cn::parser::Parser;

impl Parser {
    pub fn _json_parser (&self, raw_log: &JsonValue, str_log: &mut JsonValue) {
        let mut xyz = raw_log.entries();
        while let Some((key, value)) = xyz.next()  {
            if value.is_empty() {
                continue;
            }
            if value.is_object() {
                self._nested_iterator(value, str_log, key);
            }
            else if value.is_array() {
                if value[0].is_object() {
                    for index in 0..value.len() {
                        let obj = &value[index];
                        if obj.len() == 2 && obj.has_key("key") && obj.has_key("value") {
                            str_log[obj["key"].to_string()] = obj["value"].to_owned();
                            continue;
                        }
                        self._nested_iterator(obj, str_log, &format!("{}{}", key, index));
                    }
                }
                else {
                    str_log[key] = value.members().map(|x| x.to_string()).collect:: <Vec<String>>().join(", ").into();
                }
            }
            else {
                str_log[key] = value.clone();
            }
        }
    }

    fn _nested_iterator (&self, raw_log: &JsonValue, str_log: &mut JsonValue, something: &str) {
        let mut xyz = raw_log.entries();
        while let Some((key, value)) = xyz.next()  {
            if value.is_empty() {
                continue;
            }
            if value.is_object() {
                self._nested_iterator(&value, str_log, key);
            }
            else if value.is_array() {
                if value[0].is_object() {
                    for index in 0..value.len() {
                        let obj = &value[index];
                        if obj.len() == 2 && obj.has_key("key") && obj.has_key("value") {
                            str_log[format!("{something}_{}", obj["key"].to_string())] = obj["value"].to_owned();
                            continue;
                        }
                        self._nested_iterator(obj, str_log, &format!("{}{}", key, index));
                    }
                }
                else {
                    str_log[format!("{something}_{key}")] = value.members().map(|x| x.to_string()).collect:: <Vec<String>>().join(", ").into();
                }
            }
            else {
                str_log[format!("{something}_{key}")] = value.clone();
            } 
        }
    }

    // pub fn _json_parser (&self, raw_log: &JsonValue, str_log: &mut JsonValue, some: Option<&str>) {
    //     for (key, value) in raw_log.entries() {
    //         if value.is_empty() {
    //             continue;
    //         }
    //         let mut parent_key = "";
    //         if !some.is_none() {
    //             parent_key = some.unwrap();
    //         }
    //         if value.is_object() {
    //             self._json_parser(&value, str_log, Some(format!("{parent_key}{key}_").as_str()));
    //         }
    //         else if value.is_array() {
    //             if value[0].is_object() {
    //                 for index in 0..value.len() {
    //                     let obj = &value[index];
    //                     if obj.len() == 2 && obj.has_key("key") && obj.has_key("value") {
    //                         str_log[format!("{parent_key}{key}_{}", obj["key"].to_string())] = obj["value"].to_owned();
    //                         continue;
    //                     }
    //                     self._json_parser(obj, str_log, Some(format!("{parent_key}{}_{}_", key, index).as_str()));
    //                 }
    //             }
    //             else {
    //                 str_log[format!("{parent_key}{key}")] = value.members().map(|x| x.to_string()).collect:: <Vec<String>>().join(", ").into();
    //             }
    //         }
    //         else {
    //             str_log[format!("{parent_key}{key}")] = value.clone();
    //         } 
    //     }
}