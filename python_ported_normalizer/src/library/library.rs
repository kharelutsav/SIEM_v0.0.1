use serde_json::Value;
use pyo3::prelude::*;
use std::{collections::HashMap, fs};

fn serde_value_to_object(val: &Value, py: Python<'_>) -> PyObject {
    match val {
        Value::Null => py.None(),
        Value::Bool(b) => b.to_object(py),
        Value::Number(n) => {
            let oi64 = n.as_i64().map(|i| i.to_object(py));
            let ou64 = n.as_u64().map(|i| i.to_object(py));
            let of64 = n.as_f64().map(|i| i.to_object(py));
            oi64.or(ou64).or(of64).expect("number too large")
        },
        Value::String(s) => s.to_object(py),
        Value::Array(v) => {
            let inner: Vec<_> = v.iter().map(|x| serde_value_to_object(x, py)).collect();
            inner.to_object(py)
        },
        Value::Object(m) => {
            let inner: HashMap<_, _> =
                m.iter().map(|(k, v)| (k, serde_value_to_object(v, py))).collect();
            inner.to_object(py)
        },
    }
}

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct PyValue <'a> (pub &'a Value);

impl ToPyObject for PyValue<'_> {
    fn to_object(&self, py: Python) -> PyObject {
        serde_value_to_object(&self.0, py)
    }
}

impl IntoPy<PyObject> for PyValue<'_> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        serde_value_to_object(&self.0, py)
    }
}

// fn capture_regex_rust_capture_iter(&mut self, log: &str, py: Python) -> PyResult<Py<PyDict>>  {
//     let mut finds = self._find_sig.captures_iter(log);
//     let obj = PyDict::new(py);
//     let x: &[_] = &['\'', ';', '\"'];
//     while let Some(_match) = finds.next() {
//         if let (Some(key), Some(value)) = (_match.get(1), _match.get(2)) {
//             let value = &log[value.start()..value.end()].trim_matches(x);
//             if let Ok(value) = value.parse::<usize>() {
//                 obj.set_item(&log[key.start()..key.end()], value).unwrap();                    
//             } else {
//                 obj.set_item(&log[key.start()..key.end()], value).unwrap();              
//             }

//         }
//     }
//     Ok(obj.into())
// }

pub fn csv_to_hashmap (path_of_file: &str) -> HashMap<String, String> { // Csv to hashmap <String, String>
    let mut new_hashmap: HashMap<String, String> = HashMap::new();
    let binding = fs::read_to_string(path_of_file).unwrap();
    let read_file = binding.lines();
    for lines in read_file {
        let pair = lines.split(",").collect:: <Vec<&str>> ();
        new_hashmap.insert(pair[0].to_string(), pair[1].to_string());
    };
    new_hashmap
}