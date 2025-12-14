use crate::authentication::UserId;
use crate::models::{UserProfile, UserProfileAPI};
use crate::utils::{e400, e404, e500};
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, get, put, web};
use anyhow::Context;
use serde::Deserialize;
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

#[derive(Deserialize)]
struct UserProfileParams {
    pub bio: String,
    pub description: String,
    pub display_name: String,
}

#[put("/user")]
#[tracing::instrument(
  name = "Updating user profile",
  skip_all,
  fields(user_id=%*user_id)
)]
pub async fn put(
    params: web::Json<UserProfileParams>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    UserProfile {
        bio: params.0.bio,
        description: params.0.description,
        display_name: params.0.display_name,
        user_id: *user_id.into_inner(),
    }
    .validate()
    .map_err(e400)?
    .update(&pool)
    .await
    .context("Failed to update user profile.")
    .map_err(e500)?;

    Ok(HttpResponse::Ok().finish())
}
