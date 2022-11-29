use std::sync::Mutex;

use crate::cn::normalizer_instance::Normalizer;
use actix_web::{
    post,
    HttpResponse, web::Data
};
use json::stringify;

#[post("/normalize")]
pub  async fn normalize(normalizer: Data<Mutex<Normalizer>>, raw_logs: String) -> HttpResponse {
    let mut _normalized_log = None;
    for _ in 0..10000 {
        let mut raw_logs = raw_logs.lines();
        while let Some(raw_log) = raw_logs
            .next()
            {
                let normalizer = normalizer.lock().unwrap();
                _normalized_log = normalizer.normalize_log(&raw_log);
                if _normalized_log.is_none() {
                    return HttpResponse::Ok().body(raw_log.to_owned());
                };
            }
        }
    HttpResponse::Ok().content_type("application/json").body(stringify(_normalized_log))
}