mod view;
// mod r#static;
mod admin;
mod auth;

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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = "postgresql://darshil:9919@localhost:5432/aptabase".to_string();
    let pool = PgPool::connect(&database_url).await.expect("Failed to create pool");
    sqlx::migrate!("./migrations").run(&pool).await.expect("Error migrating database");
    println!("Database migration successful");
    let state: Data<AppState> = Data::new(AppState { pool });

    HttpServer::new(move || {
        App::new()
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
            .app_data(state.clone())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

