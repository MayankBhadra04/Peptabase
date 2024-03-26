mod simple_fetch;
pub mod insert;

use actix_web::web;
use simple_fetch::{fetch_partial, fetch_single, fetch};
use crate::view::insert::{insert_approval, insert_comment};
use crate::view::simple_fetch::{fetch_admin, fetch_keyword};


pub fn view_config (cfg: &mut web::ServiceConfig) {
    cfg.route("/fetch", web::get().to(fetch))
        .route("/fetchsingle", web::post().to(fetch_single))
        .service(fetch_partial)
        .service(insert_approval)
        .service(fetch_keyword)
        .route("/comment", web::post().to(insert_comment))
        .route("/fetchadmin", web::get().to(fetch_admin));
}