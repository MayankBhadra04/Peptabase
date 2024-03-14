use bcrypt::{DEFAULT_COST, hash};
use serde::Deserialize;
use actix_web::{web, HttpResponse, Responder, error};
use std::error::Error;
use sqlx::{PgPool};
use crate::AppState;

#[derive(Clone, Deserialize)]
pub struct NewUser {
    pub password: String,
    pub email: String
}

impl NewUser {
    pub fn new (password: String, email: String) -> NewUser {
        let hashed_password: String = hash (password.as_str(), DEFAULT_COST).unwrap();
        return NewUser {
            password: hashed_password,
            email
        }
    }
}
pub async fn insert( st: NewUser, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let todo = sqlx::query("INSERT INTO users (password, email, admin) VALUES ($1, $2, false)")
        .bind(&st.password)
        .bind(&st.email)
        .execute(pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()));
    match todo {
        Ok(_) => {
            println!("OK");
            Ok(())
        }
        Err(e) => {
            println!("Err");
            println!("{e}");
            Err(Box::try_from(e).unwrap())
        }
    }
}
pub async fn register (new_user: web::Json<NewUser>, state: web::Data<AppState>) -> impl Responder {
    let new_user = NewUser::new(
        new_user.password.clone(),
        new_user.email.clone()
    );
    println!("{}", new_user.password);
    match insert(new_user, &state.pool).await {
        Ok(_) => {
            println!("Created");
            HttpResponse::Created()
        },
        Err(_) => {
            println!("Conflict");
            HttpResponse::Conflict()
        }
    }
}