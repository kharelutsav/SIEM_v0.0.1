use std::sync::Mutex;

use crate::{models::cn_model::Normalizer, repo::mongo_repo::Mongo, cn::normalizer_instance};
use actix_web::{
    post, get,
    web::{Data, Json},
    HttpResponse, put
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    id: String,
    status: bool
}

#[post("/normalizer")]
pub async fn create_normalizer (db: Data<Mongo>, new_normalizer: Json<Normalizer>) -> HttpResponse {
    let data = Normalizer {
        id: None,
        category: new_normalizer.category.to_owned(),
        regex: new_normalizer.regex.to_owned(),
        internal_regex: new_normalizer.internal_regex.to_owned(),
        norm_id: new_normalizer.norm_id.to_owned(),
        taxonomy_mapping: new_normalizer.taxonomy_mapping.to_owned(),
        type_mapping: new_normalizer.type_mapping.to_owned(),
        active: false,
        log_type: new_normalizer.log_type.to_owned()
    };
    let normalizer_detail = db.create_normalizer(data).await;
    match normalizer_detail {
        Ok(normalizer) => HttpResponse::Ok().json(normalizer),
        Err(_) => HttpResponse::InternalServerError().body("Unable to create normaizer."),
    }
}

#[put("/update-status")]
pub async fn update_status (db: Data<Mongo>, preprocessor: Data<Mutex<normalizer_instance::Normalizer>>, req_body: Json<Status>) -> HttpResponse {
    let id = req_body.id.clone();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let action_details = db.update_status(&id, req_body.status).await;
    match action_details {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_info = db.get_normalizer(&id).await;
                return match updated_info {
                    Ok(normalizer) => {
                        {
                            let mut preprocessor = preprocessor.lock().unwrap();
                            preprocessor.update_normalizer(id, &normalizer);
                            println!("{:#?}", preprocessor);
                        }
                        HttpResponse::Ok().json(normalizer)
                    }
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string())
                }
            } else {
                return HttpResponse::NotFound().body("No normalizer found with specified ID");
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[put("/normalizer")]
pub async fn update_normalizer (db: Data<Mongo>, preprocessor: Data<Mutex<normalizer_instance::Normalizer>>, updated_normalizer: Json<Normalizer>) -> HttpResponse {
    let id = updated_normalizer.id.unwrap().to_string();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let data = Normalizer {
        id: updated_normalizer.id.to_owned(),
        category: updated_normalizer.category.to_owned(),
        regex: updated_normalizer.regex.to_owned(),
        internal_regex: updated_normalizer.internal_regex.to_owned(),
        norm_id: updated_normalizer.norm_id.to_owned(),
        taxonomy_mapping: updated_normalizer.taxonomy_mapping.to_owned(),
        type_mapping: updated_normalizer.type_mapping.to_owned(),
        active: updated_normalizer.active.to_owned(),
        log_type: updated_normalizer.log_type.to_owned()
    };
    let action_details = db.update_normalizer(data).await;
    match action_details {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_info = db.get_normalizer(&id).await;
                return match updated_info {
                    Ok(normalizer) => {
                        if normalizer.active == true {
                            let mut preprocessor = preprocessor.lock().unwrap();
                            preprocessor.update_normalizer(id, &normalizer);
                        }
                        HttpResponse::Ok().json(normalizer)
                    },
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string())
                }
            } else {
                return HttpResponse::NotFound().body("No normalizer found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[get("/all-normalizers")]
pub async fn fetch_all_normalizers (db: Data<Mongo>) -> HttpResponse {
    let normalizer_details = db.get_all_normalizers().await;
    match normalizer_details {
        Ok(normalizers) => HttpResponse::Ok().json(normalizers),
        Err(_) => HttpResponse::InternalServerError().body("Unable to fetch all records.")
    }
}

#[get("/active-normalizers")]
pub async fn fetch_active_normalizers (db: Data<Mongo>) -> HttpResponse {
    let normalizer_details = db.get_active_normalizers().await;
    match normalizer_details {
        Ok(normalizers) => HttpResponse::Ok().json(normalizers),
        Err(_) => HttpResponse::InternalServerError().body("Unable to fetch all records.")
    }
}