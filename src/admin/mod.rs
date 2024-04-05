mod view;

use actix_web::web;
use crate::admin::view::{approve, delete_comment, delete_one, edit_row, insert_admin, view, view_comment};

pub fn admin_config (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("/view", web::get().to(view))
            .route("/approve", web::post().to(approve))
            .route("/delete/{id}", web::delete().to(delete_one))
            .route("/edit", web::post().to(edit_row))
            .route("/comment", web::get().to(view_comment))
            .route("/insert", web::post().to(insert_admin))
            .service(delete_comment)
    );
}
