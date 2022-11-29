use std::time;
use crate::cn::{preprocessor::Preprocessor, log_normalizer::Normalizer, log_parser::Parser};
use actix_web::{
    post,
    HttpResponse, web::Data
};
use json::{object, stringify};

#[post("/normalize")]
pub  async fn normalize(matcher: Data<Preprocessor> ,raw_log: String) -> HttpResponse {
    let mut _normalized_log = object!{};
    let start_time = time::Instant::now();
    for _i in 0..200000 {
        let (start, end, taxonomy_map, type_map) = matcher.match_regex(&raw_log).unwrap();
        _normalized_log = Normalizer {
            parsed_log: Parser{
                raw_log: json::parse(&raw_log[start..end]).unwrap(),
            }.build_stru_log(),
            taxonomy_mapping: taxonomy_map,
            type_mapping: type_map
        }.normalize();
    }
    println!("{}", start_time.elapsed().as_millis());
    HttpResponse::Ok().content_type("application/json").body(stringify(_normalized_log))
}