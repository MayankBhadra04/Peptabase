use actix_web::{delete, HttpResponse, web};
use serde_derive::{Deserialize, Serialize};
use sqlx::{Error, FromRow};
use crate::{AppState, Entry};
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
#[derive(Serialize, Deserialize, Debug)]
pub struct ApprovalList {
    id: i32,
    decision: bool
}

pub async fn view(pool: web::Data<AppState>, _: JwToken) -> HttpResponse {
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

pub async fn approve (pool: web::Data<AppState>, payload: web::Json<ApprovalList>, jwt: JwToken) -> HttpResponse {
    if jwt.is_admin == true {
        println!("{:?}", payload);
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
            match sqlx::query("DELETE from pending_list where id=$1")
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

pub async fn delete_one (jwt: JwToken, path: web::Path<i32>, pool: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();
    if jwt.is_admin == true {
        match sqlx::query("DELETE from aptamers where id=$1")
            .bind(&id)
            .execute(&pool.pool)
            .await {
            Ok(_) => {
                HttpResponse::Ok().finish()
            }
            Err(_) => {
                HttpResponse::BadRequest().finish()
            }
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

pub async fn edit_row(jwt: JwToken, pool: web::Data<AppState>, payload: web::Json<Entry>) -> HttpResponse {
    if jwt.is_admin == true {
        let todo = sqlx::query("UPDATE aptamers set aptamer = $1, target = $2, apt_type = $3, length = $4, sequence = $5, effect = $6, reference = $7 where id = $8")
            .bind(&payload.aptamer)
            .bind(&payload.target)
            .bind(&payload.apt_type)
            .bind(&payload.length)
            .bind(&payload.sequence)
            .bind(&payload.effect)
            .bind(&payload.reference)
            .bind(&payload.id)
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
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
#[derive(FromRow, Serialize, Deserialize)]
struct GetComment {
    id: i32,
    email: String,
    comment: String
}
pub async fn view_comment (jwt: JwToken, pool: web::Data<AppState>) -> HttpResponse {
    println!("{:?}", jwt);
    if jwt.is_admin {
        let result: Result<Vec<GetComment>, Error> = sqlx::query_as("SELECT * from comment")
            .fetch_all(&pool.pool)
            .await;
        match result {
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
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
#[delete("/delete/{id}")]
pub async fn delete_comment (jwt: JwToken, path: web::Path<i32>, pool: web::Data<AppState>) -> HttpResponse {
    if jwt.is_admin {
        let id = path.into_inner();
        match sqlx::query("DELETE from comment where id=$1")
            .bind(&id)
            .execute(&pool.pool)
            .await {
            Ok(_) => {
                HttpResponse::Ok().finish()
            }
            Err(_) => {
                HttpResponse::BadRequest().finish()
            }
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}