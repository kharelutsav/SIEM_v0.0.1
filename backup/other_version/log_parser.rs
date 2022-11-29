use json::{JsonValue, object};

pub struct Parser {
    pub raw_log: JsonValue
}

impl Parser {
    pub fn build_stru_log (&self) -> JsonValue {
        let mut test_str = object! {};
        self.iterator(&mut test_str);
        test_str
    }

    fn iterator(&self, str_log: &mut JsonValue) {
        for (key, value) in self.raw_log.entries() {
            if value.is_empty() {
                continue;
            }
            if value.is_object() {
                self.nested_iterator(value, str_log, &key);
            }
            else if value.is_array() {
                if value[0].is_object() {
                    for index in 0..value.len() {
                        let obj = &value[index];
                        if obj.len() == 2 && obj.has_key("key") && obj.has_key("value") {
                            str_log[obj["key"].to_string()] = obj["value"].to_owned();
                            continue;
                        }
                        self.nested_iterator(obj, str_log, &format!("{}{}", key, index));
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

    fn nested_iterator (&self, raw_log: &JsonValue, str_log: &mut JsonValue, something: &str) {
        for (key, value) in raw_log.entries() {
            if value.is_empty() {
                continue;
            }
            if value.is_object() {
                self.nested_iterator(&value, str_log, key);
            }
            else if value.is_array() {
                if value[0].is_object() {
                    for index in 0..value.len() {
                        let obj = &value[index];
                        if obj.len() == 2 && obj.has_key("key") && obj.has_key("value") {
                            str_log[format!("{something}_{}", obj["key"].to_string())] = obj["value"].to_owned();
                            continue;
                        }
                        self.nested_iterator(obj, str_log, &format!("{}{}", key, index));
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
}