use actix_web::dev::Payload;
use actix_web::{Error, error, FromRequest, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use futures::future::{Ready, ok, err};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{TimeDelta, Utc};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwToken {
    pub exp: usize,
    pub email: String,
    pub is_admin: bool,
}
#[derive(FromRow)]
pub struct AdminEmail {
    pub email: String,
    pub admin: bool
}
impl JwToken {
    pub fn get_key() -> String {
        let key: String = std::env::var("JWT_KEY").unwrap_or("secret".parse().unwrap());
        return key;
    }
    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwToken::get_key().as_ref());
        let token = encode(&Header::default(), &self, &key).unwrap();
        return token;
    }
    pub async fn new(email: String, pool: &PgPool) -> Self {
        let todo: AdminEmail = sqlx::query_as("SELECT admin, email from users where email=$1")
            .bind(&email)
            .fetch_one(pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e.to_string())).unwrap();
        let timestamp = Utc::now().checked_add_signed(TimeDelta::try_minutes(360).unwrap()).expect("valid Timestamp").timestamp();
        return JwToken {exp: timestamp as usize, email, is_admin: todo.admin, };
    }
    pub fn from_token(token: String) -> Result<Self, String>{
        let key = DecodingKey::from_secret(
            JwToken::get_key().as_ref()
        );
        let token_result = decode::<JwToken>(
            &token, &key, &Validation::default());
        match token_result {
            Ok(data) => {
                Ok(data.claims)
            },
            Err(error) => {
                let message = format!("{}", error);
                return Err(message);
            }
        }
    }
}
impl FromRequest for JwToken {
    type Error = Error;
    type Future = Ready<Result<JwToken, Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();
                let token_result = JwToken::from_token(raw_token);
                match token_result {
                    Ok(token) => {
                        ok(token)
                    },
                    Err(message) => {
                        if message == "ExpiredSignature".to_owned() {
                            return err(ErrorUnauthorized("Token expired"))
                        }
                        return err(ErrorUnauthorized("Token can't be decoded"))
                    }
                }
            },
            None => {
                return err (
                    ErrorUnauthorized("token not in header under key 'token'")
                );
            }
        }
    }
}