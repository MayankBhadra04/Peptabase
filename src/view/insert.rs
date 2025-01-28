use actix_web::{error, HttpResponse, post, web};
use actix_web::web::Json;
use serde_derive::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};
use crate::{AppState, Entry};

#[derive(FromRow, Serialize, Deserialize)]
struct EntryApproval {
    email: String,
    aptamer: String,
    target: String,
    apt_type: String,
    length: String,
    sequence: String,
    effect: String,
    reference: String,
    structure: String
}
#[derive(Deserialize, FromRow)]
pub struct Comment {
    email: String,
    comment: String
}
pub async fn insert(pool: &PgPool, id: i32) -> HttpResponse {

    let todo: Result<Entry, Error>  = sqlx::query_as("SELECT aptamer, target, apt_type, length, sequence, effect, reference, structure from pending_list where id=$1")
        .bind(&id)
        .fetch_one(pool)
        .await;
    let data = match todo {
        Ok(s) => {
            s
        }
        Err(_) => {
            return HttpResponse::BadRequest().finish();
        }
    };

    match sqlx::query("INSERT INTO aptamers (aptamer, target, apt_type, length, sequence, effect, reference, structure) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);")
        .bind(&data.aptamer)
        .bind(&data.target)
        .bind(&data.apt_type)
        .bind(&data.length)
        .bind(&data.sequence)
        .bind(&data.effect)
        .bind(&data.reference)
        .bind(&data.structure)
        .execute(pool)
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

#[post("/insert")]
pub async fn insert_approval(pool: web::Data<AppState>, data: Json<EntryApproval>) -> HttpResponse{
    match sqlx::query("INSERT INTO pending_list (email, aptamer, target, apt_type, length, sequence, effect, reference, structure, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10);")
        .bind(&data.email)
        .bind(&data.aptamer)
        .bind(&data.target)
        .bind(&data.apt_type)
        .bind(&data.length)
        .bind(&data.sequence)
        .bind(&data.effect)
        .bind(&data.reference)
        .bind(&data.structure)
        .bind("Pending".to_string())
        .execute(&pool.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string())) {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            HttpResponse::BadRequest().body(e.to_string())
        }
    }


}

pub async fn insert_comment(pool: web::Data<AppState>, data: Json<Comment>) -> HttpResponse {
    let todo = sqlx::query("INSERT INTO comment (email, comment) VALUES ($1, $2)")
        .bind(&data.email)
        .bind(&data.comment)
        .execute(&pool.pool)
        .await;
    match todo {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(_) => {
            HttpResponse::BadRequest().finish()
        }
    }
}