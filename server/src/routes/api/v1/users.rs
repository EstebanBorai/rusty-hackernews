use actix_web::http::header::AUTHORIZATION;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, HttpResponse};
use http_auth_basic::Credentials;

use crate::error::Error;
use crate::services::user::RegisterDto;
use crate::AppData;

impl From<crate::domain::User> for common::User {
    fn from(user: crate::domain::User) -> Self {
        common::User {
            id: user.id,
            avatar_url: user.avatar_url,
            email: user.email,
            first_name: user.first_name,
            surname: user.surname,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

pub async fn register(app_data: Data<AppData>, register_dto: Json<RegisterDto>) -> HttpResponse {
    match app_data
        .users_service
        .lock()
        .await
        .register(register_dto.into_inner())
        .await
    {
        Ok(user) => HttpResponse::Created().json(common::User::from(user)),
        Err(err) => err.as_http_response(),
    }
}

pub async fn login(app_data: Data<AppData>, req: HttpRequest) -> HttpResponse {
    let headers = req.headers();

    if let Some(auth_header) = headers.get(AUTHORIZATION) {
        let auth_header = match auth_header.to_str() {
            Ok(auth_header) => auth_header,
            Err(err) => {
                return Error::new(StatusCode::BAD_REQUEST, &err.to_string(), None)
                    .as_http_response()
            }
        };

        let credentials = match Credentials::from_header(String::from(auth_header)) {
            Ok(credentials) => credentials,
            Err(err) => {
                return Error::new(StatusCode::BAD_REQUEST, &err.to_string(), None)
                    .as_http_response()
            }
        };

        let user = match app_data
            .users_service
            .lock()
            .await
            .validate(&credentials.user_id, &credentials.password)
            .await
        {
            Ok(user) => user,
            Err(err) => return err.as_http_response(),
        };

        return HttpResponse::Ok().json(common::User::from(user));
    }

    Error::new(
        StatusCode::BAD_REQUEST,
        "Missing \"Authorization\" HTTP header",
        None,
    )
    .as_http_response()
}
