mod library;

use std::collections::HashMap;
use library::library::{PyValue, csv_to_hashmap};
use mimalloc::MiMalloc;
use pyo3::{prelude::*, exceptions::PyValueError, types::PyDict};
use serde_json::{Value, Map};
use regex::Regex;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[pyclass]
struct TestRegex {
    _sig_rs: Regex,
    _find_sig: regex::Regex,
    _taxonomy_map: HashMap<String, String>,
    _type_map: HashMap<String, String>,
    _parsed_data: Map<String, Value>,
    typemapper: HashMap<String, String>,
    #[pyo3(get)]
    normalized_data: Py<PyDict>
}

#[pymethods]
impl TestRegex {
    #[new]
    fn py_new(_sig: &str, find_sig: &str, py: Python) -> PyResult<Self> {
        if _sig.is_empty() {
            Err(PyValueError::new_err("Field cannot be empty"))
        } else {
            Ok(
                TestRegex {
                    _sig_rs: Regex::new(_sig).unwrap(),
                    _find_sig: Regex::new(find_sig).unwrap(),
                    _taxonomy_map: csv_to_hashmap("/Users/logpoint/string_sum/src/conf/taxonomy.csv"),
                    _type_map: csv_to_hashmap("/Users/logpoint/string_sum/src/conf/type.csv"),
                    _parsed_data: Map::new(),
                    typemapper: HashMap::with_capacity(10),
                    normalized_data: PyDict::new(py).into()
                }
            )
        }
    }

    fn normalize(&mut self, log: &str, py: Python) -> &'static str {
        if self._sig_rs.is_match(log) {
            self._normalize(log, py);
            return "SUCCESS"            
        }
        return "FAILURE";
    }
}

impl TestRegex {
    
    fn _parse_header(&mut self, log: &str) {
        let captured_names = self._sig_rs.capture_names();
        if let Some(_match) = self._sig_rs.captures(log) {
            for name in captured_names {
                if let Some(name) = name {
                    self._parsed_data.insert(name.to_string(), _match[name].into());
                }
            };
        }
    }

    fn _find_all(&mut self, log: &str) {
        let log = &log[self._sig_rs.shortest_match(log).unwrap()..];
        let mut finds = self._find_sig.find_iter(log);
        let x: &[_] = &['\'', ';', '\"'];
        while let Some(_match) = finds.next() {
            if let Some((key, value)) = log[_match.start().._match.end()].split_once("=") {
                let value = value.trim_matches(x);
                self._parsed_data.insert(key.into(), value.into());
            }
        }
    }

    fn _add_taxonomy_type (&mut self, py: Python) {
        for (key, value) in &self._parsed_data {
            let key = key.to_lowercase();
            let taxonomy = self._taxonomy_map.get(&key).unwrap_or(&key);
            self.normalized_data.as_ref(py).set_item(taxonomy, PyValue(value)).unwrap();

            if let Some(data_type) = self._type_map.get(taxonomy){
                if let Some(value) = self.typemapper.get_mut(data_type.as_str()) {
                    value.push_str(" ");
                    value.push_str(taxonomy);
                }
                else {
                    self.typemapper.insert(data_type.into(), taxonomy.to_string());
                }
            }
        }
        
        for (key, value) in &self.typemapper {
            self.normalized_data.as_ref(py).set_item(key, value).unwrap();
        }
    }

    fn _normalize(&mut self, log: &str, py: Python) {
        self._parse_header(log);
        self._find_all(log);
        self._add_taxonomy_type(py)
    }

}

#[pymodule]
fn string_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TestRegex>()?;
    Ok(())
}
