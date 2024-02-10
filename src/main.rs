use std::fs;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{error, web::{self, Json, ServiceConfig}, HttpResponse, get, post};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{Error, Executor, FromRow, PgPool, Row};
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
#[derive(Deserialize, Debug)]
struct Query {
    aptamer: String,
    target: String,
    apt_type: String
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
                .service(fetch_partial)
                .service(fetch_single)
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
        Ok(s) => { HttpResponse::Ok().body(s) }
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[get("/{field}")]
async fn fetch_partial(pool: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let field: &str = &path.into_inner();
    let query = sqlx::query("SELECT $1 from aptamers").bind(&field).fetch_all(&pool.pool).await;
    match query {
        Ok(q) => {
            let rows = q
                .iter()
                .map(|r| r.get::<&str, _>(field))
                .collect::<Vec<&str>>();
            let json = serde_json::to_string(&rows).unwrap();
            HttpResponse::Ok().body(json)
        }
        Err(_) => {
            HttpResponse::NotFound().body("Invalid field".to_string())
        }
    }
}

async fn insert(pool: web::Data<AppState>, data: Json<Entry>) -> HttpResponse {
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

#[post("/")]
pub async fn fetch_single(query: Json<Query>, pool: web::Data<AppState>) -> HttpResponse{
    let mut sql = "SELECT * FROM aptamers WHERE".to_string();
    let mut idx = 1;
    println!("{:?}", &query);
    if &query.aptamer != "" {
        sql.push_str(&format!(" aptamer = '{}' AND", &query.aptamer));
        idx += 1;
    }
    if &query.target != "" {
        sql.push_str(&format!(" target = '{}' AND", &query.target));
        idx += 1;
    }
    if &query.apt_type != "" {
        sql.push_str(&format!(" apt_type = '{}' AND", &query.apt_type));
        idx += 1;
    }


    // Remove the trailing " AND" if it exists
    if sql.ends_with(" AND") {
        sql.pop(); // Remove last character ('D')
        sql.pop(); // Remove last character ('N')
        sql.pop(); // Remove last character ('A')
        sql.pop(); // Remove last character (' ')
        sql.push(';');
    }
    println!("{}", &sql);

    let todo: Result<Vec<Entry>, Error> = sqlx::query_as(&sql)
        .fetch_all(&pool.pool)
        .await;
    match todo {
        Ok(s) => {
            let json_string = serde_json::to_string(&s);
            match json_string {
                Ok(st) => {
                    HttpResponse::Ok().body(st)
                }
                Err(_) => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}