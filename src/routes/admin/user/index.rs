use crate::authentication::UserId;
use crate::models::{UserProfile, UserProfileAPI};
use crate::utils::e404;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, get, web};
use anyhow::Context;
use sqlx::PgPool;

#[get("/user")]
#[tracing::instrument(name = "Get authenticated user profile", skip_all)]
pub async fn get(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    let user: UserProfileAPI = UserProfile::find_user_profile_api_by_user_id(*user_id, &pool)
        .await
        .context("Failed to find user.")
        .map_err(e404)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(user))
}
