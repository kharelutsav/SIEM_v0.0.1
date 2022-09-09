use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Normalizer {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub category: String,
    pub regex: String,
    pub internal_regex: Vec<String>,
    pub norm_id: String,
    pub taxonomy_mapping: HashMap<String, String>,
    pub type_mapping: HashMap<String, String>,
    #[serde(default = "bool::default")]
    pub active: bool,
    pub log_type: String
}