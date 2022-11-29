mod api;
mod cn;
mod models;
mod repo;

use std::sync::Mutex;
use actix_web::{App, HttpServer, web::Data};
use api::normalizer_api::{create_normalizer, fetch_all_normalizers, fetch_active_normalizers, update_normalizer, update_status};
use api::normalize::normalize;
use repo::mongo_repo::Mongo;
use cn::normalizer_instance;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Mongo::init().await;
    let db_data = Data::new(db);

    let _normalizer = normalizer_instance::init(db_data.clone()).await;
    let _regex_matcher = Data::new(Mutex::new(_normalizer));

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .app_data(_regex_matcher.clone())
            .service(normalize)
            .service(create_normalizer)
            .service(fetch_all_normalizers)
            .service(fetch_active_normalizers)
            .service(update_status)
            .service(update_normalizer)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}