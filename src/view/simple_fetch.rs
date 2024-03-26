use actix_web::{error, get, HttpResponse, web};
use actix_web::web::Json;
use serde_derive::Deserialize;
use sqlx::{Error, Row};
use crate::{AppState, Entry};
use crate::auth::jwt::JwToken;


#[derive(Deserialize, Debug)]
pub struct Query {
    pub aptamer: String,
    pub target: String,
    pub apt_type: String,
    pub length: String,
    pub sequence: String
}

pub async fn fetch(pool: web::Data<AppState>) -> HttpResponse {
    let todo: Vec<Entry> = sqlx::query_as("SELECT * FROM aptamers ORDER BY aptamer ASC")
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
pub async fn fetch_admin(pool: web::Data<AppState>, _: JwToken) -> HttpResponse {
    let todo: Vec<Entry> = sqlx::query_as("SELECT * FROM aptamers ORDER BY aptamer ASC")
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


pub async fn fetch_single(query: Json<Query>, pool: web::Data<AppState>) -> HttpResponse{
    let mut sql = "SELECT * FROM aptamers WHERE".to_string();
    println!("{:?}", &query);
    if &query.aptamer != "" {
        sql.push_str(&format!(" aptamer = '{}' AND", &query.aptamer));
    }
    if &query.target != "" {
        sql.push_str(&format!(" target = '{}' AND", &query.target));
    }
    if &query.apt_type != "" {
        sql.push_str(&format!(" apt_type = '{}' AND", &query.apt_type));
    }
    if &query.length != "" {
        sql.push_str(&format!(" length(sequence) >= {} AND", &query.length));
    }
    if &query.sequence != "" {
        sql.push_str(&format!(" sequence = '{}' AND", &query.sequence));
    }


    // Remove the trailing " AND" if it exists
    if sql.ends_with(" AND") {
        sql.pop(); // Remove last character ('D')
        sql.pop(); // Remove last character ('N')
        sql.pop(); // Remove last character ('A')
        sql.pop(); // Remove last character (' ')
        sql.push_str(&" ORDER BY aptamer ASC".to_string());
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

#[get("/{field}")]
pub async fn fetch_partial(pool: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let mut field: &str = &path.into_inner();
    field = field.trim();
    let query = format!("SELECT DISTINCT {} from aptamers ORDER by {} ASC;", &field, &field);
    let query = sqlx::query(&query).fetch_all(&pool.pool).await;
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

#[get("/fetch/{word}")]
pub async fn fetch_keyword(pool: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let mut word: &str = &path.into_inner();
    word = word.trim();

    let query: Result<Vec<Entry>, Error> = sqlx::query_as("select * from aptamers where CONCAT(aptamer, target, apt_type, sequence, effect, reference) like '%$1%")
        .bind(word)
        .fetch_all(&pool.pool)
        .await;

    match query {
        Ok(q) => {
            let json = serde_json::to_string(&q).unwrap();
            HttpResponse::Ok().body(json)
        }
        Err(_) => {
            HttpResponse::NotFound().body("No results found".to_string())
        }
    }
}