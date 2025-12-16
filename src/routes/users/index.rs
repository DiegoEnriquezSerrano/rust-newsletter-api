use crate::models::{PublicProfileListItem, UserProfile};
use crate::utils::e500;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, get, web};
use anyhow::Context;
use sqlx::PgPool;

#[get("/users")]
#[tracing::instrument(name = "Retrieving list of users", skip(pool))]
pub async fn get(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let user_profiles: Vec<PublicProfileListItem> = UserProfile::get_public_profiles(&pool)
        .await
        .context("Failed to fetch user profiles.")
        .map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(user_profiles))
}
