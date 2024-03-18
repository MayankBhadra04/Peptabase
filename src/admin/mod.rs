mod view;

use actix_web::web;
use crate::admin::view::{approve, delete_one, edit_row, view};

pub fn admin_config (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("/view", web::get().to(view))
            .route("/approve", web::post().to(approve))
            .route("/delete", web::delete().to(delete_one))
            .route("/edit", web::post().to(edit_row))
    );
}
