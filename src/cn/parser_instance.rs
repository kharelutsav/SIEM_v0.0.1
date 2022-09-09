use std::collections::HashMap;
use json::{JsonValue, object};
use regex::Regex;

#[derive(Debug)]
pub struct Parser {
    pub log_type: HashMap<String, String>,
    pub regex: Regex,
    pub regex_objects: HashMap<String, Regex>
}

impl Parser {

    pub fn parse(&self, id: &str, log: &str) -> JsonValue {
        let mut str_log = object! {};
        match self.log_type[id].as_str() {
            "JSON" => {
                let index = self.regex.find(log).unwrap();
                self._json_parser(&json::parse(&log[index.start()..index.end()]).unwrap(), &mut str_log, "");
            },
            "LEEF" => {
                self._capture_regex_match(&mut str_log, id, log);
                self._leef_parser(str_log.remove("attributes").to_string(), &mut str_log)
            },
            _ => todo!()
        }
        str_log
    }

    fn _capture_regex_match (&self, obj: &mut JsonValue, id: &str, log: &str) {
        let regex = &self.regex_objects[id];
        let header = regex.captures(log).unwrap();
        for name in regex.capture_names() {
            if !name.is_none() {
                obj.insert(name.unwrap(), header[name.unwrap()].to_string()).unwrap();
            }
        };
    }
 
    fn _json_parser (&self, raw_log: &JsonValue, str_log: &mut JsonValue, key_: &str) {
        // JSON Format //
        // Company=ABC Company;Product=SystemDefender;Version=1.13;EventID=console_login;Username=jsmith;Name=John Smith;authType=interactivePassword;

        for (key, value) in raw_log.entries() {
            if value.is_empty() {
                continue;
            }
            if value.is_object() {
                self._json_parser(&value, str_log, &format!("{key_}{key}_"));
            }
            else if value.is_array() {
                if value[0].is_object() {
                    for index in 0..value.len() {
                        let obj = &value[index];
                        if obj.len() == 2 && obj.has_key("key") && obj.has_key("value") {
                            str_log[format!("{key_}{key}_{}", obj["key"].to_string())] = obj["value"].to_owned();
                            continue;
                        }
                        self._json_parser(obj, str_log, &format!("{key_}{key}_{index}_"));
                    }
                }
                else {
                    str_log[format!("{key_}{key}")] = value.members().map(|x| x.to_string()).collect:: <Vec<String>>().join(", ").into();
                }
            }
            else {
                str_log[format!("{key_}{key}")] = value.clone();
            } 
        }
    }

    fn _leef_parser(&self, raw_log: String, test_str: &mut JsonValue) {
        // LEEF Format //
        // LEEF:1.0|ABC Company|SystemDefender|1.13|console_login|devTimeFormat=yyyy-MM-dd'T'HH:mm:ss.SSSZ	devTime=2017-10-18T11:26:03.060+0200	usrName=flastname	name=Firstname Lastname	authType=interactivePassword	src=192.168.0.1
        // LEEF:2.0|ABC Company|SystemDefender|1.13|console_login|^|devTimeFormat=yyyy-MMdd'T'HH:mm:ss.SSSZ^devTime=2017-10-18T11:26:03.060+0200^usrName=flastname^name=Firstname Lastname^authType=interactivePassword^src=192.168.0.1
        
        let delimiter;
        if test_str["leef_version"] == "1.0" {
            delimiter = "\t"
        } else {
            delimiter = "^"
        }

        let mut field_value= raw_log.split(delimiter);

        while let Some(pair) = field_value
            .next()
            {   
                if !pair.is_empty() {
                    let mut temp = pair.splitn(2, "=");
                    test_str.insert(temp.next().unwrap(), temp.next().unwrap().to_string()).unwrap();
                };
            }
    }

    fn _name_value_pair_parser(&self) {
        // Name Value Pair Format //
        // Company=ABC Company;Product=SystemDefender;Version=1.13;EventID=console_login;Username=jsmith;Name=John Smith;authType=interactivePassword;

        let raw_log = "Company=ABC=Company;Product=System;DefenderVersion=1.13;EventID=console_login;Username=jsmith;Name=John Smith;authType=interactivePassword;";
    
        let mut new_map = HashMap::new();
        let mut list = raw_log.split(";");
    
        while let Some(pair) = list
            .next()
            {
                if pair.is_empty() {
                    continue;
                };
                let mut temp_obj = pair.splitn(2, "=");
                new_map.insert(temp_obj.next().unwrap(), temp_obj.next().unwrap());
            }
    }

    fn _generic_list_parser(&self) {
        // Generic List Format //
        // ABC Company;1.13;console_login;jsmith;John Smith;interactivePassword;

        let info = ["vendor", "version", "action", "user_name", "name", "mode"];
        let raw_log = "ABC Company;1.13;console_login;jsmith;John Smith;interactivePassword;";
     
        let mut new_map = HashMap::new();
        let mut list = raw_log.split(";");
    
        for field in info {
            let value = list.next().unwrap();
            if !value.is_empty() {
                new_map.insert(field, value);
            }
        }
    }

}
