use std::time;
use crate::cn::{preprocessor::Preprocessor, log_normalizer::Normalizer, log_parser::Parser};
use actix_web::{
    post,
    HttpResponse, web::Data
};
use serde_json::json;

#[post("/normalize")]
pub  async fn normalize(matcher: Data<Preprocessor> ,raw_log: String) -> HttpResponse {
    let mut _normalized_log = json!({});
    let start_time = time::Instant::now();
    for _i in 0..10000 {
        let (start, end, taxonomy_map, type_map) = matcher.match_regex(&raw_log).unwrap();
        _normalized_log = Normalizer {
            parsed_log: Parser{
                raw_log: serde_json::from_str(&raw_log[start..end]).unwrap(),
            }.build_stru_log(),
            taxonomy_mapping: taxonomy_map,
            type_mapping: type_map
        }.normalize();
    }
    let stop_time = start_time.elapsed().as_millis();
    println!("{}", stop_time);
    HttpResponse::Ok().json(_normalized_log)
}