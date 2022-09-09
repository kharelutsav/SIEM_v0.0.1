use std::collections::HashMap;
use json::JsonValue;
use super::normalizer_instance::Normalizer;

impl Normalizer {
 
    pub fn _json_parser (&self, raw_log: &JsonValue, str_log: &mut JsonValue, key_: &str) {
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

    pub fn _leef_parser(&self, raw_log: String, test_str: &mut JsonValue) {
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

    pub fn _name_value_pair_parser(&self) {
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

    pub fn _generic_list_parser(&self) {
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
