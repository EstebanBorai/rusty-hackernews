use actix_web::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::User;
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
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

pub struct UserService {
    database_pool: Arc<PgPool>,
}

impl UserService {
    pub fn new(database_pool: Arc<PgPool>) -> Self {
        UserService { database_pool }
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

                if let Ok(_) = self.find_by_email(email).await {
                    return Err(Error::new(
                        StatusCode::BAD_REQUEST,
                        "The email is already taken",
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
}
