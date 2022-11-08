use std::{os::raw::c_char, ffi::CStr};

use json::{JsonValue, object};

fn _json_parser (raw_log: &JsonValue, str_log: &mut JsonValue, key_: &str) {
    // JSON Format //
    // Company=ABC Company;Product=SystemDefender;Version=1.13;EventID=console_login;Username=jsmith;Name=John Smith;authType=interactivePassword;

    for (key, value) in raw_log.entries() {
        if value.is_empty() {
            continue;
        }
        if value.is_object() {
            _json_parser(&value, str_log, &format!("{key_}{key}_"));
        }
        else if value.is_array() {
            if value[0].is_object() {
                for index in 0..value.len() {
                    let obj = &value[index];
                    if obj.len() == 2 && obj.has_key("key") && obj.has_key("value") {
                        str_log.insert(&format!("{key_}{key}_{}", obj["key"].to_string()), obj["value"].to_owned()).unwrap();
                        continue;
                    }
                    _json_parser(obj, str_log, &format!("{key_}{key}_{index}_"));
                }
            }
            else {
                str_log.insert(&format!("{key_}{key}"), value.members().map(|x| x.to_string()).collect:: <Vec<String>>().join(", ")).unwrap();
            }
        }
        else {
            str_log.insert(&format!("{key_}{key}"), value.clone()).unwrap();
        } 
    }
}

#[no_mangle]
pub unsafe extern fn parser(log: *const c_char) -> *mut JsonValue {
    let mut str_log = object! {};
    let cchar = CStr::from_ptr(log).to_str().unwrap();
    let (_, log) = cchar.split_once("{").unwrap();
    let (json_log, _) = log.rsplit_once("}").unwrap();
    _json_parser(&json::parse(&format!("{{{json_log}}}")).unwrap(), &mut str_log, "");
    Box::into_raw(Box::new(str_log))
}