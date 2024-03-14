mod view;

use actix_web::web;
use crate::admin::view::{approve, view};

pub fn admin_config (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("/view", web::get().to(view))
            .route("/approve", web::post().to(approve))
    );
}