mod view;
// mod r#static;
mod admin;
mod auth;

use std::fs::File;
use std::io::Read;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web::{self, ServiceConfig}};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{FromRow, PgPool};
use serde_derive::{Deserialize, Serialize};
use actix_web::dev::Service;
use actix_web::web::Data;
use crate::admin::admin_config;
use crate::auth::auth_config;
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

async fn execute_queries_from_file(pool: &PgPool, filename: &str) -> Result<(), sqlx::Error> {
    // Read SQL file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Split queries by delimiter (;)
    let queries: Vec<&str> = contents.split(';').collect();

    // Execute each query
    for query in queries {
        let trimmed_query = query.trim();
        if !trimmed_query.is_empty() {
            sqlx::query(trimmed_query).execute(pool).await?;
        }
    }

    Ok(())
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    execute_queries_from_file(&pool, "./migrations/0001_aptamer.sql").await.unwrap();
    println!("Database migration successful");
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
                // .configure(static_config)
                .configure(admin_config)
                .configure(auth_config)
                .app_data(state),
        );
    };
    Ok(config.into())
}

