use actix_web::web;

pub fn static_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/static")
    );
}