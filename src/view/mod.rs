mod simple_fetch;
pub mod insert;

use actix_web::web;
use simple_fetch::{fetch_partial, fetch_single, fetch};
use crate::view::insert::insert_approval;


pub fn view_config (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/fetch", web::get().to(fetch))
            .route("/fetchsingle", web::post().to(fetch_single))
            .service(fetch_partial)
            .service(insert_approval)
    );
}