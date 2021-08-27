use actix_web::web::{Data, Json};
use actix_web::HttpResponse;

use crate::services::user::RegisterDto;
use crate::AppData;

pub async fn register(app_data: Data<AppData>, register_dto: Json<RegisterDto>) -> HttpResponse {
    match app_data
        .users_service
        .lock()
        .await
        .register(register_dto.into_inner())
        .await
    {
        Ok(user) => {
            return HttpResponse::Created().json(common::User {
                id: user.id,
                avatar_url: user.avatar_url,
                email: user.email,
                first_name: user.first_name,
                surname: user.surname,
                username: user.username,
                created_at: user.created_at,
                updated_at: user.updated_at,
            });
        }
        Err(err) => err.as_http_response(),
    }
}
