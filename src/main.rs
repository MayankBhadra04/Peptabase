mod view;
// mod r#static;
mod admin;
mod auth;

use std::fs::File;
use std::io::Read;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{ HttpResponse,  web::{self, ServiceConfig}};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{FromRow, PgPool};
use serde_derive::{Deserialize, Serialize};
use actix_web::dev::Service;
use actix_web::web::Data;
use crate::admin::admin_config;
use crate::auth::auth_config;
use crate::view::view_config;
use include_dir::include_dir;
use std::str::FromStr;
use shuttle_shared_db::Postgres;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}
#[derive(FromRow, Serialize, Deserialize)]
struct Entry {
    id: i32,
    aptamer: String,
    target: String,
    apt_type: String,
    length: String,
    sequence: String,
    effect: String,
    reference: String,
    structure: String
}

async fn execute_queries_from_file(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Include the SQL file at compile time
    let contents = include_str!("../migrations/0001_aptamer.sql");

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

static STATIC_FILES: include_dir::Dir = include_dir!("./src/static");

async fn serve_static_files(path: web::Path<String>) -> HttpResponse {
    // Get the file name requested by the user (e.g., /index.html or /amit_sir.jpg)
    let file_path = format!("{}", path);

    // Attempt to find the file in the included directory
    if let Some(file) = STATIC_FILES.get_file(file_path.as_str()) {
        // Try to determine the content type based on file extension
        let content_type = match file_path.rsplit('.').next() {
            Some("html") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("png") => "image/png",
            Some("gif") => "image/gif",
            _ => "application/octet-stream", // Default type for unknown files
        };

        // If the content is text-based (HTML, CSS, JS)
        if let Some(content) = file.contents_utf8() {
            HttpResponse::Ok()
                .content_type(content_type)
                .body(content)
        } else {
            // If the content is binary (like images)
            HttpResponse::Ok()
                .content_type(content_type)
                .body(file.contents())
        }
    } else {
        // Return a 404 if the file doesn't exist
        println!("File not found {}", file_path);
        HttpResponse::NotFound().body("File not found")
    }
}

#[shuttle_runtime::main]
async fn actix_web(
    #[Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    match execute_queries_from_file(&pool).await {
        Ok(_) => {
            println!("Database migration successful");
        }
        Err(e) => {
            println!("{}", e.to_string());
        }
    }
    let state: Data<AppState> = Data::new(AppState { pool });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/v1")
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
        cfg.route("/", web::get().to(index));
        cfg.route("/{filename:.*}", web::get().to(serve_static_files));
        cfg.route("/static/{filename:.*}", web::get().to(serve_static_files));
    };
    println!("All set!");
    Ok(config.into())
}

async fn index() -> HttpResponse {
    // Load the index.html file and return it as the response
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("static/index.html"))
}
