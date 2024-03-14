mod view;
mod r#static;
mod admin;
mod auth;

use actix_files;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{error, web::{self, Json, ServiceConfig}, HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{Executor, FromRow, PgPool, Row};
use serde_derive::{Deserialize, Serialize};
use actix_web::dev::Service;
use actix_web::web::Data;
use crate::admin::admin_config;
use crate::auth::auth_config;
use crate::r#static::static_config;
use crate::view::view_config;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}
#[derive(FromRow, Serialize, Deserialize)]
struct Entry {
    aptamer: String,
    target: String,
    apt_type: String,
    length: String,
    sequence: String,
    effect: String,
    reference: String
}


#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    sqlx::migrate!("./migrations").run(&pool).await.expect("Error migrating database");
    let state: Data<AppState> = Data::new(AppState { pool });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/")
                .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header().supports_credentials())
                .wrap_fn(|req, srv| {
                    println!("{} {}", req.method(), req.uri());
                    let future = srv.call(req);
                    async {
                        let result = future.await?;
                        Ok(result)
                    }
                })
                .wrap(Logger::default())
                .configure(view_config)
                .configure(static_config)
                .configure(admin_config)
                .configure(auth_config)
                .app_data(state),

        );
    };
    Ok(config.into())
}

