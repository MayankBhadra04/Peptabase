use std::fs;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{error, web::{self, Json, ServiceConfig}, HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{Executor, FromRow, PgPool};
use serde_derive::{Deserialize, Serialize};
use actix_web::dev::Service;

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

    let table  = "CREATE TABLE aptamers (
   aptamer varchar(39) DEFAULT NULL,
   target varchar(67) DEFAULT NULL,
   apt_type varchar(10) DEFAULT NULL,
   length varchar(3) DEFAULT NULL,
   sequence varchar(42) DEFAULT NULL,
   effect varchar(285) DEFAULT NULL,
   reference varchar(415) DEFAULT NULL
);";
    let resp1 = sqlx::query(&table).execute(&pool).await;
    match resp1{
        Ok(_) => {
            println!("Created table");
        }
        Err(_) => {
            println!("Error in creating table");
        }
    }
    let query = fs::read_to_string("schema.sql").unwrap();
    let resp = sqlx::query(&query).execute(&pool).await;
    match resp{
        Ok(_) => {
            println!("Created database");
        }
        Err(_) => {
            println!("Error in database");
        }
    }

    let state = web::Data::new(AppState { pool });

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
                .route("/fetch", web::get().to(fetch))
                .route("/insert", web::post().to(insert))
                .app_data(state),
        );
    };

    Ok(config.into())
}

pub async fn fetch(pool: web::Data<AppState>) -> HttpResponse {
    let todo: Vec<Entry> = sqlx::query_as("SELECT * FROM aptamers")
        .fetch_all(&pool.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string())).unwrap();

    let json_string = serde_json::to_string(&todo);
    match json_string {
        Ok(s) => {HttpResponse::Ok().body(s)}
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn insert(pool: web::Data<AppState>, data: Json<Entry>) -> HttpResponse {
    match sqlx::query("INSERT INTO aptamers (aptamer, target, apt_type, length, sequence, effect, reference) VALUES ($1, $2, $3, $4, $5, $6, $7);")
        .bind(&data.aptamer)
        .bind(&data.target)
        .bind(&data.apt_type)
        .bind(&data.length)
        .bind(&data.sequence)
        .bind(&data.effect)
        .bind(&data.reference)
        .execute(&pool.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string())) {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(_) => {
            HttpResponse::Conflict().finish()
        }
    }
}