use serde_json::{Value, Map, json};


pub struct Parser {
    pub raw_log: Value
}

impl Parser {
    pub fn build_stru_log (&self) -> Value {
        let mut test_str = Map::new();
        self.iterator(&mut test_str);
        Value::Object(test_str)
    }

    fn iterator(&self, str_log: &mut Map<String, Value>) {
        for (key, value) in self.raw_log.as_object().unwrap() {
            if value.is_null() {
                continue;
            }
            if value.is_object() {
                self.nested_iterator(value.as_object().unwrap(), str_log, &key);
            }
            else if value.is_array() {
                if value[0].is_object() {
                    for index in 0..value.as_array().unwrap().len() {
                        let obj = value[index].as_object().unwrap();
                        if obj.len() == 2 && obj.contains_key("key") && obj.contains_key("value") {
                            str_log.insert(obj["key"].to_string(), obj["value"].to_owned());
                            continue;
                        }
                        self.nested_iterator(obj, str_log, &format!("{}{}", key, index));
                    }
                }
                else {
                    str_log.insert(key.to_string(), json!(value.as_array().unwrap().into_iter().map(|x| x.to_string()).collect:: <Vec<String>>().join(", ")));
                }
            }
            else {
                str_log.insert(key.to_string(), value.clone());
            }
        }
    }

    fn nested_iterator (&self, raw_log: &Map<String, Value>, str_log: &mut Map<String, Value>, something: &str) {
        for (key, value) in raw_log {
            if value.is_null() {
                continue;
            }
            if value.is_object() {
                self.nested_iterator(&value.as_object().unwrap(), str_log, key);
            }
            else if value.is_array() {
                if value[0].is_object() {
                    for index in 0..value.as_array().unwrap().len() {
                        let obj = value[index].as_object().unwrap();
                        if obj.len() == 2 && obj.contains_key("key") && obj.contains_key("value") {
                            str_log.insert(format!("{something}_{}", obj["key"].as_str().unwrap()), obj["value"].to_owned());
                            continue;
                        }
                        self.nested_iterator(obj, str_log, &format!("{}{}", key, index));
                    }
                }
                else {
                    str_log.insert(format!("{something}_{key}"), json!(value.as_array().unwrap().into_iter().map(|x| x.to_string()).collect:: <Vec<String>>().join(", ")));
                }
            }
            else {
                str_log.insert(format!("{something}_{key}"), value.clone());
            } 
        }
    }
}