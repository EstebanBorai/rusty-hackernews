use actix_web::http::StatusCode;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::User;
use crate::environment::Environment;
use crate::error::{Error, Result};

#[derive(Deserialize, Serialize)]
pub struct RegisterDto {
    first_name: String,
    surname: Option<String>,
    email: String,
    username: String,
    password: String,
    avatar_url: Option<String>,
}

#[derive(Debug, FromRow)]
struct UsersRow {
    id: Uuid,
    first_name: String,
    surname: Option<String>,
    email: String,
    username: String,
    password_hash: String,
    avatar_url: Option<String>,
    token: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    user: User,
}

impl From<UsersRow> for User {
    fn from(row: UsersRow) -> Self {
        User {
            id: row.id,
            first_name: row.first_name,
            surname: row.surname,
            email: row.email,
            username: row.username,
            password_hash: row.password_hash,
            avatar_url: row.avatar_url,
            token: row.token,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

pub struct UserService {
    database_pool: Arc<PgPool>,
    environment: Arc<Environment>,
}

impl UserService {
    pub fn new(database_pool: Arc<PgPool>, environment: Arc<Environment>) -> Self {
        UserService {
            database_pool,
            environment,
        }
    }

    pub async fn register(&self, register_dto: RegisterDto) -> Result<User> {
        let password_hash = User::hash_password(&register_dto.password)?;

        if !User::is_valid_email(&register_dto.email) {
            return Err(Error::new(
                StatusCode::BAD_REQUEST,
                "The provided email is not valid",
                None,
            ));
        }

        let row: UsersRow = sqlx::query_as(
            r#"
        INSERT INTO users (
            first_name,
            surname,
            email,
            username,
            password_hash,
            avatar_url
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6
        ) RETURNING *
        "#,
        )
        .bind(&register_dto.first_name)
        .bind(&register_dto.surname)
        .bind(&register_dto.email)
        .bind(&register_dto.username)
        .bind(&password_hash)
        .bind(&register_dto.avatar_url)
        .fetch_one(&*self.database_pool)
        .await?;

        Ok(User::from(row))
    }

    pub async fn validate(&self, email: &str, plain_password: &str) -> Result<User> {
        match sqlx::query_as::<Postgres, UsersRow>("SELECT * FROM users WHERE users.email = $1")
            .bind(email)
            .fetch_one(&*self.database_pool)
            .await
        {
            Ok(row) => {
                if User::verify_password(plain_password, &row.password_hash)? {
                    return Ok(User::from(row));
                }

                Err(Error::new(
                    StatusCode::UNAUTHORIZED,
                    "Invalid email/password combination",
                    None,
                ))
            }
            Err(err) => {
                if matches!(err, sqlx::Error::RowNotFound) {
                    return Err(Error::new(
                        StatusCode::BAD_REQUEST,
                        "The provided email is not valid",
                        None,
                    ));
                }

                Err(Error::from(err))
            }
        }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<User> {
        match sqlx::query_as::<Postgres, UsersRow>("SELECT * FROM users WHERE users.email = $1")
            .bind(email)
            .fetch_one(&*self.database_pool)
            .await
        {
            Ok(row) => Ok(User::from(row)),
            Err(err) => {
                if matches!(err, sqlx::Error::RowNotFound) {
                    return Err(Error::new(
                        StatusCode::BAD_REQUEST,
                        &format!("User with email \"{}\" not found", email),
                        None,
                    ));
                }

                Err(Error::from(err))
            }
        }
    }

    pub async fn sign_token(&self, user: &User) -> Result<(String, DateTime<Utc>)> {
        let header = Header::default();
        let exp = Utc::now() + Duration::hours(12);
        let claims = Claims {
            exp: exp.timestamp() as usize,
            user: user.clone(),
        };
        let encoding_key = self.environment.json_web_token_encoding_key.as_bytes();
        let encoding_key = EncodingKey::from_secret(encoding_key);
        let token = encode(&header, &claims, &encoding_key).map_err(Error::from)?;

        match self.store_user_token(&user, &token).await {
            Ok(_) => Ok((token, exp)),
            Err(error) => Err(error),
        }
    }

    pub fn decode_token(&self, token: &str) -> Result<Claims> {
        let decoding_key = self.environment.json_web_token_encoding_key.as_bytes();
        let decoding_key = DecodingKey::from_secret(decoding_key);
        let token_message =
            decode::<Claims>(&token, &decoding_key, &Validation::new(Algorithm::HS256))?;

        Ok(token_message.claims)
    }

    async fn store_user_token(&self, user: &User, token: &str) -> Result<()> {
        // The "RETURNING *" statement at the end is added because otherwise
        // the `query_as` call returns a `RowNotFound` error instance
        sqlx::query_as::<Postgres, UsersRow>(
            "UPDATE users SET token = $1 WHERE users.id = $2 RETURNING *",
        )
        .bind(&token)
        .bind(&user.id)
        .fetch_one(&*self.database_pool)
        .await?;

        Ok(())
    }
}
