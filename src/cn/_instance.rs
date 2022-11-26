use std::collections::HashMap;
use crate::repo::mongo_repo::Mongo;
use futures::TryStreamExt;
use libloading::Library;
use regex::Regex;
use super::normalizer_instance::Normalizer;
use super::processor_instance::Processor;
use super::parser_instance::Parser;

pub async fn init(db: Mongo) -> (Processor, Parser, Normalizer) {
    let mut cursor = db.get_active_normalizers_cursor().await.unwrap();

    let mut type_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut taxonomy_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut regex_objects: HashMap<String, Regex> = HashMap::new();
    let mut add_fields: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut library = HashMap::new();
    // let mut remove_fields: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut internal_regex_objects: HashMap<String, Vec<Regex>> = HashMap::new();
    let mut log_type: HashMap<String, String> = HashMap::new();
    
    while let Some(normalizer) = cursor
        .try_next()
        .await
        .ok()
        .expect("Error blah.. blah.. blah..")
    {
        let id = normalizer.id.unwrap().to_string();
        type_map.insert(id.clone(), normalizer.type_mapping);
        type_map.shrink_to_fit();
        taxonomy_map.insert(id.clone(), normalizer.taxonomy_mapping);
        taxonomy_map.shrink_to_fit();
        regex_objects.insert(id.clone(), Regex::new(&normalizer.regex).unwrap());
        add_fields.insert(id.clone(), HashMap::from([("norm_id".to_string(), normalizer.norm_id), ("device_category".to_string(), normalizer.category)]));
        internal_regex_objects.insert(id.clone(), normalizer.internal_regex.into_iter().map(|x|Regex::new(&x).unwrap()).collect:: <Vec<Regex>>());
        if normalizer.log_type == "JSON" {
            let parser;
            unsafe {
                parser = Library::new("/Users/logpoint/Downloads/libgreet-rs/target/x86_64-apple-darwin/release/libparser.dylib").unwrap();
            }
            library.insert(id.clone(), parser);
        }
        log_type.insert(id.clone(), normalizer.log_type);
    };
    let normalizer = Normalizer {
        taxonomy_map,
        type_map
    };
    let parser = Parser {
        log_type,
        regex: Regex::new("\\{.*\\}").unwrap(),
        regex_objects: regex_objects.to_owned(),
        library: library
    };
    let preprocessor = Processor {
        internal_regex_objects,
        add_fields,
        remove_fields: Default::default(),
        regex_objects
    };
    return (preprocessor, parser, normalizer);
}
