use actix_web::dev::Payload;
use actix_web::{Error, error, FromRequest, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use futures::future::{Ready, ok, err};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwToken {
    pub username: String,
    pub exp: usize,
    pub email: String,
    pub is_admin: i32,
    pub id: i32,
}
#[derive(Serialize, Deserialize, FromRow)]
pub struct AdminEmail {
    pub id: i32,
    pub admin: i32,
    pub username: String
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
        let todo: AdminEmail = sqlx::query_as("SELECT id, admin, username from users where email=$1")
            .bind(&email)
            .fetch_one(pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e.to_string())).unwrap();
        let timestamp = Utc::now().checked_add_signed(chrono::Duration::minutes(360)).expect("valid Timestamp").timestamp();
        return JwToken {username: todo.username, exp: timestamp as usize, email, is_admin: todo.admin, id: todo.id};
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
        // let res = req.cookies().unwrap();
        // println!("{:?}", res);
        // let hed = req.headers();
        // println!("{:?}", hed);
        match req.cookie("jwt") {
            Some(data) => {
                // println!("{}", data);
                let raw_token = data.value().to_string();
                // println!("{}", raw_token);

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