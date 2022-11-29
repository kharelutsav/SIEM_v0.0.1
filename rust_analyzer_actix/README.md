# rust_analyzer

## API endpoints
```
POST /normalizer -> Creates new normalizer.
    create_normalizer (&self, new_normalizer: Normalizer) -> Result<InsertOneResult, Error>

PUT /normalizer -> Updates old normalizers.
    update_normalizer (&self, updated_normalizer: Normalizer) -> Result<UpdateResult, Error>
    get_normalizer (&self, id: &String) -> Result<Normalizer, Error>

GET /all-normalizers -> Get all normalizers in the system.
    get_all_normalizers (&self) -> Result<Vec<Normalizer>, Error>

GET /active-normalizers -> Get all active normalizers in the system.
    get_active_normalizers (&self) -> Result<Vec<Normalizer>, Error>

PUT /update-status -> Updates the active state of the normalizer.
    update_status (&self, id: &String, status: bool) -> Result<UpdateResult, Error>
```

## preprocessor
```
struct Helper {
    _norm_id: String,
    _device_category: String,
    internal_regex_objects: Vec<Regex>
}
pub struct Preprocessor {
    active_normalizers: HashMap<String, Helper>,
    regex_objects: Vec<Regex>,
    taxonomy_map: HashMap<String, HashMap<String, String>>,
    type_map: HashMap<String, HashMap<String, String>>
}
```
```
process(db: Data<Mongo>) -> Preprocessor
    get_active_normalizers_cursor (&self) -> Result<Cursor<Normalizer>, Error>

match_regex(&self, log: &str) -> Result<(usize, usize, HashMap<String, String>, HashMap<String, String>), usize>
```

## parser
```
pub struct Parser {
    pub raw_log: JsonValue
}
```
```
build_stru_log (&self) -> JsonValue
    iterator(&self, str_log: &mut JsonValue)
        nested_iterator (&self, raw_log: &JsonValue, str_log: &mut JsonValue, something: &str)
```

## normalizer
```
pub struct Normalizer {
    pub parsed_log: JsonValue,
    pub taxonomy_mapping: HashMap<String, String>,
    pub type_mapping: HashMap<String, String>
}
```
```
normalize(&self) -> JsonValue
    taxonomize(&self) -> JsonValue
    _add_type(&self, taxonomized_log: &mut JsonValue)
```

## Update preprocessor when
```
Changes happen to active normalizers.
Normalizer status changes i.e. active to inactive and vice versa.
```