use actix_web::{HttpResponse};
use actix_web::web::{Data, Json};
use serde_derive::{Deserialize, Serialize};
use sqlx::{FromRow};
use crate::{AppState};
use crate::auth::jwt::JwToken;
use crate::view::insert::insert;

#[derive(FromRow, Serialize, Deserialize)]
pub struct PendingList {
    id: i32,
    email: String,
    aptamer: String,
    target: String,
    apt_type: String,
    length: String,
    sequence: String,
    effect: String,
    reference: String,
    status: String
}
#[derive(Serialize, Deserialize)]
pub struct ApprovalList {
    id: i32,
    decision: bool
}

pub async fn view(pool: Data<AppState>) -> HttpResponse {
    let todo: Result<Vec<PendingList>, _> = sqlx::query_as("SELECT * FROM pending_list where status='Pending' ORDER BY email ASC")
        .fetch_all(&pool.pool)
        .await;
    match todo {
        Ok(s) => {
            let str = serde_json::to_string(&s);
            match str {
                Ok(t) => {
                    HttpResponse::Ok().body(t)
                }
                Err(e) => {
                    HttpResponse::InternalServerError().body(e.to_string())
                }
            }

        }
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub async fn approve (pool: Data<AppState>, payload: Json<ApprovalList>, jwt: JwToken) -> HttpResponse {
    if jwt.is_admin == true {
        if payload.decision == true {
            return match sqlx::query("UPDATE pending_list set status = 'Approved' where id=$1")
                .bind(&payload.id)
                .execute(&pool.pool)
                .await {
                Ok(_) => {
                    insert(&pool.pool, payload.id).await
                }
                Err(_) => {
                    HttpResponse::BadRequest().finish()
                }
            };
        } else {
            match sqlx::query("DELETE * from pending_list where id=$1")
                .bind(&payload.id)
                .execute(&pool.pool)
                .await {
                Ok(_) => {
                    HttpResponse::Ok().finish()
                }
                Err(_) => {
                    HttpResponse::BadRequest().finish()
                }
            }
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}