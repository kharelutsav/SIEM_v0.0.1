use std::{collections::HashMap, fs};
use crate::{repo::mongo_repo::Mongo, models::cn_model};
use actix_web::web::Data;
use futures::TryStreamExt;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Helper { // Helper function
    pub _norm_id: String,
    pub _device_category: String,
    pub _internal_regex_objects: Vec<Regex>
}

#[derive(Debug)]
pub struct Normalizer { // Matches the regex objects and returns the match
    pub active_normalizers: HashMap<String, Helper>,
    pub regex_objects: Vec<(Regex, String)>,
    pub taxonomy_map: HashMap<String, HashMap<String, String>>,
    pub type_map: HashMap<String, HashMap<String, String>>,
    pub log_type: HashMap<String, String>,
    pub regex: Regex
}

impl Normalizer {
    pub fn update_normalizer (&mut self, id: String, normalizer: &cn_model::Normalizer) {
        if normalizer.active == true {
            self.regex_objects.retain(|x| x.1 != id);
            self.regex_objects.push((Regex::new(&normalizer.regex).unwrap(), id.clone()));
            self.type_map.entry(id.clone()).and_modify(|x| {*x = normalizer.type_mapping.clone()}).or_insert(normalizer.type_mapping.clone());
            self.taxonomy_map.entry(id.clone()).and_modify(|x| {*x = normalizer.taxonomy_mapping.clone()}).or_insert(normalizer.taxonomy_mapping.clone());
            let helper = Helper {
                _norm_id: normalizer.norm_id.clone(),
                _device_category: normalizer.category.clone(),
                _internal_regex_objects: normalizer.internal_regex.clone().into_iter().map(|x|Regex::new(&x).unwrap()).collect:: <Vec<Regex>>()
            };
            self.active_normalizers.entry(id.clone()).and_modify(|x| {*x = helper}).or_insert(Helper {
                _norm_id: normalizer.norm_id.clone(),
                _device_category: normalizer.category.clone(),
                _internal_regex_objects: normalizer.internal_regex.clone().into_iter().map(|x|Regex::new(&x).unwrap()).collect:: <Vec<Regex>>()
            });
            self.log_type.entry(id.clone()).and_modify(|x| *x = normalizer.log_type.clone()).or_insert(normalizer.log_type.clone());
        }
        else if normalizer.active == false {
            self.regex_objects.retain(|x| x.1 != id);
            self.type_map.remove(&id);
            self.taxonomy_map.remove(&id);
            self.active_normalizers.remove(&id);
            self.log_type.remove(&id);
        }
    }

    fn _csv_to_hashmap (&self, path_of_file: &str) -> HashMap<String, String> { // Csv to hashmap <String, String>
        let mut new_hashmap: HashMap<String, String> = HashMap::new();
        let read_file = fs::read_to_string(path_of_file).unwrap();
        for lines in read_file.clone().split_terminator("\n") {
            let pair = lines.split(",").collect:: <Vec<&str>> ();
            new_hashmap.insert(pair[0].to_string(), pair[1].to_string());
        };
        new_hashmap
    }

}

pub async fn init(db: Data<Mongo>) -> Normalizer {

    let mut cursor = db.get_active_normalizers_cursor().await.unwrap();

    let mut active_normalizers: HashMap<String, Helper> = HashMap::new();
    let mut regex_objects: Vec<(Regex, String)> = Vec::new();
    let mut taxonomy_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut type_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut log_type: HashMap<String, String> = HashMap::new();
    
    while let Some(normalizer) = cursor
        .try_next()
        .await
        .ok()
        .expect("Error blah.. blah.. blah..")
    {
        let id = normalizer.id.unwrap().to_string();
        regex_objects.push((Regex::new(&normalizer.regex).unwrap(), id.clone()));
        type_map.insert(id.clone(), normalizer.type_mapping);
        taxonomy_map.insert(id.clone(), normalizer.taxonomy_mapping);
        active_normalizers.insert(id.clone(), Helper {
            _norm_id: normalizer.norm_id,
            _device_category: normalizer.category,
            _internal_regex_objects: normalizer.internal_regex.into_iter().map(|x|Regex::new(&x).unwrap()).collect:: <Vec<Regex>>()
        });
        log_type.insert(id.clone(), normalizer.log_type);
    };
    let normalizer = Normalizer {
        active_normalizers,
        regex_objects,
        taxonomy_map,
        type_map,
        log_type,
        regex: Regex::new("\\{.*\\}").unwrap()
    };
    return normalizer
}
