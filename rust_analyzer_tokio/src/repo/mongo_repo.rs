extern crate dotenv;
use dotenv::dotenv;
use futures::TryStreamExt;
use std::{env, str::FromStr};

use mongodb::{
    bson::{extjson::de::Error, doc, oid::ObjectId},
    results::{InsertOneResult, UpdateResult},
    Client,
    Collection, Cursor
};

use crate::models::cn_model::Normalizer;

pub struct Mongo {
    col: Collection<Normalizer>
}

impl Mongo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(err) => format!("Error loading env variable, {}", err)
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("normalizers");
        let col: Collection<Normalizer> = db.collection("Normalizer");
        Mongo {col}
    }

    pub async fn _create_normalizer (&self, new_normalizer: Normalizer) -> Result<InsertOneResult, Error> {
        let new_cn = Normalizer {
            id: new_normalizer.id,
            category: new_normalizer.category,
            regex: new_normalizer.regex,
            internal_regex: new_normalizer.internal_regex,
            norm_id: new_normalizer.norm_id,
            taxonomy_mapping: new_normalizer.taxonomy_mapping,
            type_mapping: new_normalizer.type_mapping,
            active: new_normalizer.active,
            log_type: new_normalizer.log_type
        };
        let normalizer = self
            .col
            .insert_one(new_cn, None)
            .await
            .ok()
            .expect("Error creating normalizer");
        Ok(normalizer)
    }

    pub async fn _get_normalizer (&self, id: &String) -> Result<Normalizer, Error> {
        let _id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": _id}; 
        let normalizer = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Normalizer not found");
        Ok(normalizer.unwrap())
    }

    pub async fn _update_status (&self, id: &String, status: bool) -> Result<UpdateResult, Error> {
        let filter = doc! {
            "_id": ObjectId::from_str(id).unwrap(),
        };
        let update = doc! {
            "$set": {
                "active": status
            }
        };
        let update_result = self
            .col
            .update_one(filter, update, None)
            .await
            .ok()
            .expect("unable to update the active status");
        Ok(update_result)
    }

    pub async fn _update_normalizer (&self, updated_normalizer: Normalizer) -> Result<UpdateResult, Error> {
        let filter = doc! {
            "_id": updated_normalizer.id
        };
        let update = doc! {
            "$set": {
                "category": updated_normalizer.category,
                "regex": updated_normalizer.regex,
                "internal_regex": updated_normalizer.internal_regex,
                "norm_id": updated_normalizer.norm_id,
                "taxonomy_mapping": bson::to_bson(&updated_normalizer.taxonomy_mapping).unwrap(),
                "type_mapping": bson::to_bson(&updated_normalizer.type_mapping).unwrap(),
                "active": updated_normalizer.active,
                }
            };
        let updated_result = self
            .col
            .update_one(filter, update, None)
            .await
            .ok()
            .expect("Error updatiing normalizer");
        Ok(updated_result)
    }

    pub async fn _get_active_normalizers (&self) -> Result<Vec<Normalizer>, Error>  {
        let mut cursor = self.col.find(doc!{
            "active": true
        }, None)
        .await
        .ok()
        .expect("Unable to perform request.");
        let mut normalizers: Vec<Normalizer> = Vec::new();
        while let Some(normalizer) = cursor
            .try_next()
            .await
            .ok()
            .expect("Error blah.. blah.. blah..")
        {
            normalizers.push(normalizer)
        }
        Ok(normalizers)
    }
    
    pub async fn _get_all_normalizers (&self) -> Result<Vec<Normalizer>, Error> {
        let mut cursor = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Unable to fetch records");
        let mut normalizers: Vec<Normalizer> = Vec::new();
        while let Some(normalizer) = cursor
            .try_next()
            .await
            .ok()
            .expect("Error blah.. blah.. blah..")
        {
            normalizers.push(normalizer)
        }
        Ok(normalizers)
    }

    pub async fn get_active_normalizers_cursor (&self) -> Result<Cursor<Normalizer>, Error>  {
        let cursor = self
            .col
            .find(doc!{ "active": true}, None)
            .await
            .ok()
            .expect("Unable to perform request.");   
        Ok(cursor)
    }
}